extern crate actix;
use crate::messages::request_completed::RequestCompleted;
use actix::{Actor, Context, Handler};

pub struct Benchmark {
    pub(crate) finished_requests: u128,
    pub(crate) average_time: f64,
}

impl Actor for Benchmark {
    type Context = Context<Self>;
}

impl Handler<RequestCompleted> for Benchmark {
    type Result = ();
    fn handle(&mut self, msg: RequestCompleted, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "[BENCHMARKER] recibi request completed, elapsed={}, origin={}, dest={}",
            msg.time_elapsed.as_millis(),
            msg.origin,
            msg.destination
        );
        let updated_finished_count = self.finished_requests + 1;
        self.average_time = (self.average_time * (self.finished_requests as f64)
            + (msg.time_elapsed.as_millis() as f64))
            / (updated_finished_count as f64);
        self.finished_requests = updated_finished_count;
        println!("[BENCHMARKER] new average is: {}", self.average_time);
    }
}
