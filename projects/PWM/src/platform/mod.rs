use embedded_hal::blocking::delay::DelayMs;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

use embedded_hal::PwmPin;
use rp2040_hal::clocks::Clock;

use rp2040_hal::Timer;

use rp2040_hal::pwm::{CountFallingEdge, CountRisingEdge, InputHighRunning};

use rp2040_hal::{
    gpio::{AnyPin, PinId},
    pwm::{
        self, AnySlice, ChannelId, DynChannelId, Slice, SliceId, ValidPwmInputPin,
        ValidPwmOutputPin, ValidSliceMode,
    },
};

const FSYS: u32 = 125_000_000u32;

//////////////////////////////////////////////////////
/// PWM Output for channel A
//////////////////////////////////////////////////////
pub struct PwmOutput_A<SliceNum: SliceId> {
    slice: Slice<SliceNum, pwm::FreeRunning>,
}

impl<SliceNum: SliceId> PwmOutput_A<SliceNum> {
    pub fn new<Pin: AnyPin>(mut slice: Slice<SliceNum, pwm::FreeRunning>, pin: Pin) -> Self
    where
        Pin::Id: ValidPwmOutputPin<SliceNum, pwm::A>,
    {
        slice.channel_a.output_to(pin);
        Self { slice }
    }

    pub fn enable(&mut self) {
        self.slice.enable();
    }

    pub fn disable(&mut self) {
        self.slice.disable();
    }

    ////////////////////////////////////////////////////////////////////////////////////
    /// TODO : essayer de faire une fonction pwm configure qui va permettre de configurer les div int,frac...
    /// et ensuite de choisir le bon div int en foction de la range du pwm signal souhaite
    /// //////////////////////////////////////////////////////////////////////////

    // pub fn configure(&mut self) {
    //     self.slice.set_ph_correct();
    //     self.slice.set_div_int(255);
    //     self.slice.set_div_frac(0);
    // }

    pub fn set_freq(&mut self, fpwm: f32) -> (f32, f32, u16, u16, f32, u8, u8) {
        let period_wanted = (FSYS as f32) / fpwm;
        // let top = self.pwm_slices.pwm5.get_top();
        let precision = 0.1f32;

        self.slice.set_ph_correct();
        self.slice.set_div_int(1);
        self.slice.set_div_frac(0);

        let mut csr_ph_correct = 1u8;
        let mut div_int = 1u8;
        let div_frac = 0u8;

        if fpwm < 1000.0 {
            div_int = 255;
            self.slice.set_div_int(255);
        } else if fpwm > 62_500_500.0 {
            csr_ph_correct = 0u8;
            self.slice.clr_ph_correct();
        }

        let top = (period_wanted
            / ((csr_ph_correct + 1) as f32 * (div_int + (div_frac / 16)) as f32))
            - 1.0;

        // fPWM = fsys/period
        // period = (TOP + 1)*(CSR_PH_CORRECT + 1)*(DIV_INT + (DIV_FRAC/16))

        let period = (top as f32 + 1.0)
            * (csr_ph_correct as f32 + 1.0)
            * (div_int as f32 + (div_frac as f32 / 16.0));

        let real_fpwm = (FSYS as f32) / period;

        let (period_wanted, period, top, iterations, real_fpwm, div_frac, div_int) = binary_search(
            period_wanted,
            period,
            top as u16,
            csr_ph_correct,
            div_int,
            div_frac,
            precision,
        );

        self.slice.set_top(top as u16);
        self.slice.set_div_frac(div_frac);
        self.slice.set_div_int(div_int);

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

    pub fn set_duty(&mut self, duty: f32) {
        let value = self.slice.get_top() as f32 * duty / 100.0;
        self.slice.channel_a.set_duty(value as u16);
    }

    pub fn get_duty(&mut self) -> f32 {
        return (self.slice.channel_a.get_duty() as f32 / self.slice.get_top() as f32) * 100.0;
    }
}

//////////////////////////////////////////////////////
/// PWM Output for channel B
//////////////////////////////////////////////////////
pub struct PwmOutput_B<SliceNum: SliceId> {
    slice: Slice<SliceNum, pwm::FreeRunning>,
}

impl<SliceNum: SliceId> PwmOutput_B<SliceNum> {
    pub fn new<PinB: AnyPin>(mut slice: Slice<SliceNum, pwm::FreeRunning>, pin_b: PinB) -> Self
    where
        PinB::Id: ValidPwmOutputPin<SliceNum, pwm::B>,
    {
        slice.channel_b.output_to(pin_b);

        Self { slice: slice }
    }

    pub fn enable(&mut self) {
        self.slice.enable();
    }

    pub fn disable(&mut self) {
        self.slice.disable();
    }

