use chrono;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

pub struct Logger {
    file: File,
    log_receive: Receiver<String>,
}

impl Logger {
    pub fn new(log_receive: Receiver<String>) -> Logger {
        let file_name = format!("./logs/{}.txt", chrono::offset::Local::now().to_string());
        let file = File::create(file_name).expect("Error creating logger file");

        Logger { file, log_receive }
    }

    pub fn run(&mut self) {
        let mut stop = false;

        while !stop {
            let text = self
                .log_receive
                .recv()
                .expect("Error receiving msg in Logger");

            if text == "STOP" {
                stop = true;
            } else {
                self.file
                    .write(text.as_ref())
                    .expect("Error writing to logger file");
            }
        }
    }
}

pub fn log_info(msg: String, log_send: Sender<String>) {
    let time_stamp = chrono::offset::Local::now().to_string();
    let log_info = format!("[INFO] {}: {}\n", time_stamp, msg);
    log_send
        .send(log_info)
        .expect("Error sending info msg to logger");
}

pub fn log_start() -> (JoinHandle<()>, Sender<String>) {
    let (log_send, log_receive) = mpsc::channel();

    let mut log = Logger::new(log_receive);

    let logger_handle = thread::spawn(move || log.run());

    return (logger_handle, log_send);
}

pub fn log_stop(log_send: Sender<String>, logger_handle: JoinHandle<()>) {
    let msg = format!("STOP");
    log_send
        .send(msg)
        .expect("Error sending STOP msg to logger");
    logger_handle.join().expect("Error joining logger_handle");
}
