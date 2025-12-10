mod routes;
mod setup;

pub mod domain;
pub mod telemetry;

pub use setup::{Settings, get_configuration, get_database, make_server};
