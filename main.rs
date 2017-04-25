#![feature(lang_items)]
#![crate_type = "staticlib"]
#![no_std]

extern {
    fn uart_init();
    fn uart_puts(data: &'static str);
}

#[no_mangle]
pub extern fn kernel_main() {
    unsafe {uart_init();}
    unsafe {uart_puts("Hello, kernel World!\r\n");}

    loop{}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0 () {}
