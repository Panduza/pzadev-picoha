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

The product USB IDs **must** be free shared USB VID/PID pair for CDC devices. The products can be differentiated with their serial number.

The vendor ID of the device **must** be 0x16C0.

The product ID of the device **must** be 0x05E1.

The serial number of the device **must** be XXXX.

#### [REQ_1001] USB Protocol

The USB protocol **must** be a custom protocol named HA and frames are encapsulated in SLIP format.

The frames are composed of:

- a 16 bits request code
- Data
- a 16 bits CRC

There are two possible transfer mechanisms. For each, there can be only one request at a time before receiving an answer.

- Standard request : the transfer is initiated by the host and wait for an answer from the host adapter
- Notification : the transfer is initiated by the host adapter and wait for an answer from the host


***Requests***

Generic requests

| Code        | Function  |
| ----------- | --------- |
| `0x0000`    | Ping      |
| `0x0001`    | ItfType   |
| `0x0002`    | Version   |
| `0x0003`    | IdGet     |

PWM

| Code        | Function        |
| ----------- | --------------  |
| `0x0200`    | PwmStart        |
| `0x0201`    | PwmStop         |
| `0x0202`    | SetFrequency    |
| `0x0203`    | GetFrequency    |
| `0x0204`    | SetDutyCycle    |
| `0x0205`    | GetDutycycle    |


***Answers***

Shared answers

| Code        | Function      |
| ----------- | ------------- |
| `0xFFFF`    | Good          |
| `0xFFFE`    | ErrGeneric    |
| `0xFFFD`    | ErrCRC        |
| `0xFFFC`    | ErrUnknownCode|
| `0xFFFB`    | ErrInvalidArgs|
| `0xFFFA`    | ErrBusy       |

Generic answers

| Code        | Function      |
| ----------- | ------------- |
| `0xFEFF`    | VersionResp   |
| `0xFEFE`    | ItfTypeResp   |
| `0xFEFD`    | IdResp        |

Pwm answers

TODO : add response code

| Code        | Function      |
| ----------- | ------------- |
| `XXXX`      | Frequency     |
| `XXXX`      | DutyCycle     |



### SECTION 2: Features

#### [REQ_2000] PWM Start/Stop:

PWM channels **must** be able to be started and stopped individually.

<br>The firmware **must** enable PWM when the command `0x0200` is received.<br>
The firmware **must** disable PWM when the command `0x2001` is received.


#### [REQ_2001] PWM signal output

The firmware **must** generate a PWM signal on GPIO1 with customizable parameters such as frequency ranging from 7 Hz to 125 MHz and duty cycle (between 0 and 100%).

<br>The firmware **must** set the frequency when the command `0x0202` is received.<br>
The firmware **must** set the duty cycle en the command `0x0204` is received.

#### [REQ_2002] PWM signal input

The firmware **must** read PWM signal on GPIO3 for frequency and duty cycle measurement.

<br>The firmware **must** measure the frequency when the command `0x0203` is received.<br>
The firmware **must** measure the duty cycle en the command `0x0205` is received.

#### [REQ_2003] PWM interrupt management:

The firmware **must** support generating interrupts based on specific events.

The events are : 
- setting the frequency outside the operating interval
- setting the duty cycle outside the operating interval