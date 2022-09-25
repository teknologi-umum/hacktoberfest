pub mod healthcheck;
pub mod repositories;
pub mod metrics;

pub use healthcheck::*;
pub use repositories::*;
pub use metrics::*;

pub fn init() {
    metrics::init_collector();
}