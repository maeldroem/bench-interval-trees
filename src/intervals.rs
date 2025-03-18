use std::ops::{Deref, RangeInclusive};

pub struct Interval(RangeInclusive<u64>);

impl Deref for Interval {
    type Target = RangeInclusive<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Interval {
    pub fn new_rand() -> Self {
        let from = rand::random::<u64>();
        let to = from.saturating_add(rand::random::<u64>());
        Interval(from..=to)
    }

    pub fn new_rand_lots(n: u64) -> Vec<Self> {
        (0..n).map(|_x| Self::new_rand()).collect::<_>()
    }
}
