use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use std_semaphore::Semaphore;
use std::thread;
use rand::Rng;

#[derive(Deserialize)]
pub struct Record {
    pub(crate) origin: String,
    pub(crate) destination: String,
    pub(crate) airline: String,
    pub(crate) package: bool,
}

pub struct RecordManager {
    record: Arc<Record>,
    airline_semaphore: Arc<Semaphore>,
}

impl RecordManager {
    pub fn new(
        record: Arc<Record>,
        sem: Arc<Semaphore>,
    ) -> RecordManager {
        RecordManager {
            record,
            airline_semaphore: sem
        }
    }
    pub fn trigger_request(self) {
        self.airline_semaphore.acquire();
        let random_millis = rand::thread_rng().gen_range(100..2_000);
        thread::sleep(Duration::from_millis(random_millis));
        println!(
            "[Thread] Origin: {}, Destination: {}, Airline: {}, Package: {}",
            self.record.origin,
            self.record.destination,
            self.record.airline,
            self.record.package
        );
        self.airline_semaphore.release();
    }
}
