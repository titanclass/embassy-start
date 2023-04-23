use embassy_stm32::interrupt::USART2;
use embassy_stm32::{interrupt, Peripherals};

pub type NetworkUart = embassy_stm32::peripherals::USART2;
pub type NetworkUartRxPin = embassy_stm32::peripherals::PD6;
pub type NetworkUartTxPin = embassy_stm32::peripherals::PD5;
pub type NetworkUartRxDma = embassy_stm32::peripherals::DMA1_CH5;
pub type NetworkUartTxDma = embassy_stm32::peripherals::DMA1_CH6;

pub struct NetworkPeripherals {
    pub uart: NetworkUart,
    pub uart_interrupt: USART2,
    pub uart_rx_pin: NetworkUartRxPin,
    pub uart_tx_pin: NetworkUartTxPin,
    pub uart_rx_dma: NetworkUartRxDma,
    pub uart_tx_dma: NetworkUartTxDma,
}

pub fn init(p: Peripherals) -> NetworkPeripherals {
    NetworkPeripherals {
        uart: p.USART2,
        uart_interrupt: interrupt::take!(USART2),
        uart_rx_pin: p.PD6,
        uart_tx_pin: p.PD5,
        uart_rx_dma: p.DMA1_CH5,
        uart_tx_dma: p.DMA1_CH6,
    }
}
