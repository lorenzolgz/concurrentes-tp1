use std::fs::File;
use std::io::Write;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

/// A simple struct that receives messages through a channel and prints each message in a log file.
/// The log files are store in a logs directory whit the current date plus the time and also for
/// each message printed.
pub struct Logger {
    file: File,
    log_receive: Receiver<String>,
}

impl Logger {
    /// Creates a new logger and opens the log file
    pub fn new(log_receive: Receiver<String>) -> Logger {
        let file_name = format!(
            "./logs/parte_a/{}.txt",
            chrono::offset::Local::now()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        );
        let file = File::create(file_name).expect("Error creating logger file");

        Logger { file, log_receive }
    }

    /// Logger begins to receive messages true the log_receive channel until it receive a "STOP"
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

/// It sends the msg string throw the channel log_send with the format:
/// [%Y-%m-%d %H:%M:%S] msg\n
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

/// Initializes a thread in witch the Logger will run and returns the channel for sending messages
/// to the logger and the JoinHandle for the thread.
pub fn log_start() -> (JoinHandle<()>, Sender<String>) {
    let (log_send, log_receive) = mpsc::channel();

    let mut log = Logger::new(log_receive);

    let logger_handle = thread::spawn(move || log.run());

    (logger_handle, log_send)
}

/// Stops the Logger sending the stop message to the Logger and joins the thread
pub fn log_stop(log_send: Sender<String>, logger_handle: JoinHandle<()>) {
    let msg = "STOP".to_string();
    log_send
        .send(msg)
        .expect("Error sending STOP msg to logger");
    logger_handle.join().expect("Error joining logger_handle");
}
