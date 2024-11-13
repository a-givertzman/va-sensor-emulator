#[cfg(test)]

mod main_service {
    use std::{fs, net::UdpSocket, sync::{Arc, Mutex, Once}, thread, time::Duration};
    use sal_sync::services::service::service::Service;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::main_service::{main_service::MainService, main_service_config::MainServiceConf, udp_header::UdpHeader, udp_message::UpdMessage};
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
        let target_count = 3;
        let dbgid = "test";
        let yaml_file = fs::read_to_string("src/tests/main_service_config.yaml")
            .expect("Failed to read yaml file.");
        let conf_tree: serde_yaml::Value = serde_yaml::from_str(&yaml_file).unwrap();
        let conf = MainServiceConf::from_yaml(dbgid, &conf_tree);
        let main_service = Arc::new(Mutex::new(MainService::new(dbgid, conf)));
        match main_service.clone().lock().unwrap().run(){
            Ok(_) => {
                let client = UdpSocket::bind("127.0.0.1:1234").unwrap();
                let client_handle = thread::spawn(move  || {
                    let mut received = 0;
                    while received < target_count {
                        let mut buffer = [0; 1024];
                        match client.recv(&mut buffer) {
                            Ok(bytes) => {
                                received += 1;
                                log::debug!("{} | step: {}", dbgid, received);
                                log::debug!("{} | bytes: {:?}", dbgid, buffer);
                                match buffer {
                                    [UdpHeader::SYN, UdpHeader::ADDR, UdpHeader::TYPE, _, _, _, _, ..] => {
                                        log::debug!("{} | syn: {},  addr: {},  type: {},  size: {:?}", dbgid, buffer[0], buffer[1], buffer[2], buffer.get(3..7));
                                        let size = u32::from_be_bytes(buffer.get(3..6).unwrap().try_into().unwrap()) as usize;
                                        let header = UdpHeader::new(
                                            buffer[0],
                                            buffer[1],
                                            buffer[2],
                                            size as u32,
                                        );
                                        let data = buffer.get(6..size).unwrap().try_into().unwrap();
                                        let message = UpdMessage::new(header, data);
                                        log::info!("Received UdpMessage: {:?}", message.build());
                                    }
                                    // 0..=1024 => {
                                    //     log::info!("Received {} bytes", bytes);
                                    //     println!("Received {} bytes", bytes);
                                    //     let package = &buffer[4..bytes];  
                                    //     let header = UdpHeader::new(
                                    //         buffer[0],
                                    //         buffer[1],
                                    //         buffer[2],
                                    //         u32::from_be_bytes(buffer[3..6].try_into().unwrap()),
                                    //     );
                                    //     let message = UpdMessage::new(header, package.to_vec());
                                    //     log::info!("Received UdpMessage: {:?}", message.build());
                                    // }
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
