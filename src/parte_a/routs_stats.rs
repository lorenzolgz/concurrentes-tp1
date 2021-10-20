pub struct RoutInfo {
    pub(crate) rout: String,
    pub(crate) number_of_trips: i32
}

pub struct RoutsStats {
    routs: Vec<RoutInfo>
}

impl RoutInfo {
    pub fn  new(rout: String, number_of_trips: i32) -> RoutInfo {
        RoutInfo {rout, number_of_trips}
    }
}

impl RoutsStats {
    pub fn  new() -> RoutsStats {
        let routs:Vec<RoutInfo> = Vec::new();
        RoutsStats {routs}
    }

    pub fn add(&mut self, rout: String) {
        let result = self.routs.iter().position(|r| r.rout == rout);
        match result {
            Some(i) => self.routs[i] = RoutInfo::new(rout, self.routs[i].number_of_trips + 1),
            None => self.routs.push(RoutInfo::new(rout, 1)),
        }
    }

    pub fn top10(&mut self) -> Vec<&RoutInfo> {
        let mut result = Vec::new();
        let mut result_len = 10;

        self.routs.sort_by_key(|r| -r.number_of_trips);

        if self.routs.len() < result_len {
            result_len = self.routs.len();
        }

        for index in 0..result_len  {
            result.push(&self.routs[index]);
        }
        return result;
    }
}
