mod record_manager;
mod record_manager_factory;

use std::sync::Arc;
use std::{fs, thread};
use crate::record_manager::{Record, RecordManager};
use crate::record_manager_factory::RecordManagerFactory;

fn main() -> Result<(), csv::Error> {
    let mut reservations = vec![];

    let csv = fs::read_to_string("./src/reservations.csv")
        .expect("Something went wrong reading the file");

    println!("[Main] reservations.csv: {}", csv);
    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let manager_factory = RecordManagerFactory::new();

    for record in reader.deserialize() {
        let record: Record = record?;
        let manager: RecordManager = (&manager_factory).get_manager(Arc::from(record));
        reservations.push(thread::spawn(move||manager.trigger_request()));
    }

    for reservation in reservations {
        reservation.join().expect("Unable to join on the associated thread");
    }

    Ok(())
}