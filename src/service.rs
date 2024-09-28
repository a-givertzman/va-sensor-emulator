use std::{sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}, mpsc::Sender}, thread};
use log::{info, warn};
use crate::{
    services::entity::{
        object::Object, point::point::Point, name::Name,
    }, 
    conf::ServiceNameConfig,
    services::{
        services::Services,
        service::service::Service,
        service::service_handles::ServiceHandles, 
    }, 
};
//
//
pub struct ServiceConfig{
    ampl: f64,
    phi: f64,
}
//
//
impl ServiceConfig{
    //
    ///
    pub fn new(conf: &serde_yaml::Value) -> Self{
        let ampl = conf["ample"].as_i64().unwrap_or(0.0) as f64;
        let phi = conf["phi"].as_i64().unwrap_or(0.0) as f64;
        Self{
            ampl,
            phi,
        }
    }
}
//
//
pub struct ServiceName{
    id: String,
    name: Name,
    conf: ServiceNameConfig,
    services: Arc<Mutex<Services>>,
    exit: Arc<AtomicBool>,
}
impl ServiceName {
    //
    /// Crteates new instance of the ServiceName 
    pub fn new(parent: impl Into<String>, conf: ServiceNameConfig, services: Arc<Mutex<Services>>) -> Self {
        Self {
            id: conf.name.join(),
            name: conf.name,
            conf: conf.clone(),
            services,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
}
impl Object for ServiceName {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn name(&self) -> Name {
        self.name.clone()
    }
}
//
// 
impl std::fmt::Debug for ServiceName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ServiceName")
            .field("id", &self.id)
            .finish()
    }
}
//
// 
impl Service for ServiceName {
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
    fn run(&mut self) -> Result<ServiceHandles, String> {
        info!("{}.run | Starting...", self.id);
        let self_id = self.id.clone();
        let exit = self.exit.clone();
        info!("{}.run | Preparing thread...", self_id);
        let handle = thread::Builder::new().name(format!("{}.run", self_id.clone())).spawn(move || {
            loop {
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
        });
        match handle {
            Ok(handle) => {
                info!("{}.run | Starting - ok", self.id);
                Ok(ServiceHandles::new(vec![(self.id.clone(), handle)]))
            }
            Err(err) => {
                let message = format!("{}.run | Start failed: {:#?}", self.id, err);
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