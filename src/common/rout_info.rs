/// Struct used to represent the amount of request for the given rout
pub struct RoutInfo {
    pub rout: String,
    pub number_of_trips: i32,
}

impl RoutInfo {
    /// Creates a new RoutInfo
    pub fn new(rout: String, number_of_trips: i32) -> RoutInfo {
        RoutInfo {
            rout,
            number_of_trips,
        }
    }
}
