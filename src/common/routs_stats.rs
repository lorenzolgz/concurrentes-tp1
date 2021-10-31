use crate::rout_info::RoutInfo;

/// Is a struct that both helps keep count of how many times each rout has been requested and obtain
/// the top 10 most requested routs.
pub struct RoutsStats {
    routs: Vec<RoutInfo>,
    pub stopped: bool,
}

impl Default for RoutsStats {
    fn default() -> Self {
        Self::new()
    }
}

impl RoutsStats {
    /// Creates a new RoutsStats whit the stopped attribute set to false by default.
    pub fn new() -> RoutsStats {
        let routs: Vec<RoutInfo> = Vec::new();
        RoutsStats {
            routs,
            stopped: false,
        }
    }

    /// Increases in one the number of trips of the given rout
    pub fn add(&mut self, origin: String, destination: String) {
        let rout = format!("{}-{}", origin, destination);
        let result = self.routs.iter().position(|r| r.rout == rout);
        match result {
            Some(i) => self.routs[i] = RoutInfo::new(rout, self.routs[i].number_of_trips + 1),
            None => self.routs.push(RoutInfo::new(rout, 1)),
        }
    }

    /// Sets the stopped attribute to true
    pub fn stop(&mut self) {
        self.stopped = true;
    }

    /// Returns a vector of RoutInfo of the top 10 requested routs(if the amount of routs is less
    /// than 10 it returns the number of routs that have been requested)
    pub fn build_top_10(&mut self) -> Vec<&RoutInfo> {
        let mut result = Vec::new();
        let mut result_len = 10;

        self.routs.sort_by_key(|r| -r.number_of_trips);

        if self.routs.len() < result_len {
            result_len = self.routs.len();
        }

        for index in 0..result_len {
            result.push(&self.routs[index]);
        }
        result
    }
}
