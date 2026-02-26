use defmt::Format;
use embedded_hal::{digital::OutputPin, spi::SpiBus};

use crate::mpu6500::reg::Register;

mod reg;

#[derive(Debug, Format)]
pub enum MpuError {
    /// For some reason, changing chip select pin state resulted in an error. How did we
    /// get here?
    ChipSelectFailed,
    SpiWriteFailed,
    SpiReadFailed,
    SpiFlushFailed,
}

/// The MPU6500 SPI driver. Because of the shared protocol, also implicitly supports
/// the MPU925x series.
pub struct MPU6500Driver<S: SpiBus, O: OutputPin> {
    spi: S,
    cs: O,
}

impl<S: SpiBus, O: OutputPin> MPU6500Driver<S, O> {
    pub fn new(spi: S, cs: O) -> Self {
        Self { spi, cs }
    }

    pub fn read_register(&mut self, reg: Register) -> Result<u8, MpuError> {
        self.cs.set_low().map_err(|_| MpuError::ChipSelectFailed)?;
        let _ = self.write(reg as u8 + 0x80)?; // 0x80 sets the RW bit, indicating that
                                               // this is a read operation
        let res = self.write(0xFF)?;
        self.cs.set_high().map_err(|_| MpuError::ChipSelectFailed)?;
        Ok(res)
    }

    pub fn write_register(&mut self, reg: Register, data: u8) -> Result<(), MpuError> {
        self.cs.set_low().map_err(|_| MpuError::ChipSelectFailed)?;
        self.write(reg as u8)?;
        self.write(data)?;
        self.cs.set_high().map_err(|_| MpuError::ChipSelectFailed)?;
        Ok(())
    }

    fn write(&mut self, data: u8) -> Result<u8, MpuError> {
        self.wait_for_idle()?;
        self.spi.write(&[data]).map_err(|_| MpuError::SpiWriteFailed)?;
        self.wait_for_idle()?;
        let mut result: [u8; 1] = [0; 1];
        self.spi.read(&mut result).map_err(|_| MpuError::SpiReadFailed)?;
        Ok(result[0])
    }

    fn wait_for_idle(&mut self) -> Result<(), MpuError> {
        self.spi.flush().map_err(|_| MpuError::SpiFlushFailed)?;
        Ok(())
    }
}
