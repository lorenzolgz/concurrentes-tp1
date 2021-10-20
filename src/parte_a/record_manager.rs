use crate::record::Record;
use rand::Rng;
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::thread;
use std::time::{Duration, Instant};
use std_semaphore::Semaphore;

const AIRLINE_SERVER_SUCCESS_RATIO: f64 = 0.75;

pub struct RecordManager {
    record: Arc<Record>,
    airline_semaphore: Arc<Semaphore>,
    package_semaphore: Arc<Semaphore>,
    times: Arc<RwLock<Vec<u128>>>,
}

impl RecordManager {
    pub fn new(
        record: Arc<Record>,
        sem: Arc<Semaphore>,
        pack: Arc<Semaphore>,
        times: Arc<RwLock<Vec<u128>>>,
    ) -> RecordManager {
        RecordManager {
            record,
            airline_semaphore: sem,
            package_semaphore: pack,
            times,
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
        let now = Instant::now();

        let mut successful_request = self.trigger_request();
        while !successful_request {
            let random_millis = rand::thread_rng().gen_range(100..2_000);
            thread::sleep(Duration::from_millis(random_millis));
            successful_request = self.trigger_request()
        }

        if self.record.package {
            self.package_semaphore.acquire();

            println!("[Package request]");
            let random_millis = rand::thread_rng().gen_range(100..2_000);
            thread::sleep(Duration::from_millis(random_millis));

            self.package_semaphore.release()
        }

        if let Ok(mut times) = self.times.write() {
            times.push(now.elapsed().as_millis());
            println!("Time average: {}ms", average(times));
        }
    }
}

fn average(numbers: RwLockWriteGuard<Vec<u128>>) -> f32 {
    numbers.iter().sum::<u128>() as f32 / numbers.len() as f32
}
