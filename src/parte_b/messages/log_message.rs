extern crate actix;

use actix::Message;

#[derive(Message)]
#[rtype(result = "()")]
pub struct LogMessage {
    pub(crate) log_entry: String,
}
