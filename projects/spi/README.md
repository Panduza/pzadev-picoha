# SPI master

## Requirements

### SECTION 1: USB Interface & Protocol

#### [REQ_1000] USB IDs

**<p>The product_id of the PICO must be 05e1</p>**
**<p>The vendor_id of the PICO must be 16c0</p>**
**<p>The serial_id of the PICO must be Picoha-spi-master/p>**


#### [REQ_2000] USB Protocol

**A custom protocole must be used to tranfert data by USB**



### SECTION 2: Features

#### [REQ_3000] Pin of PICO
**The following pin must be used :**

|   PIN used   |   function              |   type |
| -------      | ------------------------|--------|
|    GP6       |   SPI clock             | OUTPUT|
|    GP7       |  SPI Transfert (MOSI)   | OUTPUT | 
|    GP8       |   SPI Receive (MISO)    | Input Pull down  |
|    GP9       |  SPI Chip Select (CS)   | OUTPUT | 
|    GP25      |    Pico Internal LED    | OUTPUT |



#### [REQ_4000] Start of the program
**The internal led must be on, during the start and the excecution of the firmware**

#### [REQ_5000] End of the program
**The internal led must be turned off, when the program is stoped**

#### [REQ_6000] SPI desconnected
**The firmware must still be running when the spi connections are disconnected**

#### [REQ_7000] clock frequency
**When the firmware starts, the GPIO 6 must deliver 125KHz**

#### [REQ_8000] slave select
**When the firmware starts, the GPIO 9 must be in high level**

#### [REQ_9000] ACK response
**The GPIO 8 must be able to read a ACK frame after a initiation of SPI communication**