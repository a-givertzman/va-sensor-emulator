use super::upd_header::UdpHeader;
///
/// Struct UpdMessage
/// - header - contains the UPD header information
/// - data - array of values
pub struct UpdMessage{
    pub header: UdpHeader,
    pub data: Vec<u8>,
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
    pub fn build_message(&self) -> Vec<u8>{
        let mut message_bytes = Vec::new();
        for bytes in self.header.to_bytes(){
            message_bytes.push(bytes);
        }
        for &bytes in &self.data{
            message_bytes.push(bytes);
        }
        message_bytes
    }
}