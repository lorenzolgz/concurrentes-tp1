extern crate actix;

use crate::messages::entry::Entry;
use actix::Message;
use std::sync::Arc;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct AeroSuccess {
    pub(crate) original_message: Arc<Entry>,
    pub(crate) elapsed_time: Option<Duration>,
}

impl AeroSuccess {
    pub fn describe(&self) -> String {
        self.original_message.describe()
    }
}
