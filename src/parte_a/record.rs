use serde::Deserialize;

#[derive(Deserialize)]
pub struct Record {
    pub(crate) origin: String,
    pub(crate) destination: String,
    pub(crate) airline: String,
    pub(crate) package: bool,
}
