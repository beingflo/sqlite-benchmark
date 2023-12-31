pub struct Measurements {
    pub data: Vec<u128>,
}

impl Measurements {
    pub fn new() -> Measurements {
        Measurements { data: vec![] }
    }

    pub fn insert(&mut self, datum: u128) {
        self.data.push(datum);
    }

    pub fn get_average(&self) -> f64 {
        self.data.iter().sum::<u128>() as f64 / self.data.len() as f64
    }

    pub fn get_90_percentile(&self) -> u128 {
        let mut data = self.data.clone();
        data.sort();

        data[(data.len() as f64 * 0.9) as usize]
    }

    pub fn get_throughput(&self) -> f64 {
        1_000_000 as f64 / self.get_average()
    }

    pub fn print_results(&self) {
        println!(
            "throughput: {:.2}/s, avg: {}us, 90%: {}us",
            self.get_throughput(),
            self.get_average(),
            self.get_90_percentile(),
        );
    }
}
