use crate::{debug::dbg_id::DbgId, error::str_err::StrErr};

use super::udp_header::UdpHeader;
///
/// Struct UpdMessage
/// - header - contains the UPD header information
/// - data - array of values
pub struct UpdMessage {
    header: UdpHeader,
    data: Vec<u8>,
}
//
//
impl UpdMessage{
    ///
    /// Creates a message for UPD
    pub fn new(header: UdpHeader, data: Vec<u8>) -> Self{
        Self{
            header,
            data,
        }
    }
    ///
    /// Convert fields of UdpMessage to Vector
    pub fn build(&self) -> Vec<u8>{
        let mut message_bytes = self.header.to_bytes();
        message_bytes.extend_from_slice(&self.data);
        message_bytes
    }
    ///
    /// Converts bytes into UpdMessage
    pub fn from_be_bytes(bytes: &[u8]) -> Result<Self, StrErr> {
        let dbgid = DbgId(format!("UpdMessage"));
        match bytes {
            [UdpHeader::SYN, UdpHeader::ADDR, UdpHeader::TYPE, _, _, _, _, ..] => {
                let syn = bytes[0];
                let addr = bytes[1];
                let r#type = bytes[2];
                match bytes.get(3..7) {
                    Some(size_bytes) => match size_bytes.try_into() {
                        Ok(size_bytes) => {
                            let size = u32::from_be_bytes(size_bytes);
                            log::debug!("{} | syn: {},  addr: {},  type: {},  size: {:?}", dbgid, syn, addr, r#type, size);
                            let header = UdpHeader::new(syn, addr, r#type, size);
                            let data = bytes.get(6..(size as usize)).unwrap().try_into().unwrap();
                            Ok(UpdMessage::new(header, data))
                        }
                        Err(err) => {
                            panic!("{} | Error parcing size in: {:?}...\n\t error:{:#?}", dbgid, if bytes.len() > 16 {bytes.get(..16).unwrap()} else {&bytes}, err)
                        }
                    }
                    None => {
                        panic!("{} | Error parcing size in: {:?}...", dbgid, if bytes.len() > 16 {bytes.get(..16).unwrap()} else {&bytes})
                    }
                }
            }
            // 0..=1024 => {
            //     log::info!("Received {} bytes", bytes);
            //     println!("Received {} bytes", bytes);
            //     let package = &buffer[4..bytes];  
            //     let header = UdpHeader::new(
            //         buffer[0],
            //         buffer[1],
            //         buffer[2],
            //         u32::from_be_bytes(buffer[3..6].try_into().unwrap()),
            //     );
            //     let message = UpdMessage::new(header, package.to_vec());
            //     log::info!("Received UdpMessage: {:?}", message.build());
            // }
            _ => {
                panic!("{} | Incorrect message: {:?}", dbgid, bytes);
            }
        }        
    }
}