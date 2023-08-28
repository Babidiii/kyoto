use std::{
    pin::Pin,
    future::Future,
    task::{Context, Poll},
    cell::RefCell,
    sync::{Arc, mpsc::Sender}
};

use tracing::info;

pub struct Task {
    future: RefCell<Pin<Box<dyn Future<Output=()>>>>,
    task_sender : Sender<Arc<Task>>
}

impl Task {
    /// Create a new task
    pub fn new(future : impl Future<Output=()> + 'static, task_sender: Sender<Arc<Task>>) -> Self {
        info!("[Task::new]");
        Self {
            future: RefCell::new(Box::pin(future)),
            task_sender
        }
    }

    /// Indicates that the associated task is ready to make progress
    pub fn wake(self: Arc<Self>) {
        info!("[Task::wake]");
        // send the task to the excecutor
        self.task_sender.send(self.clone());
    }

    pub fn poll(&self, ctx: &mut Context) -> Poll<()>{
        info!("[Task::poll]");
        self.future.borrow_mut().as_mut().poll(ctx)
    }
}
