use embassy_nrf::{interrupt, peripherals, Peripherals};

pub type NetworkUarte = peripherals::UARTE0;
pub type NetworkUarteInterrupt = interrupt::UARTE0_UART0;
pub type NetworkUarteRxPin = peripherals::P0_04;
pub type NetworkUarteTxPin = peripherals::P0_02;

pub struct NetworkPeripherals {
    pub uarte: NetworkUarte,
    pub uarte_interrupt: NetworkUarteInterrupt,
    pub uarte_rx_pin: NetworkUarteRxPin,
    pub uarte_tx_pin: NetworkUarteTxPin,
}

pub fn init(p: Peripherals) -> NetworkPeripherals {
    NetworkPeripherals {
        uarte: p.UARTE0,
        uarte_interrupt: interrupt::take!(UARTE0_UART0),
        uarte_rx_pin: p.P0_04,
        uarte_tx_pin: p.P0_02,
    }
}
