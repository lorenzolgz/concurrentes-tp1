use std::sync::Arc;
use std::time::Duration;
use std_semaphore::Semaphore;
use std::thread;
use serde::Deserialize;
use rand::Rng;

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
        let random_millis = rand::thread_rng().gen_range(100..2_000);
        thread::sleep(Duration::from_millis(random_millis));
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