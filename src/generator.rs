use rand::distr::Alphanumeric;
use rand::prelude::*;

pub struct StringGenerator {
    rng: ThreadRng,
}

impl StringGenerator {
    pub fn new() -> Self {
        Self { rng: rand::rng() }
    }
}

impl Iterator for StringGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let length = self.rng.random_range(1..=100);
        let s: String = (&mut self.rng)
            .sample_iter(Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
        Some(s)
    }
}
