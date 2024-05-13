use bitvec::{prelude::*, ptr::null};
use log::trace;

pub static NAS_UNSPECIFIED_PROTOCOL_ERROR: u8 = 111;
pub static NAS_MESSAGE_TYPE_NONEXISTENT: u8 = 97;
pub static NAS_INVALID_MANDATORY_INFO: u8 = 96;

pub fn parse(buf: Vec<u8>, inner: bool, sec_hdr: bool, null_cipher: bool) -> Result<Vec<u8>, u8> {
    if buf.len() < 3 {
        return Err(NAS_UNSPECIFIED_PROTOCOL_ERROR);
    }

    let protocol_discriminator = ProtocolDiscriminator::from_u8(buf[0]);
    let sec_hdr_type = SecurityHeader::from_u8(buf[1]);
    let message_id = MessageIdentifier::from_u8(buf[2]);

    let protocol_discriminator = protocol_discriminator.ok_or(NAS_UNSPECIFIED_PROTOCOL_ERROR)?;

    // Parse, recurse and exit if the message is security protected
    if protocol_discriminator == ProtocolDiscriminator::MobilityManagement
        && let Some(sec_hdr_type) = sec_hdr_type
        && sec_hdr_type != SecurityHeader::NotProtected
    {
        trace!("Security protected NAS message");

        // Parse the security protected NAS message
        let msg = parse_sec_prot_nas(buf, inner, null_cipher);
        let Some(msg) = msg else {
            return Err(NAS_INVALID_MANDATORY_INFO);
        };

        // If we're told to decode the inner message as well, and we can, then
        // do so by recursing.
        if inner
            && (sec_hdr_type == SecurityHeader::IntegrityProtected
                || sec_hdr_type == SecurityHeader::IntegrityProtectedWithNewSecurityContext
                || null_cipher)
        {
            trace!("Parse clear-text NAS message payload");
            let result = parse(msg[3], inner, true, false);
            return Ok(msg);
            //                 if cont is not None:
            //     Msg.replace(Msg[3], cont)
            // return Msg, err
        } else {
            // Otherwise leave the inner payload as is and exit
            return Ok(msg);
        }
    }

    let message = if protocol_discriminator == ProtocolDiscriminator::MobilityManagement {
        trace!("5GMM unprotected NAS message");
        let msg = fgmm_type_classes(message_id);
        let Some(msg) = msg else {
            return Err(NAS_MESSAGE_TYPE_NONEXISTENT);
        };
        Ok(msg)
    } else if protocol_discriminator == ProtocolDiscriminator::SessionManagement {
        trace!("5GSM");
        let sm_typ = buf.get(3);
        let Some(sm_typ) = sm_typ else {
            return Err(NAS_INVALID_MANDATORY_INFO);
        };

        let msg = fgsm_type_classes(message_id);
        let Some(msg) = msg else {
            return Err(NAS_MESSAGE_TYPE_NONEXISTENT);
        };
        Ok(msg)
    };
}

pub fn parse_sec_prot_nas(buf: Vec<u8>, inner: bool, null_cipher: bool) -> Optional<Vec<u8>> {
    None
}

pub fn fgmm_type_classes(typ: u8) -> Option<Vec<u8>> {
    None
}

#[allow(non_camel_case_types)]
enum SecurityHeader {
    NotProtected = 0,
    IntegrityProtected = 1,
    IntegrityProtectedAndCiphered = 3,
    // Can only be used with Security Mode Command
    IntegrityProtectedWithNewSecurityContext = 4,
    // Can only be used with Security Mode Complete
    IntegrityProtectedAndCipheredWithNewSecurityContext = 5,
}

impl SecurityHeader {
    pub fn from_u8(value: u8) -> Option<SecurityHeader> {
        match value {
            0 => Some(SecurityHeader::NotProtected),
            1 => Some(SecurityHeader::IntegrityProtected),
            3 => Some(SecurityHeader::IntegrityProtectedAndCiphered),
            4 => Some(SecurityHeader::IntegrityProtectedWithNewSecurityContext),
            5 => Some(SecurityHeader::IntegrityProtectedAndCipheredWithNewSecurityContext),
            _ => None,
        }
    }
}

#[allow(non_camel_case_types)]
enum MessageIdentifier {
    MobilityManagement(MobilityMessageIdentifier),
    SessionManagement(SessionMessageIdentifier),
}

impl MessageIdentifier {
    pub fn from_u8(value: u8) -> Option<MessageIdentifier> {
        // If first two bits are 01, it is a Mobility Management message.
        // If it is 11, it is a Session Management message.
        match value & 0xC0 {
            0x40 => {
                MobilityMessageIdentifier::from_u8(value).map(MessageIdentifier::MobilityManagement)
            }
            0xC0 => {
                SessionMessageIdentifier::from_u8(value).map(MessageIdentifier::SessionManagement)
            }
            _ => None,
        }
    }
}

