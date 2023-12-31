# UART

- Directory *src* to store the code
- Directory *examples* to store python usages

## Requirements

### SECTION 1: USB Interface & Protocol

<img src="img/schema_picoha_protocole.png" alt="USB Protocol" title="USB Protocol">
<br/>

#### [REQ_1000] USB IDs

The product USB IDs **must** be free shared USB VID/PID pair for CDC devices. The products can be differentiated with their manufacturer and product identification and their serial number.

The vendor ID of the device **must** be 0x16C0.

The product ID of the device **must** be 0x05E1.

The manufacturer identification of the device **must** be `"panduza.io"`.

The product identification of the device **must** be `"picoha-uart"`.

The serial number of the device **must** be XXXX.

We chose to use the free shared USB VID/PID instead of the Raspberry Pi VID/PID to assure not to have the product been mistaken with another Raspberry Pi product.

The guidelines to use free USB IDs for shared use are described in the following document: https://github.com/obdev/v-usb/blob/master/usbdrv/USB-IDs-for-free.txt

The guidelines to use Raspberry Pi USB product ID are described in the following document: https://github.com/raspberrypi/usb-pid

#### [REQ_1010] USB device class

The product **must** use USB CDC as its device class.

#### [REQ_1020] USB protocol

The product **must** use Serial Line Internet Protocol (SLIP) as its communication protocol. It is composed of a data payload and a flag that acts as an end delimiter. If this flag is present in the data, then an escape sequence precedes it, so that the receiver does not consider it as the end of the frame.

<img src="img/slip.jpg" alt="Serial Line Internet Protocol" title="Serial Line Internet Protocol">

SLIP flags:

| Hex value   | Abbreviation  | Description             |
| ----------- | ------------- | ----------------------  |
| `0xC0`      | END           | Frame End               |
| `0xDB`      | ESC           | Frame Escape            |
| `0xDC`      | ESC_END       | Transposed Frame End    |
| `0xDD`      | ESC_ESC       | Transposed Frame Escape |

#### [REQ_1030] USB HA protocol

The product **must** use a custom frame protocol named HA protocol. The frames are composed of:

<img src="img/data_frame.jpg" alt="Data frame" title="Data frame">

- a 16 bits request code
- Data
- a 16 bits CRC

#### [REQ_1040] HA protocol CRC

The CRC used in the HA protocol frame must use the standard CRC16 crc-ccitt-false. Parameters of this CRC are:

| Parameter   | Value     |
| ----------- | --------- |
| Polynom     | `0x1021`  |
| Init        | `0xFFFF`  |
| Refin       | `false`   |
| Refout      | `false`   |
| Xorout      | `0x0000`  |
| Check       | `0x29B1`  |

Using the crcmod python library, it can be instanciated using the following call:
```
crc16 = crcmod.mkCrcFun(0x11021, rev=False, initCrc=0xFFFF, xorOut=0x0000)
```
Or using its predefined counterpart:
```
crc16 = crcmod.predefined.mkCrcFun("crc-ccitt-false")
```

#### [REQ_1050] HA protocol transfer machanism

There **must** be two possible transfer mechanisms. For each, there can be only one request at a time before receiving an answer.
- Standard request: the transfer is initiated by the host and wait for an answer from the host adapter

<img src="img/standard_request.jpg" alt="Data frame" title="Data frame">

- Notification: the transfer is initiated by the host adapter and wait for an answer from the host

<img src="img/notification.jpg" alt="Data frame" title="Data frame">

<br/>

### SECTION 2: Generic Requests

***Generic requests***

| Code        | Function  |
| ----------- | --------- |
| `0x0000`    | Ping      |
| `0x0001`    | ItfType   |
| `0x0002`    | Version   |
| `0x0003`    | IdGet     |

***Shared answers***

| Code        | Function      |
| ----------- | ------------- |
| `0xFFFF`    | Good          |
| `0xFFFE`    | ErrGeneric    |
| `0xFFFD`    | ErrCRC        |
| `0xFFFC`    | ErrUnknownCode|
| `0xFFFB`    | ErrInvalidArgs|
| `0xFFFA`    | ErrBusy       |

***Generic answers***

| Code        | Function      |
| ----------- | ------------- |
| `0xFEFF`    | VersionResp   |
| `0xFEFE`    | ItfTypeResp   |
| `0xFEFD`    | IdResp        |

#### [REQ_2000] Ping

The product **must** answer the `Good` (`0xFFFF`) answer when the `Ping` (`0x0000`) request is received.

#### [REQ_2010] Interface type

