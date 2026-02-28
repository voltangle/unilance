//! MPU6500 SPI driver based on embedded-hal. Some code like the register enum was taken
//! from https://github.com/justdimaa/embedded-sensors/tree/main/src/mpu6500, which is
//! licensed as Apache 2.0 and MIT.
extern crate nalgebra as na;
use defmt::Format;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiBus;
use int_enum::IntEnum;
pub use na::Vector3;

const DEG_TO_RAD: f32 = 0.01745329252;
const STD_GRAVITY: f32 = 9.80665;

#[derive(Debug, Format)]
pub enum MpuError {
    /// For some reason, changing chip select pin state resulted in an error. How did we
    /// get here?
    ChipSelectFailed,
    SpiWriteFailed,
    SpiReadFailed,
    SpiFlushFailed,
    /// If the device responded to a whoami request with an ID that is unknown to the
    /// driver
    UnknownDeviceIdentifier,
    /// If the divider value in [MPU6500Driver::set_sample_rate_divider] is set as 0
    ZeroDividerNumber,
}

/// The MPU6500 SPI driver. Because of the shared protocol, also implicitly supports
/// the MPU925x series.
pub struct MPU6500Driver<S: SpiBus, O: OutputPin> {
    pub spi: S,
    cs: O,
    gyro_range: GyroRange,
    gyro_bias: Vector3<f32>,
    accel_range: AccelRange,
    accel_bias: Vector3<f32>,
    accel_scale: Vector3<f32>,
}

