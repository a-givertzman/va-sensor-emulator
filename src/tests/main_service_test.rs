#[cfg(test)]

mod main_service {
    use std::{fs, net::UdpSocket, sync::{Arc, Mutex, Once}, thread, time::{Duration, Instant}};
    use sal_sync::services::{conf::{self, conf_tree::ConfTree}, entity::name::Name};
    use sal_sync::services::service::service::Service;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{buffer::Buffer, main_service::{self, main_service::MainService, main_service_config::MainServiceConf, udp_header::UdpHeader, udp_message::UpdMessage}};
    
    use log::LevelFilter;
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
    fn test_main_service() {
        DebugSession::init(LogLevel::Info, Backtrace::Short);
        init_once();
        init_each();
        log::debug!("");
        let self_id = "test";
        log::debug!("\n{}", self_id);
        let test_duration = TestDuration::new(self_id, Duration::from_secs(5));
        test_duration.run().unwrap();

        let target_count = 3;
        let self_id = "test";
        let yaml_file = fs::read_to_string("/home/lisa/Downloads/code/va-sensor-emulator/src/tests/main_service_config.yaml")
        .expect("Failed to read yaml file.");
        let conf_tree: serde_yaml::Value = serde_yaml::from_str(&yaml_file).unwrap();
        let conf = MainServiceConf::from_yaml(self_id, &conf_tree);
        let main_service = Arc::new(Mutex::new(MainService::new(self_id, conf)));
        match main_service.clone().lock().unwrap().run(){
            Ok(_) => {
                let client = UdpSocket::bind("127.0.0.1:1234").unwrap();
                let client_handle = thread::spawn(move  || {
                    let mut received = 0;
                    while received < target_count {
                        let mut buffer = [0; 512];
                        match client.recv(&mut buffer) {
                            Ok(bytes) => {
                                received += 1;
                                log::debug!("step: {}", received);
                                match bytes{
                                    0..=512 => {
                                        log::info!("Received {} bytes", bytes);
                                        println!("Received {} bytes", bytes);
                                        let package = &buffer[4..bytes];  
                                        let header = UdpHeader::new(buffer[0], buffer[1], buffer[2], buffer[3]);
                                        let message = UpdMessage::new(header, package.to_vec());
                                        log::info!("Received UdpMessage: {:?}", message.build());
                                    }
                                    _ => {
                                        panic!("Incorrect message: {:?}", bytes);
                                    }
                                }
                            }
                            Err(err) => {
                                panic!("Recv function failed:: {}", err);
                            }
                        }
                    }
                    log::info!("Client finished!");
                });
                client_handle.join().unwrap();
            }
            Err(err) => {
                println!("Failed to run MainService: {}", err);
            }
        }
        main_service.lock().unwrap().exit();
        test_duration.exit();
    }
}
