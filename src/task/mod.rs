use lazy_static::lazy_static;
use crate::task::{TaskControlBlock, TaskContext, TaskStatus};
use crate::sync::UPSafeCell;
use crate::trap::TrapContext;
use crate::config::MAX_APP_NUM;
use crate::sbi::shutdown;

/// 任务管理器结构体
pub struct TaskManager {
    num_app: usize, // 总任务数量
    inner: UPSafeCell<TaskManagerInner>,
}

/// 任务管理器内部结构
struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM], // 所有任务的控制块
    current_task: usize,                    // 当前任务索引
}

lazy_static! {
    /// 全局任务管理器实例
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock::default(); MAX_APP_NUM];
        for (i, t) in tasks.iter_mut().enumerate().take(num_app) {
            t.task_cx = TaskContext::goto_restore(init_app_cx(i)); // 初始化任务上下文
            t.task_status = TaskStatus::Ready; // 设置任务状态为 Ready
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0, // 初始任务索引
                })
            },
        }
    };
}

impl TaskManager {
    /// 标记当前任务为挂起状态
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready; // 设置任务状态为 Ready
    }

    /// 标记当前任务为已退出状态
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited; // 设置任务状态为 Exited
    }

    /// 执行下一个任务
    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running; // 设置下一个任务为 Running
            inner.current_task = next;

            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);

            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr); // 执行任务切换
            }
        } else {
            // 所有任务完成时的处理
            self.handle_all_tasks_completed();
        }
    }

    /// 查找下一个可运行的任务
    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app) // 循环遍历任务
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    /// 运行第一个任务
    pub fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);

        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr); // 切换到第一个任务
        }
        panic!("unreachable in run_first_task!");
    }

    /// 所有任务完成后的处理
    fn handle_all_tasks_completed(&self) -> ! {
        println!("[kernel] All applications completed!");
        shutdown(); // 调用关机函数
    }
}

/// 挂起当前任务并运行下一个任务
pub fn suspend_current_and_run_next() {
    TASK_MANAGER.mark_current_suspended();
    TASK_MANAGER.run_next_task();
}

/// 退出当前任务并运行下一个任务
pub fn exit_current_and_run_next() {
    TASK_MANAGER.mark_current_exited();
    TASK_MANAGER.run_next_task();
}
pub struct TaskControlBlock {
    ...    
pub syscall_count: [u32; MAX_SYSCALL_NUM], // 系统调用计数    
pub first_scheduled_time: usize,          // 第一次被调度的时间戳
    ...
}
impl TaskControlBlock {
    pub fn new(...) -> Self {
        ...
        TaskControlBlock {
            ...
            syscall_count: [0; MAX_SYSCALL_NUM], // 初始化系统调用计数
            first_scheduled_time: 0,            // 初始化调度时间戳
        }
    }
}
#[repr(C)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub time: usize,
}

#[derive(Clone, Copy)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
