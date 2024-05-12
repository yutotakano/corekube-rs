use log::trace;
use ngap_asn1 as ngap;

pub fn handle_initial_ue_message(
    initial_ue_msg: ngap::InitialUEMessage,
    responses: &mut Vec<ngap::NGAP_PDU>,
) {
    trace!("Handling NGAP message of type InitialUEMessage");
}
