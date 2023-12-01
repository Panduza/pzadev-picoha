
#![no_std]
#![no_main]
use rp_pico::entry;
use panic_halt as _;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;
use embedded_hal::blocking::i2c::{Operation, Read, Transactional, Write};
use fugit::RateExtU32;
use embedded_hal::digital::v2::OutputPin;
use hal::uart::{DataBits, StopBits, UartConfig};
use core::fmt::Write as wr;


#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
        // Set up the watchdog driver - needed by the clock setup code
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

        // Configure the clocks
        
        // The default is to generate a 125 MHz system clock
        let clocks = hal::clocks::init_clocks_and_plls(
            rp_pico::XOSC_CRYSTAL_FREQ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let sio = hal::Sio::new(pac.SIO);

        let pins = rp_pico::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );
   
        // i2c pins
       let sda_pin = pins.gpio20.into_function::<hal::gpio::FunctionI2C>();
       let scl_pin = pins.gpio21.into_function::<hal::gpio::FunctionI2C>();

        // i2c driver
       let mut i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        //i2c bus speed
        100.kHz(),
        //periph reset control
        &mut pac.RESETS,
        &clocks.peripheral_clock,
    );

     // Slave address
     let slave_address = 0x42;

     // Data to be sent to the slave
     let data_to_send = [0x01, 0x02, 0x03];
        // Create a delay provider
        let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
        //let mut led_pin = pins.gpio20.into_push_pull_output();

        //UART **************************************** section
        // uart pins used for debugging purposes
        let uart_pins = (
            // UART TX (characters sent from RP2040) on pin 1 (GPIO0)
            pins.gpio0.into_function(),
            // UART RX (characters received by RP2040) on pin 2 (GPIO1)
            pins.gpio1.into_function(),
        );
        let mut uart = hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
            .enable(
                UartConfig::new(9600.Hz(), DataBits::Eight, None, StopBits::One),
                clocks.peripheral_clock.freq(),
            )
            .unwrap();
        //UART **************************************** section
        let mut data_buffer = [0u8; 3];
  loop {
         let transaction = i2c.write(slave_address, &data_to_send);
         
         if let Err(error) = transaction {
            panic!("I2C write error: {:?}", error);
         }
         i2c.read(slave_address, &mut data_buffer).expect("Failed to read from I2C device"); 
      // Delay for 0.5 seconds
      delay.delay_ms(500);
  }
}