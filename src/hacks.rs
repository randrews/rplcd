use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::{ErrorType, Operation, SpiBus};
use rp2040_hal::gpio::bank0::Gpio5;
use rp2040_hal::gpio::{FunctionSio, Pin, PullDown, SioOutput};


// The ILI crate requires that I send it a DelayNs, which the rp2040 crate doesn't provide,
// but then it never even calls the ns method on it! So this one that just wraps a normal slower
// delay should work.
pub struct MyDelay(pub cortex_m::delay::Delay);
impl DelayNs for MyDelay {
    fn delay_ns(&mut self, _ns: u32) {
        defmt::panic!("This isn't actually here")
    }

    fn delay_us(&mut self, us: u32) {
        self.0.delay_us(us)
    }

    fn delay_ms(&mut self, ms: u32) {
        self.0.delay_ms(ms)
    }
}

/////////////////////////////////////////////////////////////////////////////////////

pub struct SpiWithCS<S: SpiBus> {
    pub bus: S,
    pub cs: Pin<Gpio5, FunctionSio<SioOutput>, PullDown>
    // TODO: Delay (not actually _used_ by the ili crate but can't hurt to pass it in)
}

impl<S: SpiBus> ErrorType for SpiWithCS<S> { type Error = S::Error; }

impl<S: SpiBus> embedded_hal::spi::SpiDevice for SpiWithCS<S> {
    fn transaction(&mut self, operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        self.cs.set_low().unwrap();
        for op in operations {
            match op {
                Operation::Read(buf) => self.bus.read(buf)?,
                Operation::Write(buf) => self.bus.write(buf)?,
                Operation::Transfer(rd, wr) => self.bus.transfer(rd, wr)?,
                Operation::TransferInPlace(buf) => self.bus.transfer_in_place(buf)?,
                Operation::DelayNs(_) => () // TODO: Delay
            }
        }
        self.cs.set_high().unwrap();
        Ok(())
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().unwrap();
        self.bus.read(buf)?;
        self.cs.set_high().unwrap();
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low().unwrap();
        self.bus.write(buf)?;
        self.cs.set_high().unwrap();
        Ok(())
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.cs.set_low().unwrap();
        self.bus.transfer(read, write)?;
        self.cs.set_high().unwrap();
        Ok(())
    }

    fn transfer_in_place(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.cs.set_low().unwrap();
        self.bus.transfer_in_place(buf)?;
        self.cs.set_high().unwrap();
        Ok(())
    }
}
