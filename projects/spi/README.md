# SPI master

## Requirements


### SECTION 1 : Target and protocol

#### [REQ_1000] Microcontroller

To develop the SPI host adapter, the firmware must be implemented in the Raspberry Pi Pico

![](img/pinout.png)

#### [REQ_2000] SPI protocol

To communicate threw SPI, the following transaction must be followed:  

![](img/spi.png)


### SECTION 2 : USB Interface & Protocol

#### [REQ_3000] USB IDs

**<p>The product_id of the PICO must be 05e1</p>**
**<p>The vendor_id of the PICO must be 16c0</p>**
**<p>The serial_short of the PICO must be Picoha-spi-master/p>**


#### [REQ_4000] USB Protocol

**A custom protocol must be used to tranfert data by USB. The custom protocol is based on the serial line internet protocol**



### SECTION 2: Features

#### [REQ_5000] Pin of PICO
**The following pin must be used :**

|   PIN used   |   function              |   type | Idle state |
| -------      | ------------------------|--------|------------|          
|    GP6       |   SPI clock             | OUTPUT |   HIGH     | 
|    GP7       |  SPI Transfert (MOSI)   | OUTPUT |   HIGH     | 
|    GP8       |   SPI Receive (MISO)    | Input  |   HIGH     |   
|    GP9       |  SPI Chip Select (CS)   | OUTPUT |   HIGH     | 
|    GP25      |    Pico Internal LED    | OUTPUT |   LOW     | 


#### [REQ_6000] SPI polarity mode
**The polarity mode will be the mode 3**

All pins of SPI must be at high during idle state (Cf REQ_5000)

![](img/spi_mode_3.png)

#### [REQ_7000] Start of the program
**The internal led must be on, during the start and the execution of the firmware**

#### [REQ_8000] End of the program
**The internal led must be turned off, when the program is stoped**

#### [REQ_9000] SPI desconnected
**The firmware must still be running when the SPI connections are disconnected**

#### [REQ_10000] clock frequency
**When the firmware starts, the GPIO 6 must deliver a 125KHz clock frequency**

#### [REQ_20000] slave select
**When the firmware starts, the GPIO 9 must be in high level**

#### [REQ_30000] request to slave
**When the initiates the communication, it must send the Read byte followed by the slave address**

#### [REQ_40000] ACK response
**The GPIO 8 must be able to read a ACK frame after the initiation of SPI communication**

#### [REQ_50000] unused pins
**The unused pins of the PICO must be configured as inputs**