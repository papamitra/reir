#![feature(lang_items, asm, core_intrinsics, naked_functions)]
#![crate_type = "staticlib"]
#![no_std]

use core::intrinsics::volatile_store;
use core::intrinsics::volatile_load;

// Memory-Mapped I/O output
fn mmio_write(reg: u32, data: u32) {
    unsafe {volatile_store(reg as *mut u32,data)};
}

// Memory-Mapped I/O input
fn mmio_read(reg: u32) -> u32 {
    unsafe {
        let ret = volatile_load::<u32>(reg as *const u32);
        return ret;
    }
}

fn delay(count: u32) {
    unsafe {
        let mut c = count;
        asm!("1: subs $0, $0, #1; bne 1b\n"
             : "+r"(c)
             :
             : "cc");
    }
}

// The GPIO registers base address.
pub const GPIO_BASE: u32 = 0x3F200000;  // for raspi2 & 3, 0x20200000 for raspi1

// The offsets for reach register.

// Controls actuation of pull up/down to ALL GPIO pins.
pub const GPPUD: u32 = (GPIO_BASE + 0x94);

// Controls actuation of pull up/down for specific GPIO pin.
pub const GPPUDCLK0: u32 = (GPIO_BASE + 0x98);

// The base address for UART.
pub const UART0_BASE: u32 = 0x3F201000;  // for raspi2 & 3, 0x20201000 for raspi1

// The offsets for reach register for the UART.
pub const UART0_DR: u32 = (UART0_BASE + 0x00);
pub const UART0_RSRECR: u32 = (UART0_BASE + 0x04);
pub const UART0_FR: u32 = (UART0_BASE + 0x18);
pub const UART0_ILPR: u32 = (UART0_BASE + 0x20);
pub const UART0_IBRD: u32 = (UART0_BASE + 0x24);
pub const UART0_FBRD: u32 = (UART0_BASE + 0x28);
pub const UART0_LCRH: u32 = (UART0_BASE + 0x2C);
pub const UART0_CR: u32 = (UART0_BASE + 0x30);
pub const UART0_IFLS: u32 = (UART0_BASE + 0x34);
pub const UART0_IMSC: u32 = (UART0_BASE + 0x38);
pub const UART0_RIS: u32 = (UART0_BASE + 0x3C);
pub const UART0_MIS: u32 = (UART0_BASE + 0x40);
pub const UART0_ICR: u32 = (UART0_BASE + 0x44);
pub const UART0_DMACR: u32 = (UART0_BASE + 0x48);
pub const UART0_ITCR: u32 = (UART0_BASE + 0x80);
pub const UART0_ITIP: u32 = (UART0_BASE + 0x84);
pub const UART0_ITOP: u32 = (UART0_BASE + 0x88);
pub const UART0_TDR: u32 = (UART0_BASE + 0x8C);

fn uart_init() {
    // Disable UART0.
    mmio_write(UART0_CR, 0x00000000);
    // Setup the GPIO pin 14 && 15.

    // Disable pull up/down for all GPIO pins & delay for 150 cycles.
    mmio_write(GPPUD, 0x00000000);
    delay(150);

    // Disable pull up/down for pin 14,15 & delay for 150 cycles.
    mmio_write(GPPUDCLK0, (1 << 14) | (1 << 15));
    delay(150);

    // Write 0 to GPPUDCLK0 to make it take effect.
    mmio_write(GPPUDCLK0, 0x00000000);

    // Clear pending interrupts.
    mmio_write(UART0_ICR, 0x7FF);

    // Set integer & fractional part of baud rate.
    // Divider = UART_CLOCK/(16 * Baud)
    // Fraction part register = (Fractional part * 64) + 0.5
    // UART_CLOCK = 3000000; Baud = 115200.

    // Divider = 3000000 / (16 * 115200) = 1.627 = ~1.
    mmio_write(UART0_IBRD, 1);
    // Fractional part register = (.627 * 64) + 0.5 = 40.6 = ~40.
    mmio_write(UART0_FBRD, 40);

    // Enable FIFO & 8 bit data transmissio (1 stop bit, no parity).
    mmio_write(UART0_LCRH, (1 << 4) | (1 << 5) | (1 << 6));

    // Mask all interrupts.
    mmio_write(UART0_IMSC, (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) |
               (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10));

    // Enable UART0, receive & transfer part of UART.
    mmio_write(UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));
}

fn uart_putc(c: u8) {
    // Wait for UART to become ready to transmit.
    while (mmio_read(UART0_FR) & (1 << 5)) > 0 {
    }
    mmio_write(UART0_DR, c as u32);
}

fn uart_getc() {
    // Wait for UART to have received something.
    while (mmio_read(UART0_FR) & (1 << 4)) > 0 {
    }
    mmio_read(UART0_DR);
}

fn uart_puts(s: &'static str) {
    for c in s.as_bytes() {
        uart_putc(*c);
    }
}

#[no_mangle]
pub fn usertask() {
    uart_puts("usertask called()\r\n");
}

#[no_mangle]
pub extern fn kernel_main() {
    unsafe {uart_init();}
    unsafe {uart_puts("Hello, kernel World!\r\n");}

    //unsafe {asm!("svc #0x0000");}

    let mut usertask_stack: [u32; 256] = [0;256];
    unsafe {
        let mut stack_start = &mut usertask_stack[0] as *mut u32;
        stack_start = stack_start.offset(256-16);
        *stack_start.offset(8) = usertask as u32;
    }

    // Follow lines is not executed
    unsafe {uart_puts("Hello, kernel World! 2\r\n");}
    loop{}
}

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! { loop {} }

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr0 () {}

#[no_mangle]
pub extern fn __aeabi_unwind_cpp_pr1 () {}

#[no_mangle]
pub extern fn _svc_handler() {
    unsafe {uart_puts("call svc handler\r\n");}
}
