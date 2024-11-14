///
/// Struct UpdHeader
/// - syn - message starts with
/// - addr - address of the input channel
/// - type - type of values in the array in data field in struct UpdMessage
/// - count - length of the array in the data field in struct UpdMessage
pub struct UdpHeader {
    pub syn: u8,
    pub addr: u8,
    pub r#type: u8,
    pub count: u32,
}
//
//
impl UdpHeader{
    pub const SYN: u8 = 0; 
    pub const ADDR: u8 = 0;
    pub const TYPE: u8 = 16; 
    ///
    /// Creates a header for udp
    pub fn new(syn: u8, addr: u8, r#type: u8, size:  u32) -> Self{
        Self{
            syn,
            addr,
            r#type,
            count: size,
        }
    }
    ///
    /// Convert fields of UdpHeader to Vector
    pub fn to_bytes(&self) -> Vec<u8>{
        let mut header_bytes = vec![];
        header_bytes.push(self.syn);
        header_bytes.push(self.addr);
        header_bytes.push(self.r#type);
        header_bytes.extend_from_slice(&self.count.to_be_bytes());
        header_bytes
    }
}