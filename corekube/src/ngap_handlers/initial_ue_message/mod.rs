use log::trace;
use ngap_asn1 as ngap;

use super::NGAPResponse;

#[cfg(test)]
mod tests;

pub fn handle_initial_ue_message(
    _config: &crate::config::CoreKubeConfig,
    _initial_ue_msg: ngap::InitialUEMessage,
) -> Vec<NGAPResponse> {
    trace!("Handling NGAP message of type InitialUEMessage");
    vec![]
}
