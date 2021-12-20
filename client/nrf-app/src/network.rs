use crate::bsp;

use defmt::{debug, trace};
use embassy::{
    time::{self, Duration},
    traits::uart::{Read, Write},
};
use embassy_nrf::{
    gpio::NoPin,
    uarte::{self, Uarte},
};
use heapless::String;
use network_protocol::{Message, MAX_DATAGRAM_SIZE};

// Illustrates sending a message out over the Uarte
// and using Postcard to serialise/deserialise it.
// Postcard is particularly concise and intuitive in
// its approach to serialising structs.

#[embassy::task(pool_size = 1)]
pub async fn main_task(p: bsp::NetworkPeripherals) {
    let uarte_config = uarte::Config::default();
    let mut uarte = Uarte::new(
        p.uarte,
        p.uarte_interrupt,
        p.uarte_rx_pin,
        p.uarte_tx_pin,
        NoPin,
        NoPin,
        uarte_config,
    );

    debug!("Network initialised");

    let mut buf = [0_u8; MAX_DATAGRAM_SIZE];
    loop {
        let message = Message(String::from("ping"));
        debug!("Sending {:?}", message);
        if postcard::to_slice(&message, &mut buf).is_ok() {
            // We are deliberately sending all bytes in the buffer so
            // we always know how many bytes to read. There are other
            // ways of doing this though.
            trace!("Sending {}", buf);
            let _ = uarte.write(&buf).await;

            // Now we receive the server's response - again, the
            // entire buffer requires filling.
            debug!("Receiving");
            if time::with_timeout(Duration::from_millis(5000), uarte.read(&mut buf))
                .await
                .is_ok()
            {
                trace!("Received {}", buf);
                if let Ok(message) = postcard::from_bytes::<Message>(&buf) {
                    debug!("Received {:?}", message);
                }
            }
        }

        time::block_for(Duration::from_millis(1000));
    }
}
