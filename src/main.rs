#![no_std]
#![no_main]

mod hacks;

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    spi::Spi,
    watchdog::Watchdog,
    gpio::{Pins, FunctionSpi}
};
use display_interface_spi::SPIInterface;

use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::prelude::Size;
use embedded_graphics::primitives::{Primitive, PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::text::{Alignment, Text};
use embedded_hal::delay::DelayNs;
use fugit::RateExtU32;
use ili9341::{DisplaySize240x320, Orientation};
use crate::hacks::{MyDelay, SpiWithCS};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    let pins = Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    let sclk = pins.gpio2.into_function::<FunctionSpi>();
    let mosi = pins.gpio3.into_function::<FunctionSpi>();
    let miso = pins.gpio4.into_function::<FunctionSpi>();
    let cs = pins.gpio5.into_push_pull_output();
    let rst = pins.gpio6.into_push_pull_output();
    let dc = pins.gpio7.into_push_pull_output();

    let spi = Spi::<_, _, _, 8>::new(pac.SPI0, (mosi, miso, sclk));
    let spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16.MHz(),
        embedded_hal::spi::MODE_0
    );

    let sint = SPIInterface::new(SpiWithCS{ bus: spi, cs }, dc);
    let mut delay = MyDelay(cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz()));

    let mut ili = ili9341::Ili9341::new(sint, rst, &mut delay, Orientation::LandscapeFlipped, DisplaySize240x320).unwrap();

    let bg_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    Rectangle::new(Point::zero(), Size::new(320, 240)).into_styled(bg_style).draw(&mut ili).unwrap();

    Text::with_alignment(
        "Mem cleared\ndefaults set",
        Point::new(160, 120),
        MonoTextStyle::new(&FONT_6X10, Rgb565::GREEN),
        Alignment::Center,
    ).draw(&mut ili).unwrap();

    loop {
        delay.delay_ms(1000);
    }
}
