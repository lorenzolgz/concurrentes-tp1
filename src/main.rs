use std::{fs, thread};
use serde::Deserialize;
#[derive(Deserialize)]
struct Record {
    origin: String,
    destination: String,
    airline: String,
    package: bool,
}

fn main() -> Result<(), csv::Error> {
    let mut reservations = vec![];
    let csv = fs::read_to_string("./src/reservations.csv")
        .expect("Something went wrong reading the file");

    println!("[Main] reservations.csv: {}", csv);

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: Record = record?;
        reservations.push(thread::spawn(move||print_record(record)));
    }

    for reservation in reservations {
        reservation.join().unwrap();
    }

    Ok(())
}

fn print_record(record: Record) -> () {
    println!(
        "[Thread] Origin: {}, Destination: {}, Airline: {}, Package: {}",
        record.origin,
        record.destination,
        record.airline,
        record.package
    );
}
