extern crate actix;

use crate::messages::entry::Entry;
use actix::Message;
use std::sync::Arc;
use std::time::Duration;

/// A message representation of a (Successful) Aero Service Response, original message
/// is kept to leverage the flow to either the Hotel or the Benchmark
#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct AeroSuccess {
    pub(crate) original_message: Arc<Entry>,
    pub(crate) elapsed_time: Option<Duration>,
}

impl AeroSuccess {
    /// Used to describe the message across logs
    pub fn describe(&self) -> String {
        self.original_message.describe()
    }
}
