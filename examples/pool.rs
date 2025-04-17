#![allow(unused)]

use std::time::Instant;
use taskpool::pool::*;

#[derive(Debug)]
pub struct PrimeTask {
    pub num: u64,
    pub is_prime: Option<bool>,
}

impl PrimeTask {
    fn new(num: u64) -> Box<Self> {
        Box::new(Self {
            num,
            is_prime: None,
        })
    }
}

impl Task for PrimeTask {
    fn execute(&mut self) {
        self.is_prime = Some(is_prime(self.num));
    }
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    let limit = (n as f64).sqrt() as u64;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[tokio::main]
async fn main() {
    let mut pool = Pool::new();
    for i in 0..1000000 {
        let task = PrimeTask::new(i);
        pool.add_task(task);
    }

    Pool::execute(pool).await.unwrap();
}
