use std::net::UdpSocket;
use std::thread;

const BIND_ADDR: &str = "0.0.0.0";
const BIND_PORT: u16 = 9977;
const BUFFER_LEN: usize = 1024;

fn main() {
    println!("Running corekube-rs...");
    println!("Listening on {}:{}", BIND_ADDR, BIND_PORT);
    let socket = UdpSocket::bind((BIND_ADDR, BIND_PORT));
    let socket = match socket {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e),
    };

    loop {
        let mut buf = [0; BUFFER_LEN];
        let (size, src) = socket
            .recv_from(&mut buf)
            .expect("did not receive any data");

        // Clone the socket to pass it to the thread
        let socket_clone = socket.try_clone().expect("couldn't clone the socket");
        thread::spawn(move || {
            handle_client(socket_clone, &mut buf, size, src);
        });
    }
}

/// Handle the client UDP packet.
fn handle_client(
    socket: UdpSocket,
    buf: &mut [u8; BUFFER_LEN],
    size: usize,
    src: std::net::SocketAddr,
) {
    let buf = &mut buf[..size];
    println!("Received data from: {}", src);
    println!("Data: {:?}", buf);
    buf.reverse();

    socket.send_to(buf, &src).unwrap();
}
