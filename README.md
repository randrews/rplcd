# Raspberry Pi Pico with an ili9341

![demo of it working](/demo.jpg)

This repository contains the code necessary to get `embedded-graphics`, `ili9341`,
and `rp2040-hal` all working together. As I write this, there are some mismatches in these three so
you need some glue code:

- `ili9341` expects to be sent a `DelayNs`, something which does not exist in the Pico's HAL. But then it doesn't even call the `delay_ns` method on it, so, we can wrap a normal delay and wait for msec instead (this seems to only be used during init, it doesn't affect the screen's actual performance any).
- The 2040's HAL doesn't provide any control over the SPI CS pin, even though that capability exists in hardware on the chip. So there's a small wrapper type in here to handle `SpiWithCs`.

Beyond that, this is just the rp2040 project template with some tweaks. Use the lowest `SPI0` pins (2, 3, 4, 5), or don't and change the code to match your pin assignments; I'm a readme not a cop.

--Ross Andrews, Feb 2024