    pub fn set_freq(&mut self, fpwm: f32) -> (f32, f32, u16, u16, f32, u8, u8) {
        let period_wanted = (FSYS as f32) / fpwm;
        // let top = self.pwm_slices.pwm5.get_top();
        let precision = 0.1f32;

        self.slice.set_ph_correct();
        self.slice.set_div_int(1);
        self.slice.set_div_frac(0);

        let mut csr_ph_correct = 1u8;
        let mut div_int = 1u8;
        let div_frac = 0u8;

        if fpwm < 1000.0 {
            div_int = 255;
            self.slice.set_div_int(255);
        } else if fpwm > 62_500_500.0 {
            csr_ph_correct = 0u8;
            self.slice.clr_ph_correct();
        }

        let top = (period_wanted
            / ((csr_ph_correct + 1) as f32 * (div_int + (div_frac / 16)) as f32))
            - 1.0;

        // fPWM = fsys/period
        // period = (TOP + 1)*(CSR_PH_CORRECT + 1)*(DIV_INT + (DIV_FRAC/16))

        let period = (top as f32 + 1.0)
            * (csr_ph_correct as f32 + 1.0)
            * (div_int as f32 + (div_frac as f32 / 16.0));

        let real_fpwm = (FSYS as f32) / period;

        let (period_wanted, period, top, iterations, real_fpwm, div_frac, div_int) = binary_search(
            period_wanted,
            period,
            top as u16,
            csr_ph_correct,
            div_int,
            div_frac,
            precision,
        );

        self.slice.set_top(top as u16);
        self.slice.set_div_frac(div_frac);
        self.slice.set_div_int(div_int);

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

    pub fn set_duty(&mut self, duty: f32) {
        let value = self.slice.get_top() as f32 * duty / 100.0;
        self.slice.channel_b.set_duty(value as u16);
    }

    pub fn get_duty(&mut self) -> f32 {
        return (self.slice.channel_b.get_duty() as f32 / self.slice.get_top() as f32) * 100.0;
    }
}

// Define a generic trait for PWM output
// pub trait PwmOutputTrait<SliceNum: SliceId> {
//     fn new<Pin: AnyPin>(slice: Slice<SliceNum, pwm::FreeRunning>, pin: Pin) -> Self
//     where
//         Pin::Id: ValidPwmOutputPin<SliceNum, pwm::A>;
//     fn enable(&mut self);
//     fn set_freq(&mut self, fpwm: f32) -> (f32, f32, u16, u16, f32, u8, u8);
//     fn set_duty(&mut self, duty: u16);
// }

// TODO : a generic structure for PWM Output

// pub struct PwmOutput<SliceNum: SliceId, C: ChannelId> {
//     slice: Slice<SliceNum, pwm::FreeRunning>,
//     channel: C,
// }

// impl<SliceNum: SliceId, C: ChannelId> PwmOutput<SliceNum, C> {
//     pub fn new<Pin: AnyPin>(
//         mut slice: Slice<SliceNum, pwm::FreeRunning>,
//         pin: Pin,
//         channel: C,
//     ) -> Self
//     where
//         Pin::Id: ValidPwmOutputPin<SliceNum, C>,
//     {
//         let _ = match channel {
//             A => slice.channel_a.output_to(pin),
//             B => slice.channel_b.output_to(pin),
//         };
//         Self { slice, channel }
//     }
// }
/////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////
/// PWM Input only channel B
//////////////////////////////////////////////////////

pub struct PwmInput<SliceNum: SliceId> {
    slice: Slice<SliceNum, pwm::InputHighRunning>,
}

impl<SliceNum: SliceId> PwmInput<SliceNum> {
    pub fn new<Pin: AnyPin>(mut slice: Slice<SliceNum, pwm::InputHighRunning>, pin: Pin) -> Self
    where
        Pin::Id: ValidPwmInputPin<SliceNum>,
    {
        slice.channel_b.input_from(pin);

        Self { slice: slice }
    }

    // pub fn enable(&mut self) {
    //     self.slice.clr_ph_correct();
    //     self.slice.enable();
    // }

    // pub fn disable(&mut self) {
    //     self.slice.disable();
    // }

    pub fn measure_freq(&mut self, timer: &mut Timer) -> (f32, f32) {
        // self.slice.set_div_int(100);

        self.slice.enable();

        timer.delay_ms(20);
        // self.slice.disable();

        let counting_rate = 125000000.0 * 0.01;
        let max_possible_count = counting_rate * 0.01;
        let mut counter = self.slice.get_counter();
        let duty = counter as f32 / max_possible_count;
        let freq = 125000000.0 / (counter as f32);

        while counter < 65530 {
            counter = self.slice.get_counter();
        }

        (counter as f32, freq as f32)
    }
}
//////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////

fn binary_search(
    period_wanted: f32,
    mut period: f32,
    mut top: u16,
    mut csr_ph_correct: u8,
    mut div_int: u8,
    mut div_frac: u8,
    precision: f32,
) -> (f32, f32, u16, u16, f32, u8, u8) {
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
