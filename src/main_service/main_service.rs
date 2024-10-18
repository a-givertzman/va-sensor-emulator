use std::{io::{Error, ErrorKind}, net::{TcpStream, ToSocketAddrs, UdpSocket}, os::unix::net::SocketAddr, sync::{atomic::{AtomicBool, Ordering}, mpsc::Sender, Arc, Mutex}, thread, time::Duration};
use log::{info, warn};
use sal_sync::services::{conf::conf_tree::{self, ConfTree}, entity::{
        name::Name, object::Object, point::point::Point
        // services::{
        //     services::Services,
        //     service::service::Service,
        //     service::service_handles::ServiceHandles, 
        // }, 
}, service::{service::Service, service_cycle::ServiceCycle, service_handles::ServiceHandles}, types::type_of::TypeOf};

use super::main_service_config::MainServiceConf;
//
//
pub struct UdpHeader {
    pub syn: u8,
    pub addr: u8,
    pub r#type: u8,
    pub count: u8,
}
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
    pub fn header_to_bytes(&self) -> Vec<u8>{
        let mut header_bytes = Vec::with_capacity(4);
        header_bytes.push(self.syn);
        header_bytes.push(self.addr);
        header_bytes.push(self.r#type);
        header_bytes.push(self.count);

        header_bytes
    }
}
//
//
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
    pub fn message_to_bytes(&self) -> Vec<u8>{
        let mut message_bytes = Vec::new();

        for bytes in self.header.header_to_bytes(){
            message_bytes.push(bytes);
        }

        for &bytes in &self.data{
            message_bytes.push(bytes);
        }

        message_bytes
    }
}
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
    ///
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
        let conf_tree = self.conf.clone();
        let exit = self.exit.clone();
        info!("{}.run | Preparing thread...", dbg_id);

        let handle = thread::Builder::new().name(format!("{}.run", dbg_id.clone())).spawn(move || {
            let interval = 0.0; // from sampl_freq & buf_size
            let interval = Duration::from_secs_f64(interval);
            let mut cycle = ServiceCycle::new(&dbg_id, interval);
            let addr = "127.0.0.1:15181"; //other address in string format

            let config = MainService::new(&dbg_id, conf_tree);
            let mut buf = Vec::with_capacity(512);

            loop {

                for (freq, amp, phi) in config.conf.signal.iter(){
                    let amplitude = amp;
                    buf.push(amplitude);

                    if (buf.capacity() == buf.len()){

                        match Self::udp_bind(addr){
                            Ok(socket) => {
                                cycle.start();
                                let header = UdpHeader::new(0, 0, 32, 255);

                                let bytes: &[u8] = unsafe {
                                    std::slice::from_raw_parts(buf.as_ptr() as *const u8, buf.len() * 4)
                                };

                                let message = UpdMessage::new(header, bytes.to_vec());

                                // &[u8]
                                socket.send(&message.message_to_bytes()).expect("mmmm");

                                buf.clear();
                                cycle.wait();
                            }
                            Err(err) => log::error!("{}.run | Udp bind error: {}", dbg_id, err),
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