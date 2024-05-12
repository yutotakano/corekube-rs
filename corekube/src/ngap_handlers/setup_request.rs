use bitvec::prelude::*;
use log::{error, trace};
use ngap_asn1 as ngap;

#[cfg(test)]
mod tests;

// static COREKUBE_AMF_NAME: &str = "CoreKubeRS_5G_Worker";
static COREKUBE_AMF_NAME: &str = "open5gs-amf0";

pub fn handle_setup_request(ng_setup: ngap::NGSetupRequest, responses: &mut Vec<ngap::NGAP_PDU>) {
    trace!("Handling NGAP message of type NGSetupRequest");

    let mut global_ran_node_id = None;
    let mut supported_ta_list = None;
    let mut paging_drx = None;

    // Fill the ProtocolIE values from the request, check if they exist
    for protocol_ie in ng_setup.protocol_i_es.0 {
        match protocol_ie.value {
            ngap::NGSetupRequestProtocolIEs_EntryValue::Id_GlobalRANNodeID(
                global_ran_node_id_value,
            ) => {
                global_ran_node_id = Some(global_ran_node_id_value);
            }
            ngap::NGSetupRequestProtocolIEs_EntryValue::Id_SupportedTAList(
                supported_ta_list_value,
            ) => {
                supported_ta_list = Some(supported_ta_list_value);
            }
            ngap::NGSetupRequestProtocolIEs_EntryValue::Id_DefaultPagingDRX(
                default_paging_drx_value,
            ) => {
                paging_drx = Some(default_paging_drx_value);
            }
            _ => {
                error!("Unhandled ProtocolIE in NGSetupRequest: {:?}", protocol_ie);
            }
        }
    }

    let Some(global_ran_node_id) = global_ran_node_id else {
        error!("Missing GlobalRANNodeID in NGSetupRequest");
        return;
    };
    trace!("GlobalRANNodeID: {:?}", global_ran_node_id);

    let ngap::GlobalRANNodeID::GlobalGNB_ID(global_gnb_id) = global_ran_node_id else {
        error!("GlobalRANNodeID is not a GlobalGNB_ID");
        return;
    };
    trace!("GlobalGNB_ID: {:?}", global_gnb_id);

    let Some(supported_ta_list) = supported_ta_list else {
        error!("Missing SupportedTAList in NGSetupRequest");
        return;
    };
    trace!("SupportedTAList: {:?}", supported_ta_list);

    let Some(paging_drx) = paging_drx else {
        error!("Missing DefaultPagingDRX in NGSetupRequest");
        return;
    };
    trace!("DefaultPagingDRX: {:?}", paging_drx);

    // TODO: Set SCTP stream ID to 0 here

    // Create the NGSetupResponse
    let response = build_setup_response();
    responses.push(response);
}

fn build_plmn_identity() -> ngap::PLMNIdentity {
    let mcc: u8 = 208;
    let mnc = 93;

    let mut mnc1 = mnc / 100;
    if mnc1 == 0 {
        mnc1 = 0x0f;
    }
    let mnc2 = (mnc / 10) % 10;
    let mnc3 = mnc % 10;

    let mcc1 = mcc / 100;
    let mcc2 = (mcc / 10) % 10;
    let mcc3 = mcc % 10;

    ngap::PLMNIdentity(vec![mcc2 << 4 | mcc1, mnc1 << 4 | mcc3, mnc3 << 4 | mnc2])
}

fn build_amf_region_id() -> ngap::AMFRegionID {
    ngap::AMFRegionID(bitvec![u8, Msb0; 0, 0, 0, 0, 0, 0, 1, 0])
}

fn build_setup_response() -> ngap::NGAP_PDU {
    trace!("Building NGSetupResponse");

    ngap::NGAP_PDU::SuccessfulOutcome(ngap::SuccessfulOutcome {
        procedure_code: ngap::ProcedureCode(ngap::ID_NG_SETUP),
        criticality: ngap::Criticality(ngap::Criticality::REJECT),
        value: ngap::SuccessfulOutcomeValue::Id_NGSetup(ngap::NGSetupResponse {
            protocol_i_es: ngap::NGSetupResponseProtocolIEs {
                0: vec![
                    ngap::NGSetupResponseProtocolIEs_Entry {
                        id: ngap::ProtocolIE_ID(ngap::ID_AMF_NAME),
                        criticality: ngap::Criticality(ngap::Criticality::REJECT),
                        value: ngap::NGSetupResponseProtocolIEs_EntryValue::Id_AMFName(
                            ngap::AMFName(COREKUBE_AMF_NAME.to_owned()),
                        ),
                    },
                    ngap::NGSetupResponseProtocolIEs_Entry {
                        id: ngap::ProtocolIE_ID(ngap::ID_SERVED_GUAMI_LIST),
                        criticality: ngap::Criticality(ngap::Criticality::REJECT),
                        value: ngap::NGSetupResponseProtocolIEs_EntryValue::Id_ServedGUAMIList(
                            ngap::ServedGUAMIList {
                                0: vec![ngap::ServedGUAMIItem {
                                    guami: ngap::GUAMI {
                                        plmn_identity: build_plmn_identity(),
                                        amf_region_id: build_amf_region_id(),
                                        amf_set_id: ngap::AMFSetID(
                                            bitvec![u8, Msb0; 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                                        ),
                                        amf_pointer: ngap::AMFPointer(bitvec![u8, Msb0; 0; 6]),
                                        ie_extensions: None,
                                    },
                                    backup_amf_name: None,
                                    ie_extensions: None,
                                }],
                            },
                        ),
                    },
                    ngap::NGSetupResponseProtocolIEs_Entry {
                        id: ngap::ProtocolIE_ID(ngap::ID_RELATIVE_AMF_CAPACITY),
                        criticality: ngap::Criticality(ngap::Criticality::IGNORE),
                        value: ngap::NGSetupResponseProtocolIEs_EntryValue::Id_RelativeAMFCapacity(
                            ngap::RelativeAMFCapacity(255),
                        ),
                    },
                    ngap::NGSetupResponseProtocolIEs_Entry {
                        id: ngap::ProtocolIE_ID(ngap::ID_PLMN_SUPPORT_LIST),
                        criticality: ngap::Criticality(ngap::Criticality::REJECT),
                        value: ngap::NGSetupResponseProtocolIEs_EntryValue::Id_PLMNSupportList(
                            ngap::PLMNSupportList {
                                0: vec![ngap::PLMNSupportItem {
                                    plmn_identity: build_plmn_identity(),
                                    slice_support_list: ngap::SliceSupportList {
                                        0: vec![ngap::SliceSupportItem {
                                            s_nssai: ngap::S_NSSAI {
                                                sst: ngap::SST(vec![1]),
                                                sd: None,
                                                ie_extensions: None,
                                            },
                                            ie_extensions: None,
                                        }],
                                    },
                                    ie_extensions: None,
                                }],
                            },
                        ),
                    },
                ],
            },
        }),
    })
}
