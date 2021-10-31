use serde::Deserialize;

/// Is a struct made to represent each CSV entry
#[derive(Deserialize)]
pub struct Record {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub package: bool,
}
