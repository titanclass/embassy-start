use embassy_nrf::{interrupt, peripherals, Peripherals};

pub type NetworkUarte = peripherals::UARTETWISPI0;
pub type NetworkUarteInterrupt = interrupt::UARTE0_SPIM0_SPIS0_TWIM0_TWIS0;
pub type NetworkUarteRxPin = peripherals::P0_12;
pub type NetworkUarteTxPin = peripherals::P0_10;

pub struct NetworkPeripherals {
    pub uarte: NetworkUarte,
    pub uarte_interrupt: NetworkUarteInterrupt,
    pub uarte_rx_pin: NetworkUarteRxPin,
    pub uarte_tx_pin: NetworkUarteTxPin,
}

pub fn init(p: Peripherals) -> NetworkPeripherals {
    NetworkPeripherals {
        uarte: p.UARTETWISPI0,
        uarte_interrupt: interrupt::take!(UARTE0_SPIM0_SPIS0_TWIM0_TWIS0),
        uarte_rx_pin: p.P0_12,
        uarte_tx_pin: p.P0_10,
    }
}
