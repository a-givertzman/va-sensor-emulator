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
use super::udp_header::UdpHeader;
use super::udp_message::UpdMessage;
use crate::Buffer;
///
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
    ///
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
        let exit = self.exit.clone();
        let conf = self.conf.clone();
        let addr = self.conf.addr.clone();
        info!("{}.run | Preparing thread...", dbg_id);
        let handle = thread::Builder::new().name(format!("{}.run", dbg_id.clone())).spawn(move || {
            let interval = 0.0; // from sampl_freq & buf_size
            let interval = Duration::from_secs_f64(interval);
            let mut cycle = ServiceCycle::new(&dbg_id, interval);
            let mut buf = Buffer::new(512);

            loop {
                for (freq, amp, phi) in conf.signal.iter(){
                    match buf.add(*amp){
                        Some(array) =>{
                            match Self::udp_bind(addr.clone()){
                                Ok(socket) => {
                                    cycle.start();
                                    let header = UdpHeader::new(UdpHeader::SYN, UdpHeader::ADDR, UdpHeader::TYPE, UdpHeader::COUNT);
                                    let bytes = array.iter().flat_map(|&byte|byte.to_ne_bytes()).collect();
                                    let message = UpdMessage::new(header, bytes);
                                    match socket.send(&message.build()){
                                        Ok(_) => {
                                            log::debug!("Message has been sent successfully")
                                        },
                                        Err(e) => {
                                            log::error!("{}.run | Message send error: {}", dbg_id, e)
                                        },
                                    }
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