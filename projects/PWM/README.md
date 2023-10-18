# PWM

- Directory *src* to store the code
- Directory *examples* to store python usages

## Pinout and characteristics

![](img/picoha_pwm_pinout.png)

|PWM characteristics 		|Pi Pico		|
|------- 			        | -------		|
|PWM signal frequency range	|7 Hz to 125 Mhz|
|Independent PWM channels	|8			    |
|PWM output			        |16			    |
|Pulse width resolution	    |16 bits		|

The RP2040 PWM block has 8 identical PWM slices, each with two output channels (A/B), where the B pin can also be used as an input for frequency and duty cycle measurement. That means each slice can drive two PWM output signals, or measure the frequency or duty cycle of an input signal. This gives a total of up to 16 controllable PWM outputs.

## Requirements

### SECTION 1: USB Interface & Protocol

#### [REQ_1000] USB IDs

The vendor ID of the device **must** be 0x16C0

The product ID of the device **must** be 0x05E1

The serial number of the device **must** be XXXX

#### [REQ_1001] USB Protocol

The USB protocol **must** be a custom protocol named HA and frames are encapsulated in SLIP format.

### SECTION 2: Features

#### [REQ_2000] PWM configuration:

The firmware **must** allow configuration of PWM channel 0A with customizable parameters such as frequency and duty cycle (between 0 and 100%).

#### [REQ_2001] PWM Start/Stop:

PWM channels **must** be able to be started and stopped individually.

#### [REQ_2002] PWM signal output

After configuration, the firmware **must** generate a PWM signal on GPIO1 at a frequency ranging from 7 Hz to 125 MHz.

#### [REQ_2003] PWM signal input

After configuration, the firmware **must** read PWM signal on GPIO3 for frequency and duty cycle measurement.

#### [REQ_2004] PWM interrupt management:

The firmware **must** support generating interrupts based on specific events.
