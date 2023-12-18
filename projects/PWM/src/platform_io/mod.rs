pub mod pwm_ctrl;
pub use pwm_ctrl::PwmCtrltInputFreq;

#[derive(Debug)]
pub enum PlatformError {
    InitError,
}

pub trait PlatformLed {
    fn led_on(&mut self);
    fn led_off(&mut self);
}

pub trait PlatformSleep {
    fn sleep_ms(&mut self, delay_ms: u32);
}

pub trait PlatformData {
    //fn get_led  (&mut self) -> &mut dyn PlatformLed;
    fn get_sleep(&mut self) -> &mut dyn PlatformSleep;
    // fn get_pins(&mut self) -> &mut dyn PwmCtrl;
}
