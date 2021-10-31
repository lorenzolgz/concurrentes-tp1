use common::helper::{get_log_file_name, get_log_output_path};
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
        let file_name = get_log_file_name(get_log_output_path("parte_a".to_string()));
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
                    .write_all(text.as_ref())
                    .expect("Error writing to logger file");
            }
        }
    }
}

pub fn log_info(msg: String, log_send: Sender<String>) {
    println!("{}", msg);
    let time_stamp = chrono::offset::Local::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    let log_info = format!("[{}] {}\n", time_stamp, msg);
    log_send
        .send(log_info)
        .expect("Error sending info msg to logger");
}

pub fn log_start() -> (JoinHandle<()>, Sender<String>) {
    let (log_send, log_receive) = mpsc::channel();

    let mut log = Logger::new(log_receive);

    let logger_handle = thread::spawn(move || log.run());

    (logger_handle, log_send)
}

pub fn log_stop(log_send: Sender<String>, logger_handle: JoinHandle<()>) {
    let msg = "STOP".to_string();
    log_send
        .send(msg)
        .expect("Error sending STOP msg to logger");
    logger_handle.join().expect("Error joining logger_handle");
}
