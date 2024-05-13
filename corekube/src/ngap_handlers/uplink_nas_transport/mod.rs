use log::trace;
use ngap_asn1 as ngap;

use super::NGAPResponse;

pub fn handle_uplink_nas_transport(
    _config: &crate::config::CoreKubeConfig,
    _uplink_nas: ngap::UplinkNASTransport,
) -> Vec<NGAPResponse> {
    trace!("Handling NGAP message of type UplinkNASTransport");
    vec![]
}
