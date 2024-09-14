use embassy_stm32::{
    gpio::{AnyPin, Flex, Level, Output, Pin, Speed},
    pac::gpio::Gpio,
    Peripheral,
};

pub struct StepMotor {
    pub direction_pin: Output<'static, AnyPin>,
    pub step_pin: Output<'static, AnyPin>,
    pub enable_pin: Output<'static, AnyPin>,
    pub sleep_pin: Output<'static, AnyPin>,
    pub reset_pin: Output<'static, AnyPin>,
}
