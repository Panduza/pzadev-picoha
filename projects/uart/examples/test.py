import serial

from .device.uart import UartDevice

ser = serial.Serial('/dev/ttyACM0')
with UartDevice(ser) as device:

    data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A]
    device.send_TX(data)
