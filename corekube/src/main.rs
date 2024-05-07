use asn1_codecs::{aper::AperCodec, PerCodecData};
use ngap_asn1 as ngap;
use std::net::UdpSocket;
use std::thread;

const BIND_ADDR: &str = "0.0.0.0";
const BIND_PORT: u16 = 9977;
const BUFFER_LEN: usize = 1024;

const MULTITHREAD: bool = true;

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

        // Start a new thread for each received packet
        if MULTITHREAD {
            thread::spawn(move || {
                process_message(socket_clone, &mut buf, size, src);
            });
        } else {
            process_message(socket_clone, &mut buf, size, src);
        }
    }
}

/// Handle the client UDP packet.
fn process_message(
    socket: UdpSocket,
    buf: &mut [u8; BUFFER_LEN],
    size: usize,
    src: std::net::SocketAddr,
) {
    // Shrink the buffer to the actual size of the received data and free rest
    let mut new_buf = vec![0; size];
    new_buf.copy_from_slice(&buf[..size]);
    let mut buf = new_buf;
    println!("received data from: {}", src);
    println!("data: {:?}", buf);

    if buf.len() < 4 {
        // Has to be at least 4 bytes since it contains the frontend ID
        return;
    }

    // Create a slice of the first four bytes and the rest of the data
    let (frontend_id, buf) = buf.split_at_mut(4);
    let mut return_buf = ngap_handler_entrypoint(buf);
    // Append the frontend ID to the return buffer
    let return_buf = [frontend_id, &mut return_buf].concat();

    socket.send_to(&return_buf, src).unwrap();
}

fn ngap_handler_entrypoint(buf: &mut [u8]) -> [u8; 57] {
    // This is a placeholder for the NGAP handler
    println!("NGAP handler entrypoint");
    println!("NGAP: {:?}", buf);

    let mut codec_data = PerCodecData::from_slice_aper(&buf);
    let ngap_pdu = ngap::NGAP_PDU::aper_decode(&mut codec_data).expect("Error decoding NGAP PDU");
    println!("Decoded NGAP PDU: {:?}", ngap_pdu);
    buf
}
