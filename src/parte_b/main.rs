extern crate actix;
mod actors;
mod messages;

use crate::actors::aero_service::AeroService;
use crate::actors::benchmark::Benchmark;
use crate::actors::hotel_service::HotelService;
use crate::actors::orchestrator::Orchestrator;
use crate::messages::entry::Entry;
use actix::{Actor, SyncArbiter, System};
use common::airlines::AIRLINES;
use common::helper::get_max_requests_count;
use common::record::Record;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use std::time::SystemTime;

fn main() {
    let max_requests = get_max_requests_count() as usize;
    let csv = fs::read_to_string("./resources/reservations.csv")
        .expect("Something went wrong reading the file");

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    let system = System::new();

    system.block_on(async {
        let mut aeroservices = HashMap::new();
        for airline in AIRLINES {
            aeroservices.insert(
                airline.to_string(),
                SyncArbiter::start(max_requests, move || AeroService {
                    id: airline.to_string(),
                }),
            );
        }

        let hotel_service = SyncArbiter::start(max_requests, || HotelService {});
        let benchmark_service = Benchmark {
            finished_requests: 0,
            average_time: 0.0,
        };
        let otro_orq = Arc::from(
            Orchestrator {
                aeroservices,
                hotel: hotel_service,
                benchmark: benchmark_service.start(),
            }
            .start(),
        );
        for record in reader.deserialize() {
            let record: Record = record.expect("Unable to parse record");
            otro_orq.do_send(Entry {
                aero_id: record.airline.to_string(),
                origin: record.origin.to_string(),
                destination: record.destination.to_string(),
                includes_hotel: record.package,
                sender: Option::None,
                start_time: SystemTime::now(),
            });
        }
    });

    system.run().unwrap();
}
