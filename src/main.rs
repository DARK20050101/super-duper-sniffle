#[no_mangle]
 pub fn rust_main() -> ! {
     // ...
     trap::enable_timer_interrupt();
     timer::set_next_trigger();
     // ...
 }
