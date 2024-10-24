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
    pub count: u8,
}
//
//
impl UdpHeader{
    pub const SYN: u8 = 0; 
    pub const ADDR: u8 = 0;
    pub const TYPE: u8 = 16; 
    pub const COUNT: u8 = 255; 
    ///
    /// Creates a header for udp
    pub fn new(syn: u8, addr: u8, r#type: u8, count:  u8) -> Self{
        Self{
            syn,
            addr,
            r#type,
            count,
        }
    }
    ///
    /// Convert fields of UdpHeader to Vector
    pub fn to_bytes(&self) -> Vec<u8>{
        let mut header_bytes = Vec::with_capacity(self.syn.into());
        header_bytes.push(self.syn);
        header_bytes.push(self.addr);
        header_bytes.push(self.r#type);
        header_bytes.push(self.count);
        header_bytes
    }
}