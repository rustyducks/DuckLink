# Architecture

## Transport

A transport will put message into bytes and transmit it, or take bytes, check integrity, and transform it back to message.
In most cases, it will needs a device to put bytes in and take bytes from. (The ivy transport will be an exception).

It will be up to the transport to implement connected mode if needed.

The interfaces of a connected transport and a not connected transport could be different.


##  Device

A device is a wrapper around something that can send and receive bytes : serial port, I2C, SPI, and even things like UDP or TCP.


