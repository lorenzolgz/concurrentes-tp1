extern crate actix;

use actix::Message;
use std::time::Duration;

/// A message representation of a Successfully Completed Entry (with Hotel if applicable)
/// It is used by the Benchmark to keep metrics of popular routes and response time
#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestCompleted {
    pub(crate) time_elapsed: Option<Duration>,
    pub(crate) origin: String,
    pub(crate) destination: String,
}
