use crate::record::Record;
use rand::Rng;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};
use std_semaphore::Semaphore;
use crate::logger::log_info;


const AIRLINE_SERVER_SUCCESS_RATIO: f64 = 0.75;

pub struct RecordManager {
    record: Arc<Record>,
    airline_semaphore: Arc<Semaphore>,
    package_semaphore: Arc<Semaphore>,
    times: Arc<RwLock<Vec<u128>>>,
    log_send: Sender<String>,
}

impl RecordManager {
    pub fn new(
        record: Arc<Record>,
        sem: Arc<Semaphore>,
        pack: Arc<Semaphore>,
        times: Arc<RwLock<Vec<u128>>>,
        log_send: Sender<String>,
    ) -> RecordManager {
        RecordManager {
            record,
            airline_semaphore: sem,
            package_semaphore: pack,
            times,
            log_send,
        }
    }

    pub fn trigger_request(&self) -> bool {
        self.airline_semaphore.acquire();
        let random_millis = rand::thread_rng().gen_range(100..2_000);
        let is_success = rand::thread_rng().gen_bool(AIRLINE_SERVER_SUCCESS_RATIO);
        thread::sleep(Duration::from_millis(random_millis));
        log_info(
            format!("[{:?}] Successful: {}, Origin: {}, Destination: {}, Airline: {}, Package: {}",
                    thread::current().id(),
                    is_success,
                    self.record.origin,
                    self.record.destination,
                    self.record.airline,
                    self.record.package),
            self.log_send.clone()
        );
        self.airline_semaphore.release();
        is_success
    }

    pub fn trigger_requests_until_success(&self) {
        let now = Instant::now();

        let mut successful_request = self.trigger_request();
        while !successful_request {
            thread::sleep(Duration::from_millis(2000));
            successful_request = self.trigger_request()
        }

        if self.record.package {
            self.package_semaphore.acquire();

            log_info(format!("[{:?}] Package request", thread::current().id()), self.log_send.clone());
            let random_millis = rand::thread_rng().gen_range(100..2_000);
            thread::sleep(Duration::from_millis(random_millis));

            self.package_semaphore.release()
        }

        if let Ok(mut times) = self.times.write() {
            times.push(now.elapsed().as_millis());
            log_info(
                format!("[{:?}] Request time average: {}ms", thread::current().id(), average(times)),
                self.log_send.clone()
            );
        }
    }
}

fn average(numbers: RwLockWriteGuard<Vec<u128>>) -> f32 {
    numbers.iter().sum::<u128>() as f32 / numbers.len() as f32
}
