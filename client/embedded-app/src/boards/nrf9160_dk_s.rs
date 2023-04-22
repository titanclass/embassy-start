use embassy_nrf::{bind_interrupts, peripherals, uarte, Peripherals};

pub type NetworkUarte = peripherals::SERIAL0;
pub type NetworkUarteRxPin = peripherals::P0_12;
pub type NetworkUarteTxPin = peripherals::P0_10;

bind_interrupts!(pub struct NetworkPeripheralsIrqs {
    UARTE0_SPIM0_SPIS0_TWIM0_TWIS0 => uarte::InterruptHandler<peripherals::SERIAL0>;
});

pub struct NetworkPeripherals {
    pub uarte: NetworkUarte,
    pub uarte_interrupt: NetworkPeripheralsIrqs,
    pub uarte_rx_pin: NetworkUarteRxPin,
    pub uarte_tx_pin: NetworkUarteTxPin,
}

pub fn init(p: Peripherals) -> NetworkPeripherals {
    NetworkPeripherals {
        uarte: p.SERIAL0,
        uarte_interrupt: NetworkPeripheralsIrqs,
        uarte_rx_pin: p.P0_12,
        uarte_tx_pin: p.P0_10,
    }
}
