use riscv::register::time;

pub fn get_time() -> usize {
    time::read()
}
const MICRO_PER_SEC: usize = 1_000_000;

pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}
fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize;
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us(); // 获取微秒时间
    if ts.is_null() {
        return -1; // 如果指针无效，返回错误
    }
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0 // 成功返回 0
}
#[repr(C)]
pub struct TimeVal {
    pub sec: usize,  // 秒数
    pub usec: usize, // 微秒数
}