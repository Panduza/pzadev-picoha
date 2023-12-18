/// PWM control interface

/// Possible errors for PWM controller
#[derive(Debug)]
pub enum PwmCtrlError {
    InitError,
    PinConfigError,
    PinHalError,

    /// Invalid requested direction
    PinInvalidDir,

    /// Invalid pin index
    PinInvalidIndex,

    /// Requested pin is not in the given dir
    PinMismatchDir,
}

pub type PwmSliceIndex = usize;
pub type GpioIndex = usize;
pub type Value = f32;

/// The GPIO controller controls the state of the GPIOs
pub trait PwmCtrlOutputA {
    fn enable(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<(), PwmCtrlError>;

    fn disable(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<(), PwmCtrlError>;

    fn freq_set(
        &mut self,
        pwm: PwmSliceIndex,
        gpio: GpioIndex,
        freq: Value,
    ) -> Result<(), PwmCtrlError>;

    // fn freq_get(&self, idx: PinIndex) -> Result<PinDir, PwmCtrlError>;

    fn duty_set(
        &mut self,
        pwm: PwmSliceIndex,
        gpio: GpioIndex,
        duty: Value,
    ) -> Result<(), PwmCtrlError>;

    fn duty_get(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<Value, PwmCtrlError>;
}

pub trait PwmCtrlOutputB {
    fn enable(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<(), PwmCtrlError>;

    fn disable(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<(), PwmCtrlError>;

    fn freq_set(
        &mut self,
        pwm: PwmSliceIndex,
        gpio: GpioIndex,
        freq: Value,
    ) -> Result<(), PwmCtrlError>;

    // fn freq_get(&self, idx: PinIndex) -> Result<PinDir, PwmCtrlError>;

    fn duty_set(
        &mut self,
        pwm: PwmSliceIndex,
        gpio: GpioIndex,
        duty: Value,
    ) -> Result<(), PwmCtrlError>;

    fn duty_get(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<Value, PwmCtrlError>;
}

pub trait PwmCtrltInputFreq {
    fn freq_get(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<Value, PwmCtrlError>;
}

pub trait PwmCtrltInputDuty {
    fn duty_get(&mut self, pwm: PwmSliceIndex, gpio: GpioIndex) -> Result<Value, PwmCtrlError>;
}
