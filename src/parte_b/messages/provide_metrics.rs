extern crate actix;

use actix::Message;

/// A message representation of a Metrics Request, all required info is in
/// the actor's own state (see Benchmark)
#[derive(Message)]
#[rtype(result = "()")]
pub struct ProvideMetrics {}
