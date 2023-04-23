#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{info, unwrap};
use defmt_rtt as _;
use embassy_executor::Spawner;
use panic_probe as _;

mod boards;
mod network;

#[cfg(feature = "microbit-v2")]
use crate::boards::microbit_v2 as bsp;
#[cfg(feature = "nrf52840-dk")]
use crate::boards::nrf52840_dk as bsp;
#[cfg(feature = "nrf9160-dk-s")]
use crate::boards::nrf9160_dk_s as bsp;
#[cfg(feature = "stm32f767zi")]
use crate::boards::stm32f767zi as bsp;
#[cfg(feature = "stm32h743zi")]
use crate::boards::stm32h743zi as bsp;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Network starting");

    #[cfg(feature = "_nrf")]
    let p = embassy_nrf::init(embassy_nrf::config::Config::default());
    #[cfg(feature = "_stm32")]
    let p = embassy_stm32::init(embassy_stm32::Config::default());

    // The general idea is to initialise the board
    // specific peripherals that we will be using.
    // This often ends up being an assignment to
    // a tuple of peripherals.
    let network_peripherals = bsp::init(p);

    // We generally create a task per component
    // that ends up owning a number of peripherals.
    // There are a number of tasks like this and
    // we use either signals or channels to
    // communicate with them.
    unwrap!(spawner.spawn(network::main_task(network_peripherals,)));

    // We end up here normally with a loop and something
    // "main-like" that executes for your application,
    // often with the ability to communicate to the other
    // tasks via signals and channels etc.
}
