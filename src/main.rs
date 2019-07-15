use i2cdev::core::I2CDevice;
use log::{warn, error};
use server::aimc_config::AIMCConfig;
use server::dispatcher::*;
use server::i2c_trace_mock::I2CTraceMock;
use server::generic_message::*;
use std::io::Write;
use std::net;

fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Trace)
        .format(|buf, record| writeln!(buf, "{:5}| {}", record.level(), record.args()))
        .init();

    let dispatcher_config = DispatcherConfig {
        i2c_device_file: "/dev/i2c-1",
        aimcs: std::collections::HashMap::new(),
        debug_devices: vec!["debu".to_string()],
    };

    let mut dispatcher =
        Dispatcher::from_config(dispatcher_config).expect("Failed to initialise dispatcher");

    let message = GenericMessage::Controller("debug".to_string(), GenericCommand::SetTarget(4324.84329));
    if let Err(e) = dispatcher.dispatch(message) {
        error!("Dispatch error: {:?}", e);
    }

    /*
    let bind_address =
        net::SocketAddr::new(net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1)), 5060);
    let socket_receiver = net::UdpSocket::bind(bind_address).expect("Failed to bind UDP socket!");

    loop {
        let mut buf = [0u8; 2048];
        if let Err(e) = socket_receiver.recv_from(&mut buf) {
            warn!("Socket error: {}", e);
        } else {
            println!("Message: {}", String::from_utf8(buf.to_vec()).expect("Failed to parse UTF-8"));
        }
    }
    */
}

/*
//use std::fs::File;
fn main() {
    let mut server = Server::new(handle_server_config(), true);

    println!(
        "{:?}",
        server.process_network_request(NetworkRequest::RawCommand(
            "fdjal".to_string(),
            ControllerMessage::Home(-45)
        ))
    );

    println!(
        "{:?}",
        server.process_network_request(NetworkRequest::TargetSet("Example".to_string(), 432.43829))
    );

    //let shared_server = Arc::new(Mutex::new(server));
}
    */

/*
const SERVER_CONFIG_NAME: &str = "server.json";
fn handle_server_config(server_config_path: &Path) -> ServerConfig {
    if let Ok(file) = File::open(server_config_path) {
        serde_json::from_reader(file).expect("Unable to parse server config")
    } else {
        let file =
            File::create(server_config_path).expect("Failed to write default server config.");
        let instance = Default::default();
        serde_json::to_writer_pretty(file, &instance).expect("Unable to write server JSON");
        eprintln!(
            "WARNING: unable to find server config '{}', using defaults!",
            server_config_path
        );
        instance
    }
}
*/
