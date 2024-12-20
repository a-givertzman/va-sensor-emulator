use std::{io::Error, net::{ToSocketAddrs, UdpSocket}, sync::{atomic::{AtomicBool, Ordering}, mpsc::Sender, Arc}, thread, time::Duration};
use log::{info, warn};
use sal_sync::services::{entity::{
        name::Name, object::Object, point::point::Point
        // services::{
        //     services::Services,
        //     service::service::Service,
        //     service::service_handles::ServiceHandles, 
        // }, 
}, service::{service::Service, service_cycle::ServiceCycle, service_handles::ServiceHandles}};
use crate::buffer::Buffer;

use super::main_service_config::MainServiceConf;
use super::udp_header::UdpHeader;
use super::udp_message::UpdMessage;
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
    pub fn new(conf: MainServiceConf) -> Self {
        Self {
            dbg_id: conf.name.join(),
            name: conf.name.clone(),
            conf: conf.clone(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// Bind the UDP socket
    pub fn udp_bind(addr: impl ToSocketAddrs + std::fmt::Display) -> Result<UdpSocket, Error> {
        // UDP Bind 
        info!("Start binding to the: {}", addr);
        match UdpSocket::bind(&addr){
            Ok(socket) => {
                log::info!("MainService.bind | Connected to the: {}", addr);
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
    fn get_link(&mut self, _name: &str) -> Sender<Point> {
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
            let interval = match conf.sampl_freq{
                Some(duration) => duration.as_secs_f64(),
                None => 0.0,
            };
            log::info!("interval: {}", interval);
            let interval = 1.0 / (interval * conf.buf_size as f64); 
            let interval = Duration::from_secs_f64(interval);
            let mut cycle = ServiceCycle::new(&dbg_id, interval);
            let mut buf = Buffer::new(conf.buf_size as usize);
            loop {
                for (_freq, amp, _phi) in conf.signal.iter() {
                    match buf.add(*amp) {
                        Some(array) => {
                            match Self::udp_bind(addr.clone()){
                                Ok(socket) => {
                                    cycle.start();
                                    socket.connect("127.0.0.1:1234").unwrap();
                                    //socket.connect(addr.clone()).unwrap();
                                    let header = UdpHeader::new(UdpHeader::SYN, UdpHeader::ADDR, UdpHeader::TYPE, conf.buf_size);
                                    let bytes = array.iter().flat_map(|&byte| byte.to_ne_bytes()).collect();
                                    let message = UpdMessage::new(header, bytes);
                                    log::debug!("Builded UpdMessage: {:?}", message.build());
                                    match socket.send(&message.build()) {
                                        Ok(_) => {
                                            log::debug!("{}.run | Message has been sent successfully", dbg_id);
                                        }
                                        Err(err) => {
                                            log::error!("{}.run | Message send error: {}", dbg_id, err);
                                        }
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