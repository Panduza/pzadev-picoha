# UART

- Directory *src* to store the code
- Directory *examples* to store python usages

## Requirements

### SECTION 1: USB Interface & Protocol

#### [REQ_1000] USB IDs

The product USB IDs **must** be free shared USB VID/PID pair for CDC devices. The products can be differentiated with their serial number.

The vendor ID of the device **must** be 0x16C0.

The product ID of the device **must** be 0x05E1.

The serial number of the device **must** be XXXX.

#### [REQ_1010] USB protocol

The product **must** use a custom USB protocol. This protocol uses USB CDC and SLIP protocol to encapsulate the frames.

The frames are composed of:

<img src="img/data_frame.jpg" alt="Data frame" title="Data frame">

- a 16 bits request code
- Data
- a 16 bits CRC

There are two possible transfer mechanisms. For each, there can be only one request at a time before receiving an answer.
- Standard request: the transfer is initiated by the host and wait for an answer from the host adapter

<img src="img/standard_request.jpg" alt="Data frame" title="Data frame">

- Notification: the transfer is initiated by the host adapter and wait for an answer from the host

<img src="img/notification.jpg" alt="Data frame" title="Data frame">

<br/>

**Request code**:

Digital functions

| Range start | Range end | Function              |
| ----------- | --------- | --------------------- |
| `0x0000`    | `0x00FF`  | Generic requests      |
| `0x0100`    | `0x01FF`  | GPIO request codes    |
| `0x0200`    | `0x02FF`  | PWM request codes     |
| `0x0300`    | `0x03FF`  | Encoder request codes |
| `0x0400`    | `0x04FF`  | Timer request codes   |

 Protocols

| Range start | Range end | Function   |
| ----------- | --------- | ---------- |
| `0x3800`    | `0x38FF`  | UART       |
| `0x3900`    | `0x39FF`  | SPI master |
| `0x3A00`    | `0x3AFF`  | SPI slave  |
| `0x3B00`    | `0x3BFF`  | I2C master |
| `0x3C00`    | `0x3CFF`  | I2C slave  |
| `0x3D00`    | `0x3DFF`  | Modbus RTU |
| `0x3E00`    | `0x3EFF`  | *TBD*      |
| `0x3F00`    | `0x3FFF`  | *TBD*      | 
| `0x4000`    | `0x40FF`  | CAN 2.0    |
| `0x4100`    | `0x41FF`  | CAN FD     |
| `0x4200`    | `0x42FF`  | LIN        |

Digital functions answers

| Range start | Range end | Function              |
| ----------- | --------- | --------------------- |
| `0xFFFF`    | `0xFF00`  | Shared answers        |
| `0xFEFF`    | `0xFE00`  | Generic answers       |
| `0xFDFF`    | `0xFD00`  | GPIO answers codes    |
| `0xFCFF`    | `0xFC00`  | PWM answers codes     |
| `0xFBFF`    | `0xFB00`  | Encoder answers codes |
| `0xFAFF`    | `0xFA00`  | Timer answers codes   |

 Protocols answers

| Range start | Range end | Function   |
| ----------- | --------- | ---------- |
| `0xB4FF`    | `0xB400`  | UART       |
| `0xB3FF`    | `0xB300`  | SPI master |
| `0xB2FF`    | `0xB200`  | SPI slave  |
| `0xB1FF`    | `0xB100`  | I2C master |
| `0xB0FF`    | `0xB000`  | I2C slave  |
| `0xAFFF`    | `0xAF00`  | Modbus RTU |
| `0xAEFF`    | `0xAE00`  | *TBD*      |
| `0xADFF`    | `0xAD00`  | *TBD*      | 
| `0xACFF`    | `0xAC00`  | CAN 2.0    |
| `0xABFF`    | `0xAB00`  | CAN FD     |
| `0xAAFF`    | `0xAA00`  | LIN        |

***Requests***

Generic requests

| Code        | Function  |
| ----------- | --------- |
| `0x0000`    | Ping      |
| `0x0001`    | ItfType   |
| `0x0002`    | Version   |
| `0x0003`    | IdGet     |

GPIO request codes

| Code        | Function  |
| ----------- | --------- |
| `0x0100`    | GpioDirSet|
| `0x0101`    | GpioDirGet|
| `0x0102`    | GpioRead  |
| `0x0103`    | GpioWrite |

UART

| Code        | Function  |
| ----------- | --------- |
| `0x3800`    | UartBegin |
| `0x3801`    | UartEscape|
| `0x3802`    | DataTX    |
| `0x3803`    | DataRXGet |
| `0x3804`    | SetBaud   |
| `0x3805`    | SetParity |
| `0x3806`    | SetStopBit|
| `0x3807`    | SetDataSz |
| `0x3808`    | StopCom   |

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

GPIO answers codes

| Code        | Function      |
| ----------- | ------------- |
| `0xFDFF`    | GpioValue     |
| `0xFDFE`    | GpioDir       |

UART answers

| Code        | Function      |
| ----------- | ------------- |
| `0xB4FF`    | DataRX        |

### SECTION 2: Features

#### [REQ_2000] Send data

The product **must** send data to one other product using UART communication.

#### [REQ_2010] Receive data

The product **must** receive data from one other product using UART communication.

#### [REQ_2020] Baud rate

The product **must** enable the baud rate configuration. The baud rate can be out of the standards baud rates used in UART.

#### [REQ_2030] Parity

The product **must** enable the parity bit configuration.

#### [REQ_2040] Stop bits

The product **must** enable the stop bits configuration.

#### [REQ_2050] Data size

The product **must** enable the data size configuration.

#### [REQ_2060] UART pinout

The product UART TX is connected to GP0 and the UART RX is connected to GP1.

#### [REQ_2070] PIO

The product **must** use the Programmable I/O of the Pi Pico board.

#### [REQ_2080] Start of the program

The internal LED of the product **must** be turned on during the start and the execution of the firmware.

#### [REQ_2090] End of the program

The internal LED of the product **must** be turned off when the firmware is stopped.

#### [REQ_2100] UART disconnected

The firmware **must** still be running when the UART connection is disconnected.

#### [REQ_2110] UART stop

The firmware **must** be able to stop the UART in the middle of a communication when the command ***StopCom*** is received.