enum ProtocolDiscriminator {
    MobilityManagement = 0x7E,
    SessionManagement = 0x2E,
}

impl ProtocolDiscriminator {
    pub fn from_u8(value: u8) -> Option<ProtocolDiscriminator> {
        match value {
            0x7E => Some(ProtocolDiscriminator::MobilityManagement),
            0x2E => Some(ProtocolDiscriminator::SessionManagement),
            _ => None,
        }
    }
}

#[allow(non_camel_case_types)]
enum MobilityMessageIdentifier {
    REGISTRATION_REQUEST = 0x41,
    REGISTRATION_ACCEPT = 0x42,
    REGISTRATION_COMPLETE = 0x43,
    REGISTRATION_REJECT = 0x44,
    DEREGISTRATION_REQUEST = 0x45,
    DEREGISTRATION_ACCEPT = 0x46,
    DEREGISTRATION_REQUEST_UE_ORIGINATING = 0x47,
    DEREGISTRATION_ACCEPT_UE_ORIGINATING = 0x48,

    SERVICE_REQUEST = 0x4C,
    SERVICE_REJECT = 0x4D,
    SERVICE_ACCEPT = 0x4E,
    CONTROL_PLANE_SERVICE_REQUEST = 0x4F,

    SLICE_SPECIFIC_AUTHENTICATION_COMMAND = 0x50,
    SLICE_SPECIFIC_AUTHENTICATION_COMPLETE = 0x51,
    SLICE_SPECIFIC_AUTHENTICATION_RESULT = 0x52,
    CONFIGURATION_UPDATE_COMMAND = 0x54,
    CONFIGURATION_UPDATE_COMPLETE = 0x55,
    AUTHENTICATION_REQUEST = 0x56,
    AUTHENTICATION_RESPONSE = 0x57,
    AUTHENTICATION_REJECT = 0x58,
    AUTHENTICATION_FAILURE = 0x59,
    AUTHENTICATION_RESULT = 0x5A,
    IDENTITY_REQUEST = 0x5B,
    IDENTITY_RESPONSE = 0x5C,
    SECURITY_MODE_COMMAND = 0x5D,
    SECURITY_MODE_COMPLETE = 0x5E,
    SECURITY_MODE_REJECT = 0x5F,

    STATUS = 0x64,
    NOTIFICATION = 0x65,
    NOTIFICATION_RESPONSE = 0x66,
    UPLINK_NAS_TRANSPORT = 0x67,
    DOWNLINK_NAS_TRANSPORT = 0x68,
}

impl MobilityMessageIdentifier {
    pub fn from_u8(value: u8) -> Option<MobilityMessageIdentifier> {
        match value {
            0x41 => Some(MobilityMessageIdentifier::REGISTRATION_REQUEST),
            0x42 => Some(MobilityMessageIdentifier::REGISTRATION_ACCEPT),
            0x43 => Some(MobilityMessageIdentifier::REGISTRATION_COMPLETE),
            0x44 => Some(MobilityMessageIdentifier::REGISTRATION_REJECT),
            0x45 => Some(MobilityMessageIdentifier::DEREGISTRATION_REQUEST),
            0x46 => Some(MobilityMessageIdentifier::DEREGISTRATION_ACCEPT),
            0x47 => Some(MobilityMessageIdentifier::DEREGISTRATION_REQUEST_UE_ORIGINATING),
            0x48 => Some(MobilityMessageIdentifier::DEREGISTRATION_ACCEPT_UE_ORIGINATING),
            0x4C => Some(MobilityMessageIdentifier::SERVICE_REQUEST),
            0x4D => Some(MobilityMessageIdentifier::SERVICE_REJECT),
            0x4E => Some(MobilityMessageIdentifier::SERVICE_ACCEPT),
            0x4F => Some(MobilityMessageIdentifier::CONTROL_PLANE_SERVICE_REQUEST),
            0x50 => Some(MobilityMessageIdentifier::SLICE_SPECIFIC_AUTHENTICATION_COMMAND),
            0x51 => Some(MobilityMessageIdentifier::SLICE_SPECIFIC_AUTHENTICATION_COMPLETE),
            0x52 => Some(MobilityMessageIdentifier::SLICE_SPECIFIC_AUTHENTICATION_RESULT),
            0x54 => Some(MobilityMessageIdentifier::CONFIGURATION_UPDATE_COMMAND),
            0x55 => Some(MobilityMessageIdentifier::CONFIGURATION_UPDATE_COMPLETE),
            0x56 => Some(MobilityMessageIdentifier::AUTHENTICATION_REQUEST),
            0x57 => Some(MobilityMessageIdentifier::AUTHENTICATION_RESPONSE),
            0x58 => Some(MobilityMessageIdentifier::AUTHENTICATION_REJECT),
            0x59 => Some(MobilityMessageIdentifier::AUTHENTICATION_FAILURE),
            0x5A => Some(MobilityMessageIdentifier::AUTHENTICATION_RESULT),
            0x5B => Some(MobilityMessageIdentifier::IDENTITY_REQUEST),
            0x5C => Some(MobilityMessageIdentifier::IDENTITY_RESPONSE),
            0x5D => Some(MobilityMessageIdentifier::SECURITY_MODE_COMMAND),
            0x5E => Some(MobilityMessageIdentifier::SECURITY_MODE_COMPLETE),
            0x5F => Some(MobilityMessageIdentifier::SECURITY_MODE_REJECT),
            0x64 => Some(MobilityMessageIdentifier::STATUS),
            0x65 => Some(MobilityMessageIdentifier::NOTIFICATION),
            0x66 => Some(MobilityMessageIdentifier::NOTIFICATION_RESPONSE),
            0x67 => Some(MobilityMessageIdentifier::UPLINK_NAS_TRANSPORT),
            0x68 => Some(MobilityMessageIdentifier::DOWNLINK_NAS_TRANSPORT),
            _ => None,
        }
    }
}

