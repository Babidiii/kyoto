pub mod task;
pub mod executor;
pub mod waker;
pub mod timefuture;

use std::time::Duration;

use crate::{
    executor::{Executor, Spawner, new_runtime}, 
    task::Task,
    timefuture::TimerFuture
};

fn main() {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .init();

    let (mut executor, mut spawner) = new_runtime();
    spawner.spawn(my_task());

    drop(spawner);
    executor.run();
}


async fn add(a: isize, b :isize) -> isize {
    a + b 
}

async fn my_task() {
    println!("hello world");

    println!("5 + 5 = {}", add(5,5 ).await);

    println!("time future:");
    TimerFuture::new(Duration::new(2, 0)).await;

    println!("end");
}
