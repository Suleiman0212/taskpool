use super::{Pool, Task};
use crate::error::PoolError;
use std::{collections::VecDeque, os::unix::thread, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle};

pub type ExecutionOut = Result<Pool, PoolError>;
impl Pool {
    pub async fn execute(mut pool: Pool) -> ExecutionOut {
        let out_pool = Arc::new(Mutex::new(Pool::new()));
        let mut threads: Vec<JoinHandle<()>> = Vec::new();

        loop {
            let task = pool.next_task();
            let mut task = match task {
                Some(t) => t,
                None => break,
            };

            let out_pool = Arc::clone(&out_pool);

            let thread = tokio::spawn(async move {
                task.execute();

                let mut pool_guard = out_pool.lock().await;
                pool_guard.tasks.push(task);
            });

            threads.push(thread);
        }

        while let Some(handle) = threads.pop() {
            handle.await.map_err(|_| PoolError::Execution)?;
        }

        let out_pool = Arc::try_unwrap(out_pool)
            .map_err(|_| PoolError::Execution)?
            .into_inner();

        Ok(out_pool)
    }
}