#[allow(non_camel_case_types)]
enum SessionMessageIdentifier {
    PDU_SESSION_ESTABLISHMENT_REQUEST = 0xC1,
    PDU_SESSION_ESTABLISHMENT_ACCEPT = 0xC2,
    PDU_SESSION_ESTABLISHMENT_REJECT = 0xC3,

    PDU_SESSION_AUTHENTICATION_COMMAND = 0xC5,
    PDU_SESSION_AUTHENTICATION_COMPLETE = 0xC6,
    PDU_SESSION_AUTHENTICATION_RESULT = 0xC7,

    PDU_SESSION_MODIFICATION_REQUEST = 0xC9,
    PDU_SESSION_MODIFICATION_REJECT = 0xCA,
    PDU_SESSION_MODIFICATION_COMMAND = 0xCB,
    PDU_SESSION_MODIFICATION_COMPLETE = 0xCC,
    PDU_SESSION_MODIFICATION_COMMAND_REJECT = 0xCD,

    PDU_SESSION_RELEASE_REQUEST = 0xCF,
    PDU_SESSION_RELEASE_REJECT = 0xD0,
    PDU_SESSION_RELEASE_COMMAND = 0xD1,
    PDU_SESSION_RELEASE_COMPLETE = 0xD2,

    STATUS = 0xD6,
}

impl SessionMessageIdentifier {
    pub fn from_u8(value: u8) -> Option<SessionMessageIdentifier> {
        match value {
            0xC1 => Some(SessionMessageIdentifier::PDU_SESSION_ESTABLISHMENT_REQUEST),
            0xC2 => Some(SessionMessageIdentifier::PDU_SESSION_ESTABLISHMENT_ACCEPT),
            0xC3 => Some(SessionMessageIdentifier::PDU_SESSION_ESTABLISHMENT_REJECT),
            0xC5 => Some(SessionMessageIdentifier::PDU_SESSION_AUTHENTICATION_COMMAND),
            0xC6 => Some(SessionMessageIdentifier::PDU_SESSION_AUTHENTICATION_COMPLETE),
            0xC7 => Some(SessionMessageIdentifier::PDU_SESSION_AUTHENTICATION_RESULT),
            0xC9 => Some(SessionMessageIdentifier::PDU_SESSION_MODIFICATION_REQUEST),
            0xCA => Some(SessionMessageIdentifier::PDU_SESSION_MODIFICATION_REJECT),
            0xCB => Some(SessionMessageIdentifier::PDU_SESSION_MODIFICATION_COMMAND),
            0xCC => Some(SessionMessageIdentifier::PDU_SESSION_MODIFICATION_COMPLETE),
            0xCD => Some(SessionMessageIdentifier::PDU_SESSION_MODIFICATION_COMMAND_REJECT),
            0xCF => Some(SessionMessageIdentifier::PDU_SESSION_RELEASE_REQUEST),
            0xD0 => Some(SessionMessageIdentifier::PDU_SESSION_RELEASE_REJECT),
            0xD1 => Some(SessionMessageIdentifier::PDU_SESSION_RELEASE_COMMAND),
            0xD2 => Some(SessionMessageIdentifier::PDU_SESSION_RELEASE_COMPLETE),
            0xD6 => Some(SessionMessageIdentifier::STATUS),
            _ => None,
        }
    }
}
