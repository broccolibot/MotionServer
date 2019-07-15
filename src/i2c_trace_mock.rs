use i2cdev::{core::I2CDevice, linux::LinuxI2CError};
use log::{trace, debug};

pub struct I2CTraceMock {
    name: String
}

impl I2CTraceMock {
    pub fn new(name: String) -> Self {
        debug!("I2C Mock initialized: {}", name);
        Self { name }
    }
}

impl I2CDevice for I2CTraceMock {
    type Error = LinuxI2CError;

    fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        trace!("{}: Read {} bytes.", self.name, data.len());
        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        trace!("{}: Wrote {} bytes; {:?}", self.name, data.len(), data);
        Ok(())
    }

    fn smbus_write_quick(&mut self, _bit: bool) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn smbus_read_block_data(&mut self, _register: u8) -> Result<Vec<u8>, Self::Error> {
        unimplemented!()
    }

    fn smbus_write_block_data(&mut self, _register: u8, _values: &[u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn smbus_process_block(
        &mut self,
        _register: u8,
        _values: &[u8],
    ) -> Result<Vec<u8>, Self::Error> {
        unimplemented!()
    }

    fn smbus_read_i2c_block_data(
        &mut self,
        _register: u8,
        _len: u8,
    ) -> Result<Vec<u8>, Self::Error> {
        unimplemented!()
    }

    fn smbus_write_i2c_block_data(
        &mut self,
        _register: u8,
        _values: &[u8],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
