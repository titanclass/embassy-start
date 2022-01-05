use embassy_stm32::Peripherals;

pub type NetworkUart = embassy_stm32::peripherals::UART7;
pub type NetworkUartRxPin = embassy_stm32::peripherals::PF6;
pub type NetworkUartTxPin = embassy_stm32::peripherals::PF7;
pub type NetworkUartRxDma = embassy_stm32::peripherals::DMA1_CH0;
pub type NetworkUartTxDma = embassy_stm32::peripherals::DMA1_CH1;

pub struct NetworkPeripherals {
    pub uart: NetworkUart,
    pub uart_rx_pin: NetworkUartRxPin,
    pub uart_tx_pin: NetworkUartTxPin,
    pub uart_rx_dma: NetworkUartRxDma,
    pub uart_tx_dma: NetworkUartTxDma,
}

pub fn init(p: Peripherals) -> NetworkPeripherals {
    NetworkPeripherals {
        uart: p.UART7,
        uart_rx_pin: p.PF6,
        uart_tx_pin: p.PF7,
        uart_rx_dma: p.DMA1_CH0,
        uart_tx_dma: p.DMA1_CH1,
    }
}
