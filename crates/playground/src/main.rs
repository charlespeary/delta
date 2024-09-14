#![no_std]
#![no_main]

use defmt::*;
use delta_electronics::step_motor::StepMotor;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut motor_1 = StepMotor {
        direction_pin: Output::new(p.PA3, Level::High, Speed::Low).degrade(),
        step_pin: Output::new(p.PC0, Level::High, Speed::Low).degrade(),
        enable_pin: Output::new(p.PC3, Level::Low, Speed::Low).degrade(),
        sleep_pin: Output::new(p.PF3, Level::High, Speed::Low).degrade(),
        reset_pin: Output::new(p.PF5, Level::High, Speed::Low).degrade(),
    };

    // let mut direction = Output::new(p.PA3, Level::High, Speed::Low);
    // let mut step = Output::new(p.PC0, Level::High, Speed::Low);
    // let mut enable = Output::new(p.PC3, Level::Low, Speed::Low);
    // let mut sleep = Output::new(p.PF3, Level::High, Speed::Low);
    // let mut reset = Output::new(p.PF5, Level::High, Speed::Low);

    // loop {
    //     step.set_low();
    //     Timer::after_millis(50).await;
    //     step.set_high();
    //     Timer::after_millis(50).await;
    // }
}
