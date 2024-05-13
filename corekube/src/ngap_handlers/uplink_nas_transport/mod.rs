use log::trace;
use ngap_asn1 as ngap;

pub fn handle_uplink_nas_transport(
    _config: &crate::config::CoreKubeConfig,
    _uplink_nas: ngap::UplinkNASTransport,
    _responses: &mut Vec<ngap::NGAP_PDU>,
) {
    trace!("Handling NGAP message of type UplinkNASTransport");
}