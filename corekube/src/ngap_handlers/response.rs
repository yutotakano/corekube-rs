use ngap_asn1 as ngap;

/// A core response type for NGAP messages.
pub struct NGAPResponse {
    pub sctp_stream: u8,
    pub ngap_pdu: ngap::NGAP_PDU,
}

/// A core response type for byte-encoded NGAP messages.
pub struct ByteResponse {
    pub sctp_stream: u8,
    pub buf: Vec<u8>,
}