// Starting functions
impl<S: SpiBus, O: OutputPin> MPU6500Driver<S, O> {
    pub fn new(spi: S, cs: O) -> Self {
        Self {
            spi,
            cs,
            gyro_range: GyroRange::Dps250,
            gyro_bias: Vector3::new(0.0, 0.0, 0.0),
            accel_range: AccelRange::Range2G,
            accel_bias: Vector3::new(0.0, 0.0, 0.0),
            accel_scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    /// Do initialisation of the IMU. Ideally, you should run it right after creating the
    /// driver instance. You should run [MPU6500Driver::reset] with a delay of ~100 ms
    /// before running init.
    pub fn init(&mut self) -> Result<(), MpuError> {
        self.reset_signal_path_all()?;
        self.set_register(Register::PWR_MGMT_1, 0b1000000, 0 << 6)?;
        self.set_clock_source(Some(ClockSource::Autoselect))?;
        self.set_spi_mode_only(true)?;
        Ok(())
    }
}

// High level safe API over raw register reads
impl<S: SpiBus, O: OutputPin> MPU6500Driver<S, O> {
    pub fn whoami(&mut self) -> Result<DeviceModel, MpuError> {
        let val = self.read_register(Register::WHO_AM_I)?;
        Ok(DeviceModel::try_from(val).map_err(|_| MpuError::UnknownDeviceIdentifier)?)
    }

    /// Resets the internal registers and restores the default settings.
    pub fn reset(&mut self) -> Result<(), MpuError> {
        self.set_register(Register::PWR_MGMT_1, 0b10000000, 0x80)
    }

    /// Resets the gyro, accel, and temp digital signal paths.
    pub fn reset_signal_path_all(&mut self) -> Result<(), MpuError> {
        self.write_register(Register::SIGNAL_PATH_RESET, 0x7)
    }

    /// Sets the clock source. If [src] is set to None, will stop the clock and keep the
    /// timing generator in reset.
    pub fn set_clock_source(&mut self, src: Option<ClockSource>) -> Result<(), MpuError> {
        match src {
            Some(src) => self.set_register(Register::PWR_MGMT_1, 0b111, src as u8)?,
            // Stops the clock and keeps timing generator in reset
            None => self.set_register(Register::PWR_MGMT_1, 0b111, 0x7)?,
        }
        Ok(())
    }

    /// If yes, reset I2C Slave module and put the serial interface in SPI mode only.
    pub fn set_spi_mode_only(&mut self, yes: bool) -> Result<(), MpuError> {
        self.set_register(Register::USER_CTRL, 0b10000, (yes as u8) << 4)
    }

    /// Divides the internal sample rate (see register CONFIG) to generate the sample
    /// rate that controls sensor data output rate, FIFO sample rate.
    /// This register is only effective when FCHOICE = 2’b11 (FCHOICE_B register bits are
    /// 2’b00), and (0 < DLPF_CFG < 7)
    ///
    /// This is the update rate of sensor register.
    /// Sample rate = INTERNAL_SAMPLE_RATE / ([div])
    /// where INTERNAL_SAMPLE_RATE = 1kHz.
    ///
    /// The divider number cannot be zero.
    // TODO: maybe rewrite this guy as a "set sample rate" function with predefined rates
    // in an enum, make it more portable?
    pub fn set_sample_rate_divider(&mut self, div: u8) -> Result<(), MpuError> {
        if div == 0 {
            return Err(MpuError::ZeroDividerNumber);
        }
        self.write_register(Register::SMPLRT_DIV, div - 1)
    }

    /// Set gyroscope range. Check out [GyroRange] for all available values.
    pub fn set_gyro_range(&mut self, range: GyroRange) -> Result<(), MpuError> {
        self.gyro_range = range;
        self.set_register(Register::GYRO_CONFIG, 0b11000, (range as u8) << 3)
    }

    /// Set accelerometer range. Check out [AccelRange] for all available values.
    pub fn set_accel_range(&mut self, range: AccelRange) -> Result<(), MpuError> {
        self.accel_range = range;
        self.set_register(Register::ACCEL_CONFIG, 0b11000, (range as u8) << 3)
    }

    /// Read current IMU data, without doing any processing on it. Values from the IMU
    /// are passed as-is, directly from the chip.
    pub fn get_raw_measurements(&mut self) -> Result<RawMeasurements, MpuError> {
        let mut buf: [u8; 14] = [0; 14];

        self.read_multi(Register::ACCEL_XOUT_H, &mut buf)?;

        Ok(RawMeasurements {
            accel_x: i16::from_be_bytes([buf[0], buf[1]]),
            accel_y: i16::from_be_bytes([buf[2], buf[3]]),
            accel_z: i16::from_be_bytes([buf[4], buf[5]]),
            gyro_x: i16::from_be_bytes([buf[8], buf[9]]),
            gyro_y: i16::from_be_bytes([buf[10], buf[11]]),
            gyro_z: i16::from_be_bytes([buf[12], buf[13]]),
            temp: i16::from_be_bytes([buf[6], buf[7]]),
        })
    }
}

// Higher level helper functions
impl<S: SpiBus, O: OutputPin> MPU6500Driver<S, O> {
    /// Fetch current measurements, applying calibration offsets and scaling.
    ///
    /// Output units of measurement are Rad/s for gyro, m/s^2 for accelerometer, and
    /// Celcius for temperature.
    pub fn get_measurements(&mut self) -> Result<Measurements, MpuError> {
        let raw = self.get_raw_measurements()?;
        Ok(Measurements {
            accel: (Vector3::new(
                raw.accel_x as f32 * self.accel_range.conv_multiplier() * STD_GRAVITY,
                raw.accel_y as f32 * self.accel_range.conv_multiplier() * STD_GRAVITY,
                raw.accel_z as f32 * self.accel_range.conv_multiplier() * STD_GRAVITY,
            ) - self.accel_bias)
                .component_mul(&self.accel_scale),
            gyro: Vector3::new(
                raw.gyro_x as f32 * self.gyro_range.conv_multiplier() * DEG_TO_RAD,
                raw.gyro_y as f32 * self.gyro_range.conv_multiplier() * DEG_TO_RAD,
                raw.gyro_z as f32 * self.gyro_range.conv_multiplier() * DEG_TO_RAD,
            ) - self.gyro_bias,
            temp: raw.temp as f32 / 333.87 + 21.0,
        })
    }

    /// Fetch current measurements without applying calibration offsets and scaling.
    ///
    /// Output units of measurement are Rad/s for gyro, m/s^2 for accelerometer, and
    /// Celcius for temperature.
    pub fn get_measurements_uncalibrated(&mut self) -> Result<Measurements, MpuError> {
        let raw = self.get_raw_measurements()?;
        Ok(Measurements {
            accel: Vector3::new(
                raw.accel_x as f32 * self.accel_range.conv_multiplier() * STD_GRAVITY,
                raw.accel_y as f32 * self.accel_range.conv_multiplier() * STD_GRAVITY,
                raw.accel_z as f32 * self.accel_range.conv_multiplier() * STD_GRAVITY,
            ),
            gyro: Vector3::new(
                raw.gyro_x as f32 * self.gyro_range.conv_multiplier() * DEG_TO_RAD,
                raw.gyro_y as f32 * self.gyro_range.conv_multiplier() * DEG_TO_RAD,
                raw.gyro_z as f32 * self.gyro_range.conv_multiplier() * DEG_TO_RAD,
            ),
            temp: raw.temp as f32 / 333.87 + 21.0,
        })
    }

    /// Saves the offsets and applies them on each [MPU6500Driver::get_measurement] call.
    ///
    /// Accel and gyro units are m/s^2 and Rad/s, same as in the previously mentioned
    /// function.
    pub fn set_biases(&mut self, accel: Vector3<f32>, gyro: Vector3<f32>) {
        self.accel_bias = accel;
        self.gyro_bias = gyro;
    }

    /// Saves the accelerometer scaling values. They are then applied in
    /// [MPU6500Driver::get_measurement] calls.
    ///
    /// Units are m/s^2, same as in the previously mentioned function.
    pub fn set_accel_scale(&mut self, scale: Vector3<f32>) {
        self.accel_scale = scale;
    }
}

// Lower level register access
impl<S: SpiBus, O: OutputPin> MPU6500Driver<S, O> {
    pub fn set_register(
        &mut self,
        reg: Register,
        mask: u8,
        value: u8,
    ) -> Result<(), MpuError> {
        let current = self.read_register(reg)?;
        self.write_register(reg, (current & !mask) | value & mask)
    }

    /// Read multiple registers sequentially.
    ///
    /// For example, if you pass in [Register::ACCEL_XOUT_H] and size the buffer to have
    /// 14 bytes of capacity, it will read 14 registers starting with the address of the
    /// register passed in, and then effectively incrementing the register address with
    /// each subsequent read.
    pub fn read_multi<const SIZE: usize>(
        &mut self,
        reg: Register,
        buf: &mut [u8; SIZE],
    ) -> Result<(), MpuError> {
        self.start_operation()?;
        _ = self.write(reg as u8 | 0x80)?;
        let write_buf: [u8; SIZE] = [0xFF; SIZE];
        self.spi.transfer(buf, &write_buf).map_err(|_| MpuError::SpiWriteFailed)?;
        self.end_operation()?;
        Ok(())
    }

    pub fn read_register(&mut self, reg: Register) -> Result<u8, MpuError> {
        self.start_operation()?;
        // 0x80 sets the RW bit, indicating that this is a read operation
        let _ = self.write(reg as u8 + 0x80)?;
        let res = self.write(0xFF)?;
        self.end_operation()?;
        Ok(res)
    }

    pub fn write_register(&mut self, reg: Register, data: u8) -> Result<(), MpuError> {
        self.start_operation()?;
        self.write(reg as u8)?;
        self.write(data)?;
        self.end_operation()?;
        Ok(())
    }

    fn write(&mut self, data: u8) -> Result<u8, MpuError> {
        let mut buf = [data];
        self.spi
            .transfer_in_place(&mut buf)
            .map_err(|_| MpuError::SpiWriteFailed)?;
        self.wait_for_idle()?;
        Ok(buf[0])
    }

    fn start_operation(&mut self) -> Result<(), MpuError> {
        self.cs.set_low().map_err(|_| MpuError::ChipSelectFailed)?;
        Ok(())
    }

    fn end_operation(&mut self) -> Result<(), MpuError> {
        self.cs.set_high().map_err(|_| MpuError::ChipSelectFailed)?;
        Ok(())
    }

    fn wait_for_idle(&mut self) -> Result<(), MpuError> {
        self.spi.flush().map_err(|_| MpuError::SpiFlushFailed)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Format, Default)]
pub struct Measurements {
    // m/s^2
    pub accel: Vector3<f32>,
    // Rads/s
    pub gyro: Vector3<f32>,
    // Celcius
    pub temp: f32,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Format, IntEnum)]
pub enum GyroRange {
    Dps250 = 0b00,
    Dps500 = 0b01,
    Dps1000 = 0b10,
    Dps2000 = 0b11,
}

impl GyroRange {
    /// Conversion multiplier for converting from raw counts to Rad/s.
    pub fn conv_multiplier(&self) -> f32 {
        match self {
            GyroRange::Dps250 => 250.0 / 32768.0,
            GyroRange::Dps500 => 500.0 / 32768.0,
            GyroRange::Dps1000 => 1000.0 / 32768.0,
            GyroRange::Dps2000 => 2000.0 / 32768.0,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Format, IntEnum)]
pub enum AccelRange {
    Range2G,
    Range4G,
    Range8G,
    Range16G,
}

impl AccelRange {
    /// Conversion multiplier for converting from raw counts to m/s^2.
    pub fn conv_multiplier(&self) -> f32 {
        match self {
            AccelRange::Range2G => 2.0 / 32768.0,
            AccelRange::Range4G => 4.0 / 32768.0,
            AccelRange::Range8G => 8.0 / 32768.0,
            AccelRange::Range16G => 16.0 / 32768.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Format, Default)]
pub struct RawMeasurements {
    pub accel_x: i16,
    pub accel_y: i16,
    pub accel_z: i16,
    pub gyro_x: i16,
    pub gyro_y: i16,
    pub gyro_z: i16,
    pub temp: i16,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Format, IntEnum)]
pub enum ClockSource {
    /// Internal 20MHz oscillator
    Internal = 0x1,
    /// Auto selects the best available clock source – PLL if ready, else use the
    /// Internal oscillator.
    Autoselect = 0x3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Format, IntEnum)]
pub enum DeviceModel {
    MPU6500 = 0x70,
    MPU9250 = 0x71,
    MPU9255 = 0x73,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Format)]
