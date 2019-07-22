use byteorder::{ByteOrder, LittleEndian};

// Currently, all AIMCs are little endian
type DeviceEndian = LittleEndian;

// Byte slice that make up the content of the message
const CONTENT_BYTE_SLICE: std::ops::Range<usize> = 1..5;

/// Create a buffer and only set the opcode byte
fn get_bytes_op(operation: u8) -> [u8; 5] {
    let mut buffer = [0u8; 5];
    buffer[0] = operation;
    buffer
}

// Message translation functions
fn get_bytes_u32(operation: u8, data: u32) -> [u8; 5] {
    let mut buffer = get_bytes_op(operation);
    DeviceEndian::write_u32(&mut buffer[CONTENT_BYTE_SLICE], data);
    buffer
}

fn get_bytes_i32(operation: u8, data: i32) -> [u8; 5] {
    let mut buffer = get_bytes_op(operation);
    DeviceEndian::write_i32(&mut buffer[CONTENT_BYTE_SLICE], data);
    buffer
}

fn get_bytes_f32(operation: u8, data: f32) -> [u8; 5] {
    let mut buffer = get_bytes_op(operation);
    DeviceEndian::write_f32(&mut buffer[CONTENT_BYTE_SLICE], data);
    buffer
}

fn get_bytes_bool(operation: u8, data: bool) -> [u8; 5] {
    get_bytes_u32(operation, if data { 1 } else { 0 })
}

/// Single communication from host to device
#[derive(Clone, Copy)]
pub enum AIMCMessage {
    Enable(bool),
    SetTarget(f32),
    Reset,
    ModePWM,
    ModePID,
    ModePneumatic,
    SetKp(f32),
    SetKi(f32),
    SetKd(f32),
    Home(i32),
    LimitPwm(u8),
    LimitTargetMin(f32),
    LimitTargetMax(f32),
    EncoderPolarity(bool),
}

impl AIMCMessage {
    /// Convert this message into device-dependent bytes.
    /// Note that values start at 1 as 0 may signify an error.
    pub fn into_bytes(&self) -> [u8; 5] {
        match *self {
            AIMCMessage::Enable(value) => get_bytes_bool(1, value),
            AIMCMessage::SetTarget(value) => get_bytes_f32(2, value),
            AIMCMessage::Reset => get_bytes_op(3),
            AIMCMessage::ModePWM => get_bytes_op(4),
            AIMCMessage::ModePID => get_bytes_op(5),
            AIMCMessage::ModePneumatic => get_bytes_op(6),
            AIMCMessage::SetKp(value) => get_bytes_f32(7, value),
            AIMCMessage::SetKi(value) => get_bytes_f32(8, value),
            AIMCMessage::SetKd(value) => get_bytes_f32(9, value),
            AIMCMessage::Home(value) => get_bytes_i32(10, value),
            AIMCMessage::LimitPwm(value) => get_bytes_u32(11, u32::from(value)),
            AIMCMessage::LimitTargetMin(value) => get_bytes_f32(12, value),
            AIMCMessage::LimitTargetMax(value) => get_bytes_f32(13, value),
            AIMCMessage::EncoderPolarity(value) => get_bytes_u32(14, u32::from(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enable() {
        assert_eq!(AIMCMessage::Enable(false).into_bytes(), [1, 0, 0, 0, 0]);
        assert_eq!(AIMCMessage::Enable(true).into_bytes(), [1, 1, 0, 0, 0]);
    }
}
