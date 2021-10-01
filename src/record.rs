use std::sync::Arc;
use std::time::Duration;
use std_semaphore::Semaphore;
use std::thread;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Record {
    origin: String,
    destination: String,
    airline: String,
    package: bool,
}

pub struct RecordManager {
    record: Arc<Record>,
    sem: Arc<Semaphore>,
}

impl RecordManager {
    pub fn new(
        record: Arc<Record>,
        sem: Arc<Semaphore>,
    ) -> RecordManager {
        RecordManager {
            record,
            sem
        }
    }
    pub fn trigger_request(self) {
        self.sem.acquire();
        thread::sleep(Duration::from_millis(1_000));
        println!(
            "[Thread] Origin: {}, Destination: {}, Airline: {}, Package: {}",
            self.record.origin,
            self.record.destination,
            self.record.airline,
            self.record.package
        );
        self.sem.release();
    }
}