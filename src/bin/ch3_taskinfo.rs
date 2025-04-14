// os/src/syscall/task.rs
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    if !check_ptr_valid(ti) {
        return -1;
    }

    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    
    unsafe {
        *ti = TaskInfo {
            status: inner.status,
            syscall_times: inner.syscall_times.clone(),
            time: (get_time_ms() - inner.start_time) as usize,
        };
    }
    
    inner.syscall_times[SYS_TASK_INFO] += 1;
    0
}

// os/src/mm/memory_set.rs
fn check_ptr_valid<T>(ptr: *const T) -> bool {
    let addr = ptr as usize;
    // 用户空间地址范围 + 对齐检查
    (0x80400000..0x88000000).contains(&addr) && addr % core::mem::align_of::<T>() == 0
}