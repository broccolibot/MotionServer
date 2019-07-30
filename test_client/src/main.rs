use server::generic_message::{GenericCommand, GenericMessage};
use std::net;

fn main() {
    let send_address: net::SocketAddr = "127.0.0.1:5060".parse().unwrap();
    let socket_sender =
        net::UdpSocket::bind("0.0.0.0:0").expect("Server failed to bind UDP socket!");
    let mut i = 0.0;
    loop {
        i += 0.1;
        let message = GenericMessage::Controller("debug".to_string(), GenericCommand::SetTarget(i));
        let message_string = serde_json::to_string_pretty(&message).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        socket_sender
            .send_to(message_string.as_bytes(), send_address)
            .expect("Failed to send");
    }
}