pub enum Register {
    SELF_TEST_X_GYRO = 0x00,
    SELF_TEST_Y_GYRO = 0x01,
    SELF_TEST_Z_GYRO = 0x02,
    SELF_TEST_X_ACCEL = 0x0D,
    SELF_TEST_Y_ACCEL = 0x0E,
    SELF_TEST_Z_ACCEL = 0x0F,
    XG_OFFSET_H = 0x13,
    XG_OFFSET_L = 0x14,
    YG_OFFSET_H = 0x15,
    YG_OFFSET_L = 0x16,
    ZG_OFFSET_H = 0x17,
    ZG_OFFSET_L = 0x18,
    SMPLRT_DIV = 0x19,
    CONFIG = 0x1A,
    GYRO_CONFIG = 0x1B,
    ACCEL_CONFIG = 0x1C,
    ACCEL_CONFIG_2 = 0x1D,
    LP_ACCEL_ODR = 0x1E,
    WOM_THR = 0x1F,
    FIFO_EN = 0x23,
    I2C_MST_CTRL = 0x24,
    I2C_SLV0_ADDR = 0x25,
    I2C_SLV0_REG = 0x26,
    I2C_SLV0_CTRL = 0x27,
    I2C_SLV1_ADDR = 0x28,
    I2C_SLV1_REG = 0x29,
    I2C_SLV1_CTRL = 0x2A,
    I2C_SLV2_ADDR = 0x2B,
    I2C_SLV2_REG = 0x2C,
    I2C_SLV2_CTRL = 0x2D,
    I2C_SLV3_ADDR = 0x2E,
    I2C_SLV3_REG = 0x2F,
    I2C_SLV3_CTRL = 0x30,
    I2C_SLV4_ADDR = 0x31,
    I2C_SLV4_REG = 0x32,
    I2C_SLV4_DO = 0x33,
    I2C_SLV4_CTRL = 0x34,
    I2C_SLV4_DI = 0x35,
    I2C_MST_STATUS = 0x36,
    INT_PIN_CFG = 0x37,
    INT_ENABLE = 0x38,
    INT_STATUS = 0x3A,
    ACCEL_XOUT_H = 0x3B,
    ACCEL_XOUT_L = 0x3C,
    ACCEL_YOUT_H = 0x3D,
    ACCEL_YOUT_L = 0x3E,
    ACCEL_ZOUT_H = 0x3F,
    ACCEL_ZOUT_L = 0x40,
    TEMP_OUT_H = 0x41,
    TEMP_OUT_L = 0x42,
    GYRO_XOUT_H = 0x43,
    GYRO_XOUT_L = 0x44,
    GYRO_YOUT_H = 0x45,
    GYRO_YOUT_L = 0x46,
    GYRO_ZOUT_H = 0x47,
    GYRO_ZOUT_L = 0x48,
    EXT_SENS_DATA_00 = 0x49,
    EXT_SENS_DATA_01 = 0x4A,
    EXT_SENS_DATA_02 = 0x4B,
    EXT_SENS_DATA_03 = 0x4C,
    EXT_SENS_DATA_04 = 0x4D,
    EXT_SENS_DATA_05 = 0x4E,
    EXT_SENS_DATA_06 = 0x4F,
    EXT_SENS_DATA_07 = 0x50,
    EXT_SENS_DATA_08 = 0x51,
    EXT_SENS_DATA_09 = 0x52,
    EXT_SENS_DATA_10 = 0x53,
    EXT_SENS_DATA_11 = 0x54,
    EXT_SENS_DATA_12 = 0x55,
    EXT_SENS_DATA_13 = 0x56,
    EXT_SENS_DATA_14 = 0x57,
    EXT_SENS_DATA_15 = 0x58,
    EXT_SENS_DATA_16 = 0x59,
    EXT_SENS_DATA_17 = 0x5A,
    EXT_SENS_DATA_18 = 0x5B,
    EXT_SENS_DATA_19 = 0x5C,
    EXT_SENS_DATA_20 = 0x5D,
    EXT_SENS_DATA_21 = 0x5E,
    EXT_SENS_DATA_22 = 0x5F,
    EXT_SENS_DATA_23 = 0x60,
    I2C_SLV0_DO = 0x63,
    I2C_SLV1_DO = 0x64,
    I2C_SLV2_DO = 0x65,
    I2C_SLV3_DO = 0x66,
    I2C_MST_DELAY_CTRL = 0x67,
    SIGNAL_PATH_RESET = 0x68,
    MOT_DETECT_CTRL = 0x69,
    USER_CTRL = 0x6A,
    PWR_MGMT_1 = 0x6B,
    PWR_MGMT_2 = 0x6C,
    FIFO_COUNTH = 0x72,
    FIFO_COUNTL = 0x73,
    FIFO_R_W = 0x74,
    WHO_AM_I = 0x75,
    XA_OFFSET_H = 0x77,
    XA_OFFSET_L = 0x78,
    YA_OFFSET_H = 0x7A,
    YA_OFFSET_L = 0x7B,
    ZA_OFFSET_H = 0x7D,
    ZA_OFFSET_L = 0x7E,
}
