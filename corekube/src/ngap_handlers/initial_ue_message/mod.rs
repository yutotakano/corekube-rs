use log::trace;
use ngap_asn1 as ngap;

#[cfg(test)]
mod tests;

pub fn handle_initial_ue_message(
    _config: &crate::config::CoreKubeConfig,
    _initial_ue_msg: ngap::InitialUEMessage,
    _responses: &mut Vec<ngap::NGAP_PDU>,
) {
    trace!("Handling NGAP message of type InitialUEMessage");
}
