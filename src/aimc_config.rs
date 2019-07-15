enum AIMCMode {
    PID(f32, f32, f32),
    Pneumatic,
    PWM,
}

pub struct AIMCConfig {
    pub address: u16,
    pub pid: Option<(f32, f32, f32)>,
}
