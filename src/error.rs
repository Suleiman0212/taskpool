#![allow(unused)]

#[derive(Debug)]
pub enum PoolError {
    Execution,
}

impl std::fmt::Display for PoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Execution => write!(f, "Pool execution error!"),
        }
    }
}

impl std::error::Error for PoolError {}
