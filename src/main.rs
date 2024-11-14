mod buffer;
mod angle;
mod amplitude;
mod stuff;
mod complex;
mod service;
mod conf;
mod main_service;
mod error;
mod debug;

#[cfg(test)]
mod tests;

use debug::dbg_id::DbgId;
use main_service::{main_service::MainService, main_service_config::MainServiceConf};
use sal_sync::services::service::service::Service;
//
//
fn main() {
    let dbgid = DbgId("main".into());
    let path = "config.yaml";
    let conf = MainServiceConf::read(&dbgid, path);
    let mut main_service = MainService::new(conf);
    match main_service.run() {
        Ok(handles) => for (_, h) in handles {
            h.join().unwrap();
        }
        Err(err) => {
            log::error!("{} | Error: {:#?}", dbgid, err);
        }
    }
}
