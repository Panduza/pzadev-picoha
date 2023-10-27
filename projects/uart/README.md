# UART

- Directory *src* to store the code
- Directory *examples* to store python usages

## Requirements

### SECTION 1: USB Interface & Protocol

#### [REQ_1000] USB IDs

The product USB IDs **must** be free shared USB VID/PID pair for CDC devices. The products can be differentiated with their manufacturer and product identification and their serial number.

The vendor ID of the device **must** be 0x16C0.

The product ID of the device **must** be 0x05E1.

The manufacturer identification of the device **must** be `"panduza.io"`.

The product identification of the device **must** be `"picoha-uart"`.

The serial number of the device **must** be XXXX.

We chose to use the free shared USB VID/PID instead of the Raspberry Pi VID/PID to assure not to have the product been mistaken with another Raspberry Pi product.

#### [REQ_1010] USB device class

The product **must** use USB CDC as its device class.

#### [REQ_1010] USB protocol

The product **must** use Serial Line Internet Protocol (SLIP) as its communication protocol. It is composed of a data payload and a flag that acts as an end delimiter. If this flag is present in the data, then an escape sequence precedes it, so that the receiver does not consider it as the end of the frame.

<img src="img/slip.jpg" alt="Serial Line Internet Protocol" title="DSerial Line Internet Protocol">

SLIP flags:

| Hex value   | Abbreviation  | Description             |
| ----------- | ------------- | ----------------------  |
| `0xC0`      | END           | Frame End               |
| `0xDB`      | ESC           | Frame Escape            |
| `0xDC`      | ESC_END       | Transposed Frame End    |
| `0xDD`      | ESC_ESC       | Transposed Frame Escape |

#### [REQ_1010] USB custom frame

The product **must** use a custom frame protocol. The frames are composed of:

<img src="img/data_frame.jpg" alt="Data frame" title="Data frame">

- a 16 bits request code
- Data
- a 16 bits CRC

#### [REQ_1010] Custom frame CRC

The CRC used in USB custom frame **must** be `crc-ccit-false`. It can be implemented in two different ways. By using the predefined:
```
crc16 = crcmod.predefined.mkCrcFun("crc-ccitt-false")
```
Or by using the crcmod method:
```
crc16 = crcmod.mkCrcFun(0x11021, rev=False, initCrc=0xFFFF, xorOut=0x0000)
```

#### [REQ_1010] Custom frame transfer machanism

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

The product **must** answer when the ping request `0x0000` is received with the good answer `0xFFFF`.

#### [REQ_2000] Interface type

The product **must** answer when the ItfType request `0x0001` is received with the ItfTypeResp answer `0xFEFE`. The data returned must be `"picoha-uart"`.

#### [REQ_2000] Version

The product **must** answer when the Version request `0x0002` is received with the VersionResp answer `0xFEFF`. The data returned must be version of the firmware loaded on the product.

#### [REQ_2000] ID

The product **must** answer when the IdGet request `0x0003` is received with the IdResp answer `0xFEFD`. The data returned must be unique board ID of the product.

#### [REQ_2000] Good

The product **must** answer the code `0xFFFF` when no error has been encountered.

#### [REQ_2000] Generic error

The product **must** answer the code `0xFFFE` when a generic error is encountered.

#### [REQ_2000] CRC error

The product **must** answer the code `0xFFFD` when a CRC error is encountered.

#### [REQ_2000] Unknown code error

The product **must** answer the code `0xFFFC` when an unknown code error is encountered.

#### [REQ_2000] Invalid arguments error

The product **must** answer the code `0xFFFB` when an invalid arguments error is encountered.

#### [REQ_2000] Busy error

The product **must** answer the code `0xFFFA` when a busy error is encountered.

### SECTION 3: UART Requests

***UART requests***

| Code        | Function  |
| ----------- | --------- |
| `0x1000`    | UartBegin |
| `0x1001`    | UartEscape|
| `0x1002`    | DataTX    |
| `0x1003`    | DataRXGet |
| `0x1004`    | BaudSet   |
| `0x1005`    | BaudGet   |
| `0x1006`    | SetParity |
| `0x1007`    | SetStopBit|
| `0x1008`    | SetDataSz |
| `0x1009`    | StopCom   |

***UART answers***

| Code        | Function      |
| ----------- | ------------- |
| `0xEFFF`    | DataRX        |
| `0xEFFE`    | Baud          |

#### [REQ_2000] Enable UART

The product **must** enable the UART communication when the command `0x1000` is received.

#### [REQ_2010] Disable UART

The product **must** disable the UART communication when the command `0x1001` is received.

#### [REQ_2020] Send data

The product **must** send data to one other product using UART communication when the command `0x1002` is received.

#### [REQ_2030] Receive data

The product **must** receive data from one other product using UART communication when the command `0x1003` is received.

#### [REQ_2040] Baud rate

The product **must** provide a way to change and read the baudrate. The request code to set the baudrate is `0x1004`, the request code to read the baudrate is `0x1005`. The baud rate can be out of the standards baud rates used in UART.

#### [REQ_2050] Parity

The product **must** configure the parity bit when the command `0x1006` is received. The parity bit can be set to 0 or 1 with the default value set to 0.

#### [REQ_2060] Stop bits

The product **must** configure the stop bits when the command `0x1007` is received. The stop bit can be set to 1 or 2 with the default value set to 1.

#### [REQ_2070] Data size

The product **must** configure the data size when the command `0x1008` is received. The data size can be set to 5, 6, 7, 8 or 9 with the default value set to 8.

#### [REQ_2080] UART pinout

The product UART TX is connected to GP0 and the UART RX is connected to GP1.

### SECTION 4: Features

#### [REQ_2090] PIO

The product **must** use the Programmable I/O of the Pi Pico board.

#### [REQ_2100] Start of the program

The internal LED of the product **must** be turned on at the start of the firmware. The LED state **must** be inverted at each command received by the product during the execution of the firmware.

#### [REQ_2120] UART stop

The firmware **must** be able to stop the UART in the middle of a communication when the command `0x1009` is received.