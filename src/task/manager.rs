
pub fn run_next_task(&self) {
    if let Some(next) = self.find_next_task() {
        let mut inner = self.inner_exclusive_access();
        let current = inner.current_task;
        inner.tasks[next].task_status = TaskStatus::Running; // 设置下一个任务为运行状态
        inner.current_task = next;

        let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
        let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
        drop(inner);

        unsafe {
            __switch(current_task_cx_ptr, next_task_cx_ptr); // 切换到下一个任务
        }
    } else {
        // 所有任务完成后的处理逻辑
        self.handle_all_tasks_completed();
    }
}

/// 所有任务完成后的处理
fn handle_all_tasks_completed(&self) -> ! {
    println!("[kernel] All applications completed!");
    shutdown(); // 调用关机函数
}  impl TaskManager {
    /// 所有任务完成后的处理
    fn handle_all_tasks_completed(&self) -> ! {
        println!("[kernel] All applications completed!");
        shutdown(); // 调用关机函数
    }
}impl TaskManager {
    /// 标记当前任务为挂起状态
    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        println!("[kernel] Suspending task {}", current);
        inner.tasks[current].task_status = TaskStatus::Ready; // 设置任务状态为 Ready
    }

    /// 标记当前任务为已退出状态
    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        println!("[kernel] Exiting task {}", current);
        inner.tasks[current].task_status = TaskStatus::Exited; // 设置任务状态为 Exited
    }
