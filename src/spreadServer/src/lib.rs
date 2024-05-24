pub use crate::server_request::{register, hello};
pub use crate::database::DataStructure;

pub mod server_request;

pub mod database;
mod publisher;
mod results;
