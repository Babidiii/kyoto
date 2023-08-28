use std::{
    task::{Waker, RawWaker, RawWakerVTable},
    sync::{
        Arc, 
        mpsc::Sender
    }
};
use tracing::info;

use crate::Task;

const WAKER_VTABLE : RawWakerVTable = RawWakerVTable::new(
   waker_clone,
   waker_wake,
   waker_wake_by_ref,
     waker_drop
);


/// Create a new waker from a RawWaker for a Task 
pub fn new_waker(task: Arc<Task>) -> Waker {
    info!("[new_waker] -> Waker");
    let ptr = Arc::into_raw(task) as *const ();

    /// create the raw waker with the appropriate vtable
    let raw_waker = RawWaker::new(ptr, &WAKER_VTABLE);

    // create the Waker
    unsafe{
        Waker::from_raw(raw_waker)
    }
}


/// WAKER FUNCTIOONS ===============================================

/// clone the waker
fn waker_clone(ptr: *const ()) -> RawWaker {
    info!("[Waker::clone]");
    let waker_arc : Arc<Task> = unsafe{ Arc::from_raw(ptr as *const Task) };
    // we must increase the ref count
    std::mem::forget(waker_arc.clone()); 
    RawWaker::new(Arc::into_raw(waker_arc) as *const (), &WAKER_VTABLE)
}

/// Wake up the task associated
fn waker_wake(ptr: *const ()) {
    info!("[Waker::wake]");
    let waker_arc : Arc<Task> = unsafe{ Arc::from_raw(ptr as *const Task) };
    waker_arc.wake()
}

/// Wake up the task by ref
fn waker_wake_by_ref(ptr: *const ()) {
    info!("[Waker::wake_by_ref]");
}

/// Drop the waker
fn waker_drop(ptr: *const ()) {
    info!("[Waker::drop]");
    unsafe{ drop(Arc::from_raw(ptr)) }
}

