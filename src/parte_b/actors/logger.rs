extern crate actix;
use crate::messages::log_message::LogMessage;
use actix::{Actor, Context, Handler};
use std::fs::File;
use std::io::Write;

pub struct Logger {
    pub(crate) file: File,
}

impl Actor for Logger {
    type Context = Context<Self>;
}

impl Handler<LogMessage> for Logger {
    type Result = ();
    fn handle(&mut self, msg: LogMessage, _ctx: &mut Context<Self>) -> Self::Result {
        println!("{}", msg.log_entry);
        let time_stamp = chrono::offset::Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string();
        let log_info = format!("[{}] {}\n", time_stamp, msg.log_entry);
        self.file
            .write_all(log_info.as_ref())
            .expect("Error writing to logger file");
    }
}
