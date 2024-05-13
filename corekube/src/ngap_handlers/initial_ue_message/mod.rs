use log::{debug, error, trace};
use ngap_asn1 as ngap;

use super::NGAPResponse;

#[cfg(test)]
mod tests;

pub fn handle_initial_ue_message(
    config: &crate::config::CoreKubeConfig,
    initial_ue_msg: ngap::InitialUEMessage,
) -> Vec<NGAPResponse> {
    trace!("Handling NGAP message of type InitialUEMessage");

    let mut ran_ue_ngap_id = None;
    let mut nas_pdu = None;
    let mut user_location_information = None;

    // Fill the ProtocolIE values from the request, check if they exist
    for protocol_ie in initial_ue_msg.protocol_i_es.0 {
        match protocol_ie.value {
            ngap::InitialUEMessageProtocolIEs_EntryValue::Id_RAN_UE_NGAP_ID(
                ran_ue_ngap_id_value,
            ) => {
                ran_ue_ngap_id = Some(ran_ue_ngap_id_value);
            }
            ngap::InitialUEMessageProtocolIEs_EntryValue::Id_NAS_PDU(nas_pdu_value) => {
                nas_pdu = Some(nas_pdu_value);
            }
            ngap::InitialUEMessageProtocolIEs_EntryValue::Id_UserLocationInformation(
                user_location_value,
            ) => {
                user_location_information = Some(user_location_value);
            }
            _ => {
                debug!("Ignored ProtocolIE in InitialUEMessage: {:?}", protocol_ie);
            }
        }
    }

    let Some(ran_ue_ngap_id) = ran_ue_ngap_id else {
        error!("Missing RAN_UE_NGAP_ID in InitialUEMessage");
        return vec![];
    };
    debug!("RAN_UE_NGAP_ID: {:?}", ran_ue_ngap_id);

    let Some(nas_pdu) = nas_pdu else {
        error!("Missing NAS_PDU in InitialUEMessage");
        return vec![];
    };
    debug!("NAS_PDU: {:?}", nas_pdu);

    let Some(user_location_information) = user_location_information else {
        error!("Missing UserLocationInformation in InitialUEMessage");
        return vec![];
    };
    debug!("UserLocationInformation: {:?}", user_location_information);

    // Only UserLocationInformation NR is implemented
    let ngap::UserLocationInformation::UserLocationInformationNR(user_location_nr) =
        user_location_information
    else {
        error!("UserLocationInformation is not UserLocationInformationNR");
        return vec![];
    };

    todo!("Implement handling of InitialUEMessage based on NAS_PDU");
}
