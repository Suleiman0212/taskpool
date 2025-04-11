#![allow(unused)]

use crate::error::PoolError;
use async_trait::async_trait;
use std::fmt::Debug;

pub mod execution;

type TaskT = Box<dyn Task>;
type TaskWrapped = Option<TaskT>;

pub struct Pool {
    tasks: Vec<TaskT>,
}

impl Pool {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: TaskT) {
        self.tasks.push(task);
    }

    fn next_task(&mut self) -> TaskWrapped {
        self.tasks.pop()
    }
}

pub trait Task: Send {
    fn execute(&mut self);
}