The product **must** answer the `ItfTypeResp` (`0xFEFE`) answer when the `ItfType` (`0x0001`) request is received. The data returned must be `"picoha-uart"`.

#### [REQ_2020] Version

The product **must** answer the `VersionResp` (`0xFEFF`) answer when the `Version` (`0x0002`) request is received. The data returned must be version of the firmware loaded on the product.

#### [REQ_2030] ID

The product **must** answer the `IdResp` (`0xFEFD`) answer when the `IdGet` (`0x0003`) request is received. The data returned must be unique board ID of the product.

#### [REQ_2040] Good

The product **must** answer the `Good` (`0xFFFF`) code when no error has been encountered and the request has no specific answer.

#### [REQ_2050] Generic error

The product **must** answer the `ErrGeneric` (`0xFFFE`) code when an error not linked to a preexisting error codde is encountered.

#### [REQ_2060] CRC error

The product **must** answer the `ErrCRC` (`0xFFFD`) code when the CRC of a received request is invalid.

#### [REQ_2070] Unknown code error

The product **must** answer the `ErrUnknownCode` (`0xFFFC`) code when the request code received is unknown.

#### [REQ_2080] Invalid arguments error

The product **must** answer the `ErrInvalidArgs` (`0xFFFB`) code when a request is received with the wrong arguments.

#### [REQ_2090] Busy

The product **must** answer the `ErrBusy` (`0xFFFA`) code when an operation is still in progress and the product is busy.

### SECTION 3: UART Requests

| UART Setting | Range              | Default     |
| -----------  | ------------------ | ----------- |
| Baudrate     | up to 15.625 Mbaud | 115200 baud |
| Parity bit   | 0 to 1 bit         | 0 bit       |
| Stop bits    | 1 to 2 bits        | 1 bit       |
| Data bits    | 5 to 9 bits        | 8 bits      |

***UART requests***

| Code        | Function      |
| ----------- | ------------- |
| `0x1000`    | DataTX        |
| `0x1001`    | DataRXGet     |
| `0x1002`    | BaudSet       |
| `0x1003`    | BaudGet       |
| `0x1004`    | SetParity     |
| `0x1005`    | SetStopBit    |
| `0x1006`    | SetDataSz     |
| `0x1007`    | HWFlowControl |
| `0x1008`    | StopCom       |

***UART answers***

| Code        | Function      |
| ----------- | ------------- |
| `0xEFFF`    | DataRX        |
| `0xEFFE`    | Baud          |

#### [REQ_3000] Send data

The product **must** send data to one other product using UART communication when the `DataTX` (`0x1000`) command is received.

#### [REQ_3010] Receive data

The product **must** receive data from one other product using UART communication when the `DataRXGet` (`0x1001`) command is received.

#### [REQ_3020] Write baud rate

The product **must** provide a way to change the baudrate. The request code to set the baudrate is `BaudSet` (`0x1002`). The baud rate can be out of the standards baud rates used in UART.

#### [REQ_3030] Read baud rate

The product **must** provide a way to read the baudrate. The request code to read the baudrate is `BaudGet` (`0x1003`). The product must answer the baudrate value with the `Baud` (`0xEFFE`) code.

#### [REQ_3040] Parity

The product **must** configure the parity bit when the `SetParity` (`0x1004`) command is received. The parity bit can be set to 0 or 1 with the default value set to 0.

#### [REQ_3050] Stop bits

The product **must** configure the stop bits when the `SetStopBit` (`0x1005`) command is received. The stop bit can be set to 1 or 2 with the default value set to 1.

#### [REQ_3060] Data size

The product **must** configure the data size when the `SetDataSz` (`0x1006`) command is received. The data size can be set to 5, 6, 7, 8 or 9 with the default value set to 8.

#### [REQ_3070] Hardware flow control

The product **must** enable hardware flow control when the `HWFlowControl` (`0x1007`) command is received. The hardware flow control is disabled by default.

### SECTION 4: Features

#### [REQ_4000] UART pinout

The product UART TX is connected to GP0 and the UART RX is connected to GP1.

#### [REQ_4010] Hardware flow control pinout

The product CTS is connected to GP2 and the RTS is connected to GP3.

#### [REQ_4020] PIO

The product **must** use the Programmable I/O of the Pi Pico board.

#### [REQ_4030] Start of the program

The product **must** enable the UART communication at the start of the firmware using the default UART settings. The internal LED of the product **must** be turned on at the start of the firmware. The LED state **must** be toggled at each command received by the product during the execution of the firmware.

#### [REQ_4040] UART stop

The firmware **must** be able to stop the UART in the middle of a communication when the `StopCom` (`0x1008`) command is received.
