use libaimc::{AIMCMessage, AIMC};
use rustyline::Editor;
mod parser;
use parser::{Action, HELP_LINES};

const DEFAULT_DEVICE_FILE: &str = "/dev/i2c-0";

fn main() {
    let mut args = std::env::args();
    args.next();
    let device_address = match args.next() {
        Some(addr) => match u16::from_str_radix(&addr, 16) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("Failed to parse address. Did you add an '0x' by accident?");
                return;
            }
        },
        None => {
            eprintln!("Must specify device address.");
            return;
        }
    };

    let i2c_device_file = args.next().unwrap_or(DEFAULT_DEVICE_FILE.to_string());
    let mut device = match AIMC::new(&i2c_device_file, device_address) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Could not connect to device; {:?}", e);
            return;
        }
    };

    eprintln!("Using I2C device file: {}", i2c_device_file);

    let mut line_editor: Editor<()> = Editor::new();

    'main: loop {
        match line_editor.readline(">> ") {
            Err(_) => {
                let _ = device.write_message(AIMCMessage::Enable(false));
                break 'main;
            }
            Ok(line) => {
                line_editor.add_history_entry(&line);
                let mut split = line.split_whitespace();
                match Action::from_commandline(&mut split) {
                    Err(e) => eprintln!("Error parsing line: {}", e),
                    Ok(Action::Write(msg)) => {
                        println!("Writing message: {:?}", msg);
                        println!("Response: {:?}", device.write_message(msg));
                    }
                    Ok(Action::Read) => println!("Device status: {:#?}", device.status()),
                    Ok(Action::Help) => {
                        for line in HELP_LINES {
                            println!("{}", line);
                        }
                    }
                }
            }
        }
    }
}
