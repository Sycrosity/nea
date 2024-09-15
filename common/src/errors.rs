#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum NeaError {
    I2C,
    Unknown,
    IntegerOverflow,
    InterfaceError,
}
