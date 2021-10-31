extern crate actix;

use actix::Message;

/// A message representation of a Log
#[derive(Message)]
#[rtype(result = "()")]
pub struct LogMessage {
    pub(crate) log_entry: String,
}
