#[cfg(test)]

mod main_service {
    use std::{fs, net::UdpSocket, sync::{Arc, Mutex, Once}, thread, time::Duration};
    use sal_sync::services::service::service::Service;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::main_service::{main_service::MainService, main_service_config::MainServiceConf, udp_message::UpdMessage};
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing such functionality / behavior
    #[test]
    fn run() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(5));
        test_duration.run().unwrap();
        let target_count = 12;
        let dbgid = "test";
        let path = "src/tests/main_service_config.yaml";
        let yaml_file = fs::read_to_string(path)
            .expect(&format!("{} | Failed to read yaml file '{}'", dbgid, path));
        let conf_tree: serde_yaml::Value = serde_yaml::from_str(&yaml_file).unwrap();
        let conf = MainServiceConf::from_yaml(dbgid, &conf_tree);
        let main_service = Arc::new(Mutex::new(MainService::new(conf)));
        match main_service.clone().lock().unwrap().run(){
            Ok(_) => {
                let client = UdpSocket::bind("127.0.0.1:1234").unwrap();
                let client_handle = thread::spawn(move  || {
                    let mut received = 0;
                    while received < target_count {
                        let mut buffer = [0; 1024];
                        match client.recv(&mut buffer) {
                            Ok(_) => {
                                received += 1;
                                log::debug!("{} | step: {}", dbgid, received);
                                let dbg_buf = if buffer.len() > 16 { format!("{:?}", buffer.get(..16).unwrap()) } else { format!("{:?}", buffer) };
                                log::debug!("{} | bytes: {:?}", dbgid, dbg_buf);
                                match UpdMessage::from_be_bytes(&buffer) {
                                    Ok(message) => {
                                        log::info!("{} | Received UdpMessage: {:?}", dbgid, message.build());
                                    }
                                    Err(err) => {
                                        panic!("{} | Receiver invalid message, error: {}", dbgid, err);
                                    }
                                }
                            }
                            Err(err) => {
                                panic!("{} | Recv function failed: {}", dbgid, err);
                            }
                        }
                    }
                    log::info!("{} | Client finished!", dbgid);
                });
                client_handle.join().unwrap();
            }
            Err(err) => {
                println!("{} | Failed to run MainService: {}", dbgid, err);
            }
        }
        main_service.lock().unwrap().exit();
        test_duration.exit();
    }
}
