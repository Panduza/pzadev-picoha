use hal::clocks::ClocksManager;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Alias for our HAL crate
use rp2040_hal as hal;

use embedded_hal::PwmPin;
use rp2040_hal::clocks::Clock;
use rp2040_hal::Timer;

use rp2040_hal::pwm::{CountFallingEdge, CountRisingEdge, InputHighRunning};

use rp2040_hal::{
    gpio::{AnyPin, PinId},
    pwm::{self, ChannelId, Slice, SliceId, ValidPwmOutputPin, ValidSliceMode},
};

const FSYS: u32 = 125_000_000u32;

pub struct Pwm_Channel_A<SliceANum: SliceId> {
    sliceA: Slice<SliceANum, pwm::FreeRunning>,
}

impl<SliceANum: SliceId> Pwm_Channel_A<SliceANum> {
    pub fn new<PinA: AnyPin>(mut sliceA: Slice<SliceANum, pwm::FreeRunning>, pin_a: PinA) -> Self
    where
        PinA::Id: ValidPwmOutputPin<SliceANum, pwm::A>,
    {
        sliceA.channel_a.output_to(pin_a);

        Self { sliceA: sliceA }
    }

    pub fn enable(&mut self) {
        self.sliceA.enable();
    }

    pub fn set_freq(&mut self, fpwm: f32) -> (f32, f32, u16, u16, f32, u8, u8) {
        let period_wanted = (FSYS as f32) / fpwm;
        // let top = self.pwm_slices.pwm5.get_top();
        let precision = 0.1f32;
        let mut top = (period_wanted / 2.0).clamp(0.0, u16::MAX as f32);

        self.sliceA.set_ph_correct();
        self.sliceA.set_div_int(1);
        self.sliceA.set_div_frac(0);

        let csr_ph_correct = 1u8;
        let div_int = 1u8;
        let div_frac = 0u8;

        let period = (top as f32 + 1.0)
            * (csr_ph_correct as f32 + 1.0)
            * (div_int as f32 + div_frac as f32 / 16.0);

        let (period_wanted, period, top, iterations, real_fpwm, div_frac, div_int) =
            binary_search(period_wanted, period, top as u16, precision);

        self.sliceA.set_top(top);
        self.sliceA.set_div_frac(div_frac);
        self.sliceA.set_div_int(div_int);

        (
            period_wanted,
            period,
            top,
            iterations,
            real_fpwm,
            div_frac,
            div_int,
        )
    }

    pub fn set_duty(&mut self, duty: u16) {
        self.sliceA.channel_a.set_duty(duty);
    }
}

/////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////

pub struct Pwm_Channel_B<SliceANum: SliceId> {
    sliceB: Slice<SliceANum, pwm::FreeRunning>,
}

impl<SliceBNum: SliceId> Pwm_Channel_B<SliceBNum> {
    pub fn new<PinB: AnyPin>(mut sliceB: Slice<SliceBNum, pwm::FreeRunning>, pin_b: PinB) -> Self
    where
        PinB::Id: ValidPwmOutputPin<SliceBNum, pwm::B>,
    {
        sliceB.channel_b.output_to(pin_b);

        Self { sliceB: sliceB }
    }

    pub fn enable(&mut self) {
        self.sliceB.enable();
    }

    pub fn set_freq(&mut self, fpwm: f32) -> (f32, f32, u16, u16, f32, u8, u8) {
        let period_wanted = (FSYS as f32) / fpwm;
        // let top = self.pwm_slices.pwm5.get_top();
        let precision = 0.1f32;
        let mut top = (period_wanted / 2.0).clamp(0.0, u16::MAX as f32);

        self.sliceB.set_ph_correct();
        self.sliceB.set_div_int(1);
        self.sliceB.set_div_frac(0);

        let csr_ph_correct = 1u8;
        let div_int = 1u8;
        let div_frac = 0u8;

        let period = (top as f32 + 1.0)
            * (csr_ph_correct as f32 + 1.0)
            * (div_int as f32 + div_frac as f32 / 16.0);

        let (period_wanted, period, top, iterations, real_fpwm, div_frac, div_int) =
            binary_search(period_wanted, period, top as u16, precision);

        self.sliceB.set_top(top);
        self.sliceB.set_div_frac(div_frac);
        self.sliceB.set_div_int(div_int);

        (
            period_wanted,
            period,
            top,
            iterations,
            real_fpwm,
            div_frac,
            div_int,
        )
    }

    pub fn set_duty(&mut self, duty: u16) {
        self.sliceB.channel_b.set_duty(duty);
    }
}

/////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////

fn binary_search(
    period_wanted: f32,
    mut period: f32,
    mut top: u16,
    precision: f32,
) -> (f32, f32, u16, u16, f32, u8, u8) {
    let mut csr_ph_correct = 1u8;
    let mut div_int = 1u8;
    let mut div_frac = 0u8;

    // Iterative adjustment to achieve the desired frequency
    let mut iterations = 0;
    let max_iterations = 20; // Set an upper limit on iterations

    let mut lower_bound_div_frac = 0;
    let mut upper_bound_div_frac = u8::MAX;

    let mut lower_bound_div_int = 1;
    let mut upper_bound_div_int = u8::MAX;

    let fpwm_wanted = (FSYS as f32) / period_wanted;
    let mut real_fpwm = 0.0;

    // Iterative adjustment to achieve the desired frequency
    while iterations < max_iterations {
        period = (top as f32 + 1.0)
            * (csr_ph_correct as f32 + 1.0)
            * (div_int as f32 + (div_frac as f32 / 16.0));

        real_fpwm = (FSYS as f32) / period;
        // Check if the adjusted period is within the desired range
        if (real_fpwm == fpwm_wanted)
            || fpwm_wanted >= real_fpwm - precision && fpwm_wanted <= real_fpwm + precision
        {
            break;
        }
        // Binary search for a faster adjustment
        if period_wanted < period {
            upper_bound_div_frac = div_frac;
        } else {
            lower_bound_div_frac = div_frac;
        }

        div_frac = (lower_bound_div_frac + upper_bound_div_frac) / 2;

        // if period_wanted < period {
        //     upper_bound_div_int = div_int;
        // } else {
        //     lower_bound_div_int = div_int;
        // }

        // div_int = (lower_bound_div_int + upper_bound_div_int) / 2;

        iterations += 1;
    }
    return (
        period_wanted,
        period,
        top,
        iterations,
        real_fpwm,
        div_frac,
        div_int,
    );
}

// End of file
