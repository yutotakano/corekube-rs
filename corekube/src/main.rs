use asn1_codecs::{aper::AperCodec, PerCodecData};
use flexi_logger::Logger;
use log::{debug, info, trace};
use ngap_asn1 as ngap;
use std::net::UdpSocket;
use std::sync::Arc;
use std::thread;

mod config;
mod ngap_handlers;

#[cfg(test)]
mod tests;

const BUFFER_LEN: usize = 1024;

fn main() {
    let _logger = Logger::try_with_env_or_str("info")
        .expect("could not retrieve log level")
        .format_for_stderr(flexi_logger::colored_default_format)
        .start()
        .expect("could not start logger");

    // Load the default configuration. We wrap it in an atomic reference to
    // allow sharing it between threads.
    let config: Arc<config::CoreKubeConfig> = Arc::new(Default::default());

    info!("Running corekube-rs...");
    info!("Listening on {}:{}", config.bind_addr, config.bind_port);
    let socket = UdpSocket::bind((config.bind_addr.as_str(), config.bind_port));
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

        // Clone the reference to the config
        let config = Arc::clone(&config);

        // Start a new thread for each received packet
        if config.multithreaded {
            thread::spawn(move || {
                process_message(&*config, socket_clone, &mut buf, size, src);
            });
        } else {
            process_message(&*config, socket_clone, &mut buf, size, src);
        }
    }
}

/// Handle the client UDP packet.
fn process_message(
    config: &config::CoreKubeConfig,
    socket: UdpSocket,
    buf: &mut [u8; BUFFER_LEN],
    size: usize,
    src: std::net::SocketAddr,
) {
    // Shrink the buffer to the actual size of the received data and free rest
    let mut new_buf = vec![0; size];
    new_buf.copy_from_slice(&buf[..size]);
    let mut buf = new_buf;
    trace!("processing data of size {} from: {}", size, src);
    debug!("data: {:?}", buf);

    if buf.len() < 4 {
        // Has to be at least 4 bytes since it contains the frontend ID
        return;
    }

    // Create a slice of the first four bytes and the rest of the data
    let (frontend_id, buf) = buf.split_at_mut(4);
    debug!("frontend_id: {:?}", frontend_id);

    let responses = ngap_handler_entrypoint(config, buf);

    // Send the responses back to the client
    for mut return_buf in responses {
        // Append the frontend ID to the return buffer
        let return_buf = [
            &mut *frontend_id,
            &mut vec![return_buf.sctp_stream],
            &mut return_buf.buf,
        ]
        .concat();
        socket.send_to(&return_buf, src).unwrap();
    }
}

fn ngap_handler_entrypoint(
    config: &config::CoreKubeConfig,
    buf: &[u8],
) -> Vec<ngap_handlers::ByteResponse> {
    // This is a placeholder for the NGAP handler
    trace!("NGAP handler entrypoint");
    debug!("NGAP: {:?}", buf);

    let mut codec_data = PerCodecData::from_slice_aper(&buf);
    let ngap_pdu = ngap::NGAP_PDU::aper_decode(&mut codec_data).expect("Error decoding NGAP PDU");

    let responses = match ngap_pdu {
        ngap::NGAP_PDU::InitiatingMessage(init_msg) => {
            ngap_initiating_message_handler(config, init_msg)
        }
        ngap::NGAP_PDU::SuccessfulOutcome(success_outcome) => {
            info!("SuccessfulOutcome: {:?}", success_outcome);
            vec![]
        }
        ngap::NGAP_PDU::UnsuccessfulOutcome(unsuccess_outcome) => {
            info!("UnsuccessfulOutcome: {:?}", unsuccess_outcome);
            vec![]
        }
    };

    let mut codec_data = PerCodecData::default();
    responses
        .iter()
        .map(|resp| {
            resp.ngap_pdu
                .aper_encode(&mut codec_data)
                .expect("Error encoding NGAP PDU");
            let buf = codec_data.get_inner().expect("Error getting inner buffer");
            ngap_handlers::ByteResponse {
                sctp_stream: resp.sctp_stream,
                buf: buf.to_vec(),
            }
        })
        .collect()
}

fn ngap_initiating_message_handler(
    config: &config::CoreKubeConfig,
    init_msg: ngap::InitiatingMessage,
) -> Vec<ngap_handlers::NGAPResponse> {
    trace!("Handling NGAP message of type InitiaingMessage");

    match init_msg.value {
        ngap::InitiatingMessageValue::Id_NGSetup(ng_setup) => {
            ngap_handlers::handle_setup_request(config, ng_setup)
        }
        ngap::InitiatingMessageValue::Id_InitialUEMessage(ue_msg) => {
            ngap_handlers::handle_initial_ue_message(config, ue_msg)
        }
        ngap::InitiatingMessageValue::Id_UplinkNASTransport(nas_transport) => {
            ngap_handlers::handle_uplink_nas_transport(config, nas_transport)
        }
        unhandled => {
            info!("Unknown InitiatingMessage: {:?}", unhandled);
            vec![]
        }
    }
}
