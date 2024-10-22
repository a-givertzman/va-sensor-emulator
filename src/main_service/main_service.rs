use std::{io::{Error}, net::{ToSocketAddrs, UdpSocket}, os::unix::net::SocketAddr, sync::{atomic::{AtomicBool, Ordering}, mpsc::Sender, Arc, Mutex}, thread, time::Duration};
use log::{info, warn};
use sal_sync::services::{conf::conf_tree::{self, ConfTree}, entity::{
        name::Name, object::Object, point::point::Point
        // services::{
        //     services::Services,
        //     service::service::Service,
        //     service::service_handles::ServiceHandles, 
        // }, 
}, service::{service::Service, service_cycle::ServiceCycle, service_handles::ServiceHandles}, types::type_of::TypeOf};
use serde::de::value;

use super::main_service_config::MainServiceConf;
use crate::Buffer;
//
/// Struct `UpdHeader`
/// - `syn` - message starts with
/// - `addr` - address of the input channel
/// - `type` - type of values in the array in data field in struct `UpdMessage`
/// - `count` - length of the array in the data field in struct `UpdMessage`
pub struct UdpHeader {
    pub syn: u8,
    pub addr: u8,
    pub r#type: u8,
    pub count: u8,
}
//
//
impl UdpHeader{
    //
    /// Creates a header for udp
    pub fn new(syn: u8, addr: u8, r#type: u8, count:  u8) -> Self{
        Self{
            syn,
            addr,
            r#type,
            count,
        }
    }
    //
    /// Convert fields of UdpHeader to Vector
    pub fn to_bytes(&self) -> Vec<u8>{
        let mut header_bytes = Vec::with_capacity(4);
        header_bytes.push(self.syn);
        header_bytes.push(self.addr);
        header_bytes.push(self.r#type);
        header_bytes.push(self.count);
        header_bytes
    }
}
//
/// Struct `UpdMessage`
/// - `header` - contains the UPD header information
/// - `data` - array of values
pub struct UpdMessage{
    pub header: UdpHeader,
    pub data: Vec<u8>,
}
//
//
impl UpdMessage{
    //
    /// Creates a message for udp
    pub fn new(header: UdpHeader, data: Vec<u8>) -> Self{
        Self{
            header,
            data,
        }
    }
    //
    /// Convert fields of UdpMessage to Vector
    pub fn message(&self) -> Vec<u8>{
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
//
/// Struct `MainService`
/// - `dbg_id` - id for debugging
/// - `name` - name of the service
/// - `conf` - configuration settings for the service
/// - `exit` - boolean flag for monitoring the operation of the service
pub struct MainService{
    dbg_id: String,
    name: Name,
    conf: MainServiceConf,
    exit: Arc<AtomicBool>,
}
impl MainService {
    //
    /// Creates new instance of the MainService 
    pub fn new(parent: impl Into<String>, conf: MainServiceConf) -> Self {
        Self {
            dbg_id: conf.name.join(),
            name: conf.name.clone(),
            conf: conf.clone(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    //
    /// Bind the UDP socket
    fn udp_bind(addr: impl ToSocketAddrs + std::fmt::Display) -> Result<UdpSocket, Error> {
        // UDP Bind 
        info!("Start binding to the: {}", addr);
        match UdpSocket::bind(&addr){
            Ok(socket) => {
                Ok(socket)
            },
            Err(error) => {
                panic!("MainService.bind | Failed to bind due to {}", error)
            },
        }
    }

}
//
//
impl Object for MainService {
    fn id(&self) -> &str {
        &self.dbg_id
    }
    
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
// 
impl std::fmt::Debug for MainService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("MainService")
            .field("id", &self.dbg_id)
            .finish()
    }
}
//
// 
impl Service for MainService {
    //
    // 
    fn get_link(&mut self, name: &str) -> Sender<Point> {
        panic!("{}.get_link | Does not support get_link", self.id())
        // match self.rxSend.get(name) {
        //     Some(send) => send.clone(),
        //     None => panic!("{}.run | link '{:?}' - not found", self.id, name),
        // }
    }
    //
    //
    fn run(&mut self) -> Result<ServiceHandles<()>, String> {
        info!("{}.run | Starting...", self.dbg_id);
        let dbg_id = self.dbg_id.clone();
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        info!("{}.run | Preparing thread...", dbg_id);

        let handle = thread::Builder::new().name(format!("{}.run", dbg_id.clone())).spawn(move || {
            let interval = 0.0; // from sampl_freq & buf_size
            let interval = Duration::from_secs_f64(interval);
            let mut cycle = ServiceCycle::new(&dbg_id, interval);
            let addr = "127.0.0.1:15181"; //other address in string format
            let mut buf = Buffer::new(512);

            loop {
                for (freq, amp, phi) in conf.signal.iter(){
                    let amplitude = amp;
                    match buf.add(*amplitude){
                        Some(_) =>{
                            match Self::udp_bind(addr){
                                Ok(socket) => {
                                    cycle.start();
                                    let header = UdpHeader::new(0, 0, 32, 255);
                                    let buf_i16 = buf.array.clone();
                                    let bytes = buf_i16.iter().flat_map(|&byte|byte.to_ne_bytes()).collect();
    
                                    let message = UpdMessage::new(header, bytes);
    
                                    // &[u8]
                                    socket.send(&message.message()).expect("Failed to send message");
    
                                    cycle.wait();
                                }
                                Err(err) => log::error!("{}.run | Udp bind error: {}", dbg_id, err),
                            }
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.dbg_id);
                Ok(ServiceHandles::new(vec![(self.dbg_id.clone(), handle)]))
            }
            Err(err) => {
                let message = format!("{}.run | Start failed: {:#?}", self.dbg_id, err);
                warn!("{}", message);
                Err(message)
            }
        }
    }
    //
    //
    fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }    
}