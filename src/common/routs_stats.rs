use crate::rout_info::RoutInfo;

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
    pub fn new() -> RoutsStats {
        let routs: Vec<RoutInfo> = Vec::new();
        RoutsStats {
            routs,
            stopped: false,
        }
    }

    pub fn add(&mut self, origin: String, destination: String) {
        let rout = format!("{}-{}", origin, destination);
        let result = self.routs.iter().position(|r| r.rout == rout);
        match result {
            Some(i) => self.routs[i] = RoutInfo::new(rout, self.routs[i].number_of_trips + 1),
            None => self.routs.push(RoutInfo::new(rout, 1)),
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

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
