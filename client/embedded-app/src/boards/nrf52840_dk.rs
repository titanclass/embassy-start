use embassy_nrf::{bind_interrupts, peripherals, uarte, Peripherals};

pub type NetworkUarte = peripherals::UARTE0;
pub type NetworkUarteRxPin = peripherals::P1_12;
pub type NetworkUarteTxPin = peripherals::P1_10;

bind_interrupts!(pub struct NetworkPeripheralsIrqs {
    UARTE0_UART0 => uarte::InterruptHandler<peripherals::UARTE0>;
});

pub struct NetworkPeripherals {
    pub uarte: NetworkUarte,
    pub uarte_interrupt: NetworkPeripheralsIrqs,
    pub uarte_rx_pin: NetworkUarteRxPin,
    pub uarte_tx_pin: NetworkUarteTxPin,
}

pub fn init(p: Peripherals) -> NetworkPeripherals {
    NetworkPeripherals {
        uarte: p.UARTE0,
        uarte_interrupt: NetworkPeripheralsIrqs,
        uarte_rx_pin: p.P1_12,
        uarte_tx_pin: p.P1_10,
    }
}
