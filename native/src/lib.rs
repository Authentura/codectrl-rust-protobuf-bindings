/// This is the module to emulate the structure of the protobuf file imports to
/// prevent any build issues.

pub mod data {
    pub mod backtrace_data {
        tonic::include_proto!("codectrl.data.backtrace_data");
    }
    pub mod log {
        tonic::include_proto!("codectrl.data.log");
    }

    pub use backtrace_data::*;
    pub use log::*;
}

pub mod logs_service {
    use uuid::Uuid;

    tonic::include_proto!("codectrl.logs_service");

    #[cfg(not(target_arch = "wasm32"))]
    pub use log_server_server::{
        LogServer as LogServerTrait, LogServerServer as LogServerService,
    };

    #[cfg(not(target_arch = "wasm32"))]
    pub use log_client_server::{
        LogClient as LogClientTrait, LogClientServer as LogClientService,
    };

    pub use log_client_client::LogClientClient as LoggerClient;

    impl Connection {
        pub fn new() -> Self {
            Self {
                uuid: Uuid::new_v4().hyphenated().to_string(),
            }
        }
    }
}
