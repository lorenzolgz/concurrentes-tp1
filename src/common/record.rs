use serde::Deserialize;

#[derive(Deserialize)]
pub struct Record {
    pub origin: String,
    pub destination: String,
    pub airline: String,
    pub package: bool,
}
