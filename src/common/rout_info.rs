pub struct RoutInfo {
    pub rout: String,
    pub number_of_trips: i32,
}

impl RoutInfo {
    pub fn new(rout: String, number_of_trips: i32) -> RoutInfo {
        RoutInfo {
            rout,
            number_of_trips,
        }
    }
}
