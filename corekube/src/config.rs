use bitvec::prelude::*;

/// Configuration for the Core
pub struct CoreKubeConfig {
    pub bind_addr: String,
    pub bind_port: u16,
    pub multithreaded: bool,
    pub amf_name: String,
    pub amf_region_id: BitVec<u8, Msb0>,
    pub amf_set_id: BitVec<u8, Msb0>,
    pub amf_pointer: BitVec<u8, Msb0>,
    pub mcc: u8,
    pub mnc: u8,
    pub relative_amf_capacity: u8,
    pub sst: Vec<u8>,
}

impl Default for CoreKubeConfig {
    fn default() -> Self {
        CoreKubeConfig {
            bind_addr: "0.0.0.0".to_string(),
            bind_port: 9977,
            multithreaded: true,
            amf_name: "CoreKubeRS_5G_Worker".to_string(),
            amf_region_id: bitvec![u8, Msb0; 0, 0, 0, 0, 0, 0, 1, 0],
            amf_set_id: bitvec![u8, Msb0; 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            amf_pointer: bitvec![u8, Msb0; 0; 6],
            mcc: 208,
            mnc: 93,
            relative_amf_capacity: 255,
            sst: vec![1],
        }
    }
}
