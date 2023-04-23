use crate::bsp;

use defmt::{debug, trace};
#[cfg(feature = "_nrf")]
use embassy_nrf::uarte::{self, Uarte};
#[cfg(feature = "_stm32")]
use embassy_stm32::usart::{self, Uart};
use heapless::String;
use network_protocol::{Message, MAX_DATAGRAM_SIZE};

// Illustrates receiving a message over the Uarte
// and using Postcard to serialise/deserialise it.
// Postcard is particularly concise and intuitive in
// its approach to serialising structs.

#[embassy_executor::task(pool_size = 1)]
pub async fn main_task(p: bsp::NetworkPeripherals) {
    let mut uart = init_peripherals(p);

    debug!("Network initialised");

    let mut buf = [0_u8; MAX_DATAGRAM_SIZE];
    loop {
        debug!("Receiving");
        if uart.read(&mut buf).await.is_ok() {
            trace!("Received {}", buf);
            if let Ok(message) = postcard::from_bytes::<Message>(&buf) {
                debug!("Received {:?}", message);
            }
        }

        // Now we send the server's response - again, the
        // entire buffer requires filling.
        let message = Message(String::from("pong"));
        debug!("Sending {:?}", message);
        if postcard::to_slice(&message, &mut buf).is_ok() {
            // We are deliberately sending all bytes in the buffer so
            // we always know how many bytes to read. There are other
            // ways of doing this though.
            trace!("Sending {}", buf);
            let _ = uart.write(&buf).await;
        }
    }
}

#[cfg(feature = "_nrf")]
fn init_peripherals<'a>(p: bsp::NetworkPeripherals) -> Uarte<'a, bsp::NetworkUarte> {
    let uarte_config = uarte::Config::default();
    Uarte::new(
        p.uarte,
        p.uarte_interrupt,
        p.uarte_rx_pin,
        p.uarte_tx_pin,
        uarte_config,
    )
}

#[cfg(feature = "_stm32")]
fn init_peripherals<'a>(
    p: bsp::NetworkPeripherals,
) -> Uart<'a, bsp::NetworkUart, bsp::NetworkUartTxDma, bsp::NetworkUartRxDma> {
    let uart_config = usart::Config::default();
    Uart::new(
        p.uart,
        p.uart_rx_pin,
        p.uart_tx_pin,
        p.uart_interrupt,
        p.uart_tx_dma,
        p.uart_rx_dma,
        uart_config,
    )
}
