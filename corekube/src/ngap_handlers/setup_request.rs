use log::trace;
use ngap_asn1 as ngap;

pub fn handle_setup_request(ng_setup: ngap::NGSetupRequest, responses: &mut Vec<ngap::NGAP_PDU>) {
    trace!("Handling NGAP message of type NGSetupRequest");
}
