use std::{
    future::Future, 
    task::{Context},
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc
    }
};
use tracing::{info, error};

use crate::{task::Task, waker::new_waker};


pub struct Spawner {
    task_sender: Sender<Arc<Task>>,
}

impl Spawner{
    pub fn spawn(&mut self, future : impl Future<Output=()> + 'static)  {
        info!("[Spawner::spawn] create task from future");
        // create a task that wrap the future and contain a sender in order to notify the executor
        // on wake
        let task = Task::new(future, self.task_sender.clone());

        // send the task to the executor
        info!("[Spawner::spawn] send task to executor");
        self.task_sender.send(Arc::new(task));
    }
}

pub struct Executor{
    task_receiver: Receiver<Arc<Task>>
}

impl Executor {
    /// Run the executor in order to handle tasks
    pub fn run(&mut self)  {
        info!("[Executor] start loop");
        loop{
            match self.task_receiver.recv() {
                Ok(task) => {
                    info!("[Executor] receive task");
                    info!("[Executor] creating a new waker for a task");
                    let waker = new_waker(task.clone());
                    let mut ctx = Context::from_waker(&waker);

                    info!("[Executor] polling task");
                    task.poll(&mut ctx);
                },
                Err(_e) => {
                    break
                }
            }
        }
    }
}


pub fn new_runtime() -> (Executor, Spawner) {
    let (task_sender, task_receiver) = channel();
    (Executor{ task_receiver }, Spawner{task_sender})
}
