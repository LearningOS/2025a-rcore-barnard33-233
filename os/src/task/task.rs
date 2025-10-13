//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// syscall counter of task
    pub syscall_cnt: SyscallCounter
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// Syscall counter of task
#[derive(Copy, Clone, Default)]
pub struct SyscallCounter {
    /// number of write
    pub syscall_write: u32,
    /// number of exit
    pub syscall_exit: u32,
    /// number of yield
    pub syscall_yield: u32,
    /// number of get_time
    pub syscall_get_time: u32,
    /// number of trace
    pub syscall_trace: u32
}
