use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};
use server::dispatcher::*;
use server::generic_message::*;
use std::{
    fs::File,
    io::{ErrorKind, Write},
    net,
    path::Path,
};

const DEFAULT_CONFIG_DIR: &str = "server.yml";
const MESSAGE_BUFFER_SIZE: usize = 4096;

#[derive(Serialize, Deserialize)]
struct ServerConfig {
    pub socket_address: net::SocketAddr,
    #[serde(flatten)]
    pub dispatcher_config: DispatcherConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            socket_address: "127.0.0.1:5060".parse().unwrap(),
            dispatcher_config: Default::default(),
        }
    }
}

fn write_default_config<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let file = File::create(path)?;
    serde_yaml::to_writer(&file, &ServerConfig::default()).unwrap();
    Ok(())
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Trace)
        .format(|buf, record| writeln!(buf, "{:5}| {}", record.level(), record.args()))
        .init();

    let mut args = std::env::args();
    args.next();

    let config_dir = &args
        .next()
        .unwrap_or_else(|| DEFAULT_CONFIG_DIR.to_string());

    let server_config_file = match File::open(&config_dir) {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    error!("Config file not found. Writing defaults to disk and exiting.");
                    write_default_config(config_dir).expect("Failed to write default config file");
                }
                e => {
                    error!(
                        "Failed to open server config file: {} ({:?})",
                        config_dir, e
                    );
                }
            }
            return;
        }
    };

    let server_config: ServerConfig = match serde_yaml::from_reader(server_config_file) {
        Ok(c) => c,
        Err(e) => {
            error!("Error parsing server config file: {:?}", e);
            return;
        }
    };

    let mut dispatcher = match Dispatcher::from_config(server_config.dispatcher_config) {
        Ok(d) => d,
        Err(e) => {
            error!("Failed to initialise dispatcher: {:?}", e);
            return;
        }
    };

    let address = server_config.socket_address;
    let socket_receiver = match net::UdpSocket::bind(address) {
        Ok(d) => d,
        Err(e) => {
            error!("Server failed to bind UDP socket: {:?}", e);
            return;
        }
    };

    info!("Starting main loop");

    'message_loop: loop {
        let mut buf = [0u8; MESSAGE_BUFFER_SIZE];

        // Receive the message or error and continue
        let message_bytes = match socket_receiver.recv(&mut buf) {
            Err(e) => {
                error!("Socket: {}", e);
                continue 'message_loop;
            }
            Ok(n) => {
                if n == MESSAGE_BUFFER_SIZE {
                    warn!("Message was the same size as its buffer. This may suggest that the buffer size needs to be larger.");
                }
                &buf[..n]
            }
        };

        // Parse the message struct or error and continue
        let message_struct: GenericMessage = match serde_json::from_slice(message_bytes) {
            Err(e) => {
                error!("JSON parse error: {}", e);
                trace!(
                    "MESSAGE: {:#?}",
                    String::from_utf8(message_bytes.to_vec()).unwrap()
                );
                continue 'message_loop;
            }
            Ok(v) => v,
        };

        // Attempt to dispatch the command to the motor controllers
        if let Err(e) = dispatcher.dispatch(message_struct) {
            error!("Dispatch: {:?}", e);
        }
    }
}
