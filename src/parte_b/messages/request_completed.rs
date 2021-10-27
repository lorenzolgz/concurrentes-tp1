extern crate actix;

use actix::Message;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RequestCompleted {
    pub(crate) time_elapsed: Duration,
}
