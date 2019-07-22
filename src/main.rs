use log::{error, trace, warn};
use serde::{Deserialize, Serialize};
use server::dispatcher::*;
use server::generic_message::*;
use std::{
    fs::File,
    io::{ErrorKind, Write},
    net,
    path::Path,
};

const DEFAULT_CONFIG_DIR: &str = "server.json";
const MESSAGE_BUFFER_SIZE: usize = 4096;

#[derive(Serialize, Deserialize)]
struct ServerConfig {
    pub socket_address: net::SocketAddr,
    pub dispatcher_config: DispatcherConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            socket_address: net::SocketAddr::new(
                net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)),
                5060,
            ),
            dispatcher_config: Default::default(),
        }
    }
}

fn write_default_config<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    serde_json::to_writer_pretty(&mut file, &ServerConfig::default())
        .expect("Failed to serialize dispatcher config");
    Ok(())
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Trace)
        .format(|buf, record| writeln!(buf, "{:5}| {}", record.level(), record.args()))
        .init();

    let mut args = std::env::args();
    args.next();

    let config_dir = &args.next().unwrap_or(DEFAULT_CONFIG_DIR.to_string());

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

    let server_config: ServerConfig = match serde_json::from_reader(server_config_file) {
        Ok(c) => c,
        Err(e) => {
            error!("Error parsing server config file: {:?}", e);
            return;
        }
    };

    let mut dispatcher = Dispatcher::from_config(server_config.dispatcher_config)
        .expect("Failed to initialise dispatcher");

    let address = server_config.socket_address;
    let socket_receiver = net::UdpSocket::bind(address).expect("Server failed to bind UDP socket!");
    let socket_sender = socket_receiver.try_clone().expect("Failed to clone socket");

    // Hnnnng kernel, I'm dummy
    std::thread::spawn(move || {
        let mut i = 0.0;
        loop {
            i += 0.1;
            let message =
                GenericMessage::Controller("debug".to_string(), GenericCommand::SetTarget(i));
            let message_string = serde_json::to_string_pretty(&message).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(20));
            socket_sender
                .send_to(message_string.as_bytes(), address)
                .expect("Failed to send");
        }
    });

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
                    warn!("Message was the same size as MESSAGE_BUFFER_SIZE. This may suggest that the buffer size needs to be larger.");
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
