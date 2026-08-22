pub mod cli;
pub mod input;

pub use cli::{Dispatch, run};
pub use input::{MetricInput, MetricInputConsumer, discover_inputs};
