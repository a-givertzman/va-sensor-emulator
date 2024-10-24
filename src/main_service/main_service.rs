use std::{io::{Error, ErrorKind}, net::{TcpStream, ToSocketAddrs, UdpSocket}, os::unix::net::SocketAddr, sync::{atomic::{AtomicBool, Ordering}, mpsc::Sender, Arc, Mutex}, thread, time::Duration};
use log::{info, warn};
use sal_sync::services::{conf::conf_tree::ConfTree, entity::{
        name::Name, object::Object, point::point::Point
        // services::{
        //     services::Services,
        //     service::service::Service,
        //     service::service_handles::ServiceHandles, 
        // }, 
}, service::{service::Service, service_cycle::ServiceCycle, service_handles::ServiceHandles}, types::type_of::TypeOf};

use crate::amplitude;

use super::main_service_config::MainServiceConf;
//
//
// impl ServiceConfig{
//     //
//     ///
//     pub fn new(conf: &serde_yaml::Value) -> Self{
//         let ampl = conf["ample"].as_i64().unwrap_or(0.0) as f64;
//         let phi = conf["phi"].as_i64().unwrap_or(0.0) as f64;
//         Self{
//             ampl,
//             phi,
//         }
//     }
// }
//
//
pub struct UdpHeader {
    pub syn: u8,
    pub addr: u8,
    pub r#type: u8,
    pub count: u8,
    pub data: [u8; 1024],
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
        let exit = self.exit.clone();
        info!("{}.run | Preparing thread...", dbg_id);
        let handle = thread::Builder::new().name(format!("{}.run", dbg_id.clone())).spawn(move || {
            let interval = 0.0; // from sampl_freq & buf_size
            let interval = Duration::from_secs_f64(interval);
            let mut cycle = ServiceCycle::new(&dbg_id, interval);
            let addr = "127.0.0.1:15181"; //other address in string format

            let parent = "";
            let conf_tree = ConfTree::new();
            let config = MainService::new(parent, conf_tree);
            for(freq, amp, phi) in config.signal{
                let amplitude = amp;
                let angle = phi;
            }
            let buf:[u8; 0] = [];
            //let header = todo!("Udp message head, find detales here: https://github.com/a-givertzman/cma-server/issues/123#issue-2478437558");
            loop {
                match Self::udp_bind(addr) {
                    Ok(socket) => {
                        cycle.start();
                        // 
                        // do the sampling actions
                        // - calc amplitude
                        // - add to buffer
                        // - if buffer is full
                        //      do the udp actions
                        //      - build data splitting buffer amplitudes into bytes
                        //      - build message using header and data
                        //      - send message to the socket
                        let bytes = buf.map(|amp: f64| {
                            let amp: i16 = amp.round() as i16;
                            let amp_bytes = amp.to_be_bytes();
                        });
                        socket.send(bytes);
                        cycle.wait();
                    }
                    Err(err) => log::error!("{}.run | Udp bind error: {}", dbg_id, err),
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