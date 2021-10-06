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

const AIRLINE_SERVER_SUCCESS_RATIO: f64 = 0.75;

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
    pub fn trigger_request(&self) -> bool {
        self.airline_semaphore.acquire();
        let random_millis = rand::thread_rng().gen_range(100..2_000);
        let is_success = rand::thread_rng().gen_bool(AIRLINE_SERVER_SUCCESS_RATIO);
        thread::sleep(Duration::from_millis(random_millis));
        println!(
            "[Thread] Successful: {}, Origin: {}, Destination: {}, Airline: {}, Package: {}",
            is_success,
            self.record.origin,
            self.record.destination,
            self.record.airline,
            self.record.package
        );
        self.airline_semaphore.release();
        is_success
    }

    pub fn trigger_requests_until_success(&self) {
        let mut successful_request = self.trigger_request();
        while !successful_request {
            successful_request = self.trigger_request()
        }
    }
}
