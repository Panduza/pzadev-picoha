"""
=============================================
Helper class to handle GPIO interface devices
=============================================

:Authors: - Florian Dupeyron <florian.dupeyron@mugcat.fr>
:Date: March 2023
"""

from .          import DeviceWrapper
from ..protocol import uart, common


class UartDevice(DeviceWrapper):
    def __init__(self, transport):
        super().__init__(transport)

    # def message_callback(self, msg):
    #     # TODO # Filter GpioFront messages
    #     return msg

    ############

    def send_TX(self, tx_data: []):
        self.request(uart.UartRequestDataTX(tx_data), common.StatusGood)

