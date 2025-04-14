/// 系统调用：获取当前任务的信息
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    // 检查指针是否为空
    if ti.is_null() {
        return -1; // 返回错误
    }

    // 获取当前任务的控制块
    let current_task = task::current_task().unwrap(); // 获取当前任务
    let mut inner = current_task.inner_exclusive_access();

    // 填充 TaskInfo 结构
    let info = TaskInfo {
        status: TaskStatus::Running, // 当前任务状态一定是 Running
        syscall_times: inner.syscall_count.clone(), // 克隆系统调用计数数组
        time: get_time() - inner.first_scheduled_time, // 计算任务运行时间
    };

    // 将数据写入用户提供的缓冲区
    unsafe {
        *ti = info;
    }

    // 记录本次系统调用的计数
    inner.syscall_count[410] += 1;

    0 // 成功返回 0
}
