# SPI master

## Requirements

### SECTION 1: USB Interface & Protocol

#### [REQ_1000] USB IDs

**<p>The product_id of the PICO must be 05e1</p>**
**<p>The vendor_id of the PICO must be 16c0</p>**
**<p>The serial_id of the PICO must be Picoha-spi-master/p>**


#### [REQ_1000] USB Protocol

**The MODBUS protocple will be used to communicate threw USB**

#### [REQ_1000] MODBUS protocol

**the RTU method of MODBUS protocole must be used**

### SECTION 2: Features

#### [REQ_2000] Pin of PICO
**The following pin must be used :**
    <p>- GP6 : SPI clock</p>
    <p>- GP7 : SPI transfert</p>
    <p>- GP8 : SPI receive</p>
    <p>- GP9 : SPI chip select (CS)</p>
    <p>- GP25 : Internal LED</p> 

#### [REQ_3000] Start of the program
**The internal led must be on, during the start and the excecution of the firmware**

#### [REQ_4000] End of the program
**The internal led must be turned off, when the program is stoped**

#### [REQ_5000] SPI desconnected
**The firmware must still be running when the spi connections are disconnected**

#### [REQ_6000] clock frequency
**When the firmware starts, the GPIO 6 must deliver 125KHz**

#### [REQ_7000] slave select
**When the firmware starts, the GPIO 9 must be in high level**

#### [REQ_8000] ACK response
**The GPIO 8 must be able to read a ACK frame after a initiation of SPI communication**