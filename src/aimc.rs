// Arduino-based Intelligent Motor Controller protocol
use byteorder::{ByteOrder, LittleEndian};

// Currently, all AIMCs are little endian
type DeviceEndian = LittleEndian;

// Byte slice that make up the content of the message
const CONTENT_BYTE_SLICE: std::ops::Range<usize> = 1..5;

fn get_bytes_op(operation: u8) -> [u8; 5] {
    let mut buffer = [0u8; 5];
    buffer[0] = operation;
    buffer
}

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

pub fn enable(value: bool) -> [u8; 5] {
    get_bytes_bool(1, value)
}

pub fn set_target(value: f32) -> [u8; 5] {
    get_bytes_f32(2, value)
}

pub fn reset() -> [u8; 5] {
    get_bytes_op(3)
}

pub fn mode_pwm() -> [u8; 5] {
    get_bytes_op(4)
}

pub fn mode_position_pid() -> [u8; 5] {
    get_bytes_op(5)
}

pub fn mode_velocity_pid() -> [u8; 5] {
    get_bytes_op(6)
}

pub fn set_kp(value: f32) -> [u8; 5] {
    get_bytes_f32(7, value)
}

pub fn set_ki(value: f32) -> [u8; 5] {
    get_bytes_f32(8, value)
}

pub fn set_kd(value: f32) -> [u8; 5] {
    get_bytes_f32(9, value)
}

pub fn home(value: i32) -> [u8; 5] {
    get_bytes_i32(10, value)
}

pub fn limit_pwm(value: u8) -> [u8; 5] {
    get_bytes_u32(11, u32::from(value))
}

pub fn limit_target_min(value: f32) -> [u8; 5] {
    get_bytes_f32(12, value)
}

pub fn limit_target_max(value: f32) -> [u8; 5] {
    get_bytes_f32(13, value)
}

pub fn encoder_polarity(value: bool) -> [u8; 5] {
    get_bytes_u32(14, u32::from(value))
}
