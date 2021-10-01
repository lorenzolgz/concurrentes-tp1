mod record;

use std::sync::Arc;
use std_semaphore::Semaphore;
use std::{fs, thread};
use crate::record::{RecordManager, Record};

fn main() -> Result<(), csv::Error> {
    let mut reservations = vec![];
    let mut managers = vec![];
    let parallel_requests_count = 5;

    let csv = fs::read_to_string("./src/reservations.csv")
        .expect("Something went wrong reading the file");

    println!("[Main] reservations.csv: {}", csv);

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    let sem = Arc::new(Semaphore::new(parallel_requests_count));

    for record in reader.deserialize() {
        let record: Record = record?;
        managers.push(RecordManager::new(Arc::from(record), sem.clone()));
    }

    for manager in managers {
        reservations.push(thread::spawn(move||manager.trigger_request()));
    }

    for reservation in reservations {
        reservation.join().unwrap();
    }

    Ok(())
}