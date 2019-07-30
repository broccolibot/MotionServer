use server::generic_message::{GenericCommand, GenericMessage};
use std::net;

fn main() {
    let mut args = std::env::args();
    args.next();
    let send_address: net::SocketAddr = match args.next() {
        None => {
            eprintln!("No address specified");
            return;
        }
        Some(arg) => match arg.parse() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error parsing address: {:?}", e);
                return;
            }
        },
    };
    let socket_sender =
        net::UdpSocket::bind("0.0.0.0:0").expect("Server failed to bind UDP socket!");
    let mut enable = false;
    loop {
        enable = !enable;
        let message =
            GenericMessage::Controller("example".to_string(), GenericCommand::Enable(enable));
        let message_string = serde_json::to_string_pretty(&message).unwrap();
        socket_sender
            .send_to(message_string.as_bytes(), send_address)
            .expect("Failed to send");
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
