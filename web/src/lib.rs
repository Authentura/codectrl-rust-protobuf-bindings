/// This is the module to emulate the structure of the protobuf file imports to
/// prevent any build issues.

pub mod data {
    pub mod backtrace_data {
        use serde::{Deserialize, Serialize};
        tonic::include_proto!("codectrl.data.backtrace_data");
    }
    pub mod log {
        use serde::{Deserialize, Serialize};
        tonic::include_proto!("codectrl.data.log");
    }

    pub use backtrace_data::*;
    pub use log::*;
}

pub mod logs_service {
    tonic::include_proto!("codectrl.logs_service");

    pub use log_client_client::LogClientClient as LoggerClient;
}
