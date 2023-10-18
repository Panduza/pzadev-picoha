# Name of the interface

- Directory *src* to store the code
- Directory *examples* to store python usages

## Requirements

### SECTION 0: Pins Configuration

- **Pin 21**: I2C0 SDA
- **Pin 22**: I2C0 SCL
- **Pin 23**: GND

### SECTION 1: USB Interface & Protocol

#### [REQ_1000] USB IDs

The product **must** have the PID 0x0011. This PID is not used yet by a device with the same Vendor ID mentionned below.
The product **must** have the Vendor ID 0x1209. It's the Vendor ID assigned to open source hardware projects.
The product **must** have a serial ID 0x20000F38


#### [REQ_2000] USB Protocol

The product **must** use the HA protocol based on CDC protocol and SLIP frame format
The product **must** use the request and response code established by the protocol HA such as generic requests and the allocation memory of I2C master protocol
The request and response code **must** have a size 16 bits

### SECTION 2: Features

#### [REQ_3000] I2C slave ID

The product **must** allow the user to easily configure I2C slave address by adding it in the SLIP frame

#### [REQ_4000] Write data

The product **must** allow the user to easily write a message to I2C slave
Request code => 0x3B00
The frame **must** containe the slave address

#### [REQ_5000] Read data

The product **must** allow the user to easily read a message to I2C slave
Request code => 0x3B01
The frame **must** containe the slave address

#### [REQ_6000] I2C bus requirements

The product **must** take care of all I2C bus requirements such as START/Restart/STOP sequencing and handle the acknowledge cycles


