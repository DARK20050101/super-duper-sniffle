#![no_std]
#![no_main]

use user_lib::{sys_task_info, println};

#[repr(C)]
struct TaskInfo {
    status: u32,
    syscall_times: [u32; 500],
    time: usize,
}

#[no_mangle]
pub fn main() -> i32 {
    let mut info = TaskInfo {
        status: 0,
        syscall_times: [0; 500],
        time: 0,
    };
    pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
        if !check_ptr_valid(ti) { // 检查指针是否合法
            return -1; // 非法指针，返回 -1
        }
        // ... 正常逻辑 ...
        0 // 成功返回 0
    }
    let ret = unsafe { sys_task_info(&mut info as *mut TaskInfo) };
    if ret == 0 {
        println!("Task Status: {}", info.status);
        println!("Syscall Count[410]: {}", info.syscall_times[410]);
        println!("Running Time: {} ms", info.time);
    } else {
        println!("Failed to get task info");
    }
    0
}
