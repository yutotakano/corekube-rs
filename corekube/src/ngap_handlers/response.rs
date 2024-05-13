use ngap_asn1 as ngap;

pub struct NGAPResponse {
    pub sctp_stream: u8,
    pub ngap_pdu: ngap::NGAP_PDU,
}

pub struct ByteResponse {
    pub sctp_stream: u8,
    pub buf: Vec<u8>,
}
