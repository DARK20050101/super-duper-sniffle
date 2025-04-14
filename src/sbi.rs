pub fn shutdown() -> ! {
    println!("[kernel] System is shutting down...");
    sbi_shutdown(); // 调用 SBI 的关机功能
    panic!("It should not reach here!"); // 如果关机失败，触发 panic 以进行调试
}

fn sbi_shutdown() -> ! {
    unsafe {
        sbi_call(SBI_SHUTDOWN, 0, 0, 0); // 调用 RISC-V 的 SBI SHUTDOWN 功能
    }
    panic!("SBI shutdown failed!"); // 如果关机失败，打印错误信息
}

const SBI_SHUTDOWN: usize = 8; // 定义 SBI SHUTDOWN 的编号

/// 调用 SBI 的通用接口
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        asm!(
            "ecall",
            in("a7") which, // SBI 功能编号
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            lateout("a0") ret,
        );
    }
    ret
}