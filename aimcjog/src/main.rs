use libaimc::{AIMCMessage, LinuxI2CError, AIMC};
use rustyline::Editor;

type DeviceResult<T> = Result<T, LinuxI2CError>;

fn main() -> DeviceResult<()> {
    let mut args = std::env::args();
    args.next();
    let i2c_device_file = args.next().unwrap_or("/dev/i2c-0".to_string());
    println!("I2C device file: {}", i2c_device_file);

    let mut jogger = Jogger::new(i2c_device_file, None)?;
    let mut line_editor: Editor<()> = Editor::new();
    'main: loop {
        let readline = line_editor.readline(">> ");
        match readline {
            Err(_) => break 'main,
            Ok(line) => {
                if let Err(e) = jogger.next_line(&mut line.split_whitespace()) {
                    eprintln!("Error: {:?}", e);
                }
            }
        };
    }
    Ok(())
}

enum ValueSet {
    Kp(f32),
    Ki(f32),
    Kd(f32),
    Target(f32),
    None,
}

impl ValueSet {
    fn from_arg(args: &mut Iterator<Item = &str>) -> Result<Self, ()> {
        let name = args.next().ok_or(())?;
        let step: f32 = args.next().ok_or(())?.parse().map_err(|_| ())?;
        match name.as_ref() {
            "p" | "kp" => Ok(ValueSet::Kp(step)),
            "i" | "ki" => Ok(ValueSet::Ki(step)),
            "d" | "kd" => Ok(ValueSet::Kd(step)),
            "t" | "target" => Ok(ValueSet::Target(step)),
            "q" | "none" => return Ok(ValueSet::None),
            _ => Err(()),
        }
    }
}

struct Jogger {
    i2c_device_file: String,
    device: Option<AIMC>,
    jog: ValueSet,
    current_target: f32,
    current_kp: f32,
    current_ki: f32,
    current_kd: f32,
}

impl Jogger {
    pub fn new(i2c_device_file: String, address: Option<u16>) -> DeviceResult<Self> {
        Ok(Self {
            device: if let Some(address) = address {
                Some(AIMC::new(&i2c_device_file, address)?)
            } else {
                None
            },
            i2c_device_file,
            jog: ValueSet::None,
            current_target: 0.0,
            current_kp: 0.0,
            current_ki: 0.0,
            current_kd: 0.0,
        })
    }

    fn general_help() {
        println!("Commands:");
        println!("\thelp: This message");
        println!("\tdevice <Address>: Check if device is online, or set it.");
        println!("\tjog (kp|ki|kd|target) <step>: Jog the specified parameter using the specified step. Q to quit joq.");
    }

    fn warn_no_device() {
        println!("Warning: No device is currently active.");
    }

    pub fn next_line(&mut self, args: &mut Iterator<Item = &str>) -> DeviceResult<()> {
        match &args.next() {
            Some("help") => Self::general_help(),
            Some("device") => self.set_device(args)?,
            Some("disconnect") => self.device = None,
            Some("set") => match ValueSet::from_arg(args) {
                Ok(params) => self.set_param(params)?,
                Err(_) => eprintln!("Unable to parse set parameters. Syntax: Set <param> <step>"),
            },
            Some("jog") => match ValueSet::from_arg(args) {
                Ok(v) => self.jog = v,
                Err(_) => eprintln!("Unable to parse jog parameters. Syntax: Jog <param> <step>"),
            },
            None => self.jog()?,
            Some(other) => eprintln!("Unrecognized command '{}'.", other),
        }
        Ok(())
    }

    fn jog(&mut self) -> DeviceResult<()> {
        let device_message = match self.jog {
            ValueSet::Kp(step) => {
                self.current_kp += step;
                println!("Jogging kp; = {}", self.current_kp);
                AIMCMessage::SetKp(self.current_kp)
            }
            ValueSet::Ki(step) => {
                self.current_ki += step;
                println!("Jogging ki; = {}", self.current_ki);
                AIMCMessage::SetKi(self.current_ki)
            }
            ValueSet::Kd(step) => {
                self.current_kd += step;
                println!("Jogging kd; = {}", self.current_kd);
                AIMCMessage::SetKd(self.current_kd)
            }
            ValueSet::Target(step) => {
                self.current_target += step;
                println!("Jogging target; = {}", self.current_target);
                AIMCMessage::SetTarget(self.current_target)
            }
            _ => {
                println!("Warning: No jog set.");
                return Ok(());
            }
        };

        let device = match &mut self.device {
            Some(d) => d,
            None => {
                Self::warn_no_device();
                return Ok(());
            }
        };

        device.write_message(device_message)
    }

    fn set_param(&mut self, param: ValueSet) -> DeviceResult<()> {
        let device_message = match param {
            ValueSet::Kp(step) => {
                self.current_kp = step;
                println!("Setting kp; = {}", self.current_kp);
                AIMCMessage::SetKp(self.current_kp)
            }
            ValueSet::Ki(step) => {
                self.current_ki = step;
                println!("Setting ki; = {}", self.current_ki);
                AIMCMessage::SetKi(self.current_ki)
            }
            ValueSet::Kd(step) => {
                self.current_kd = step;
                println!("Setting kd; = {}", self.current_kd);
                AIMCMessage::SetKd(self.current_kd)
            }
            ValueSet::Target(step) => {
                self.current_target = step;
                println!("Setting target; = {}", self.current_target);
                AIMCMessage::SetTarget(self.current_target)
            }
            _ => {
                println!("Warning: No param set.");
                return Ok(());
            }
        };

        let device = match &mut self.device {
            Some(d) => d,
            None => {
                Self::warn_no_device();
                return Ok(());
            }
        };

        device.write_message(device_message)
    }

    fn set_device(&mut self, args: &mut Iterator<Item = &str>) -> DeviceResult<()> {
        if let Some(address) = args.next() {
            if let Ok(address) = address.parse::<u16>() {
                self.device = Some(AIMC::new(&self.i2c_device_file, address)?);
            } else {
                eprintln!("Invalid address!");
            }
        } else {
            if self.device.is_some() {
                println!("Device is connected.");
            } else {
                println!("No device connected.");
            }
        }
        Ok(())
    }
}
