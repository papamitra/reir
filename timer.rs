#![feature(core_intrinsics, naked_functions)]
#![crate_type = "staticlib"]
#![no_std]
#![allow(dead_code)]

use core::intrinsics::volatile_store;
use core::intrinsics::volatile_load;

pub const ARM_TIMER_LOD: u32 = 0x2000B400;
pub const ARM_TIMER_VAL: u32 = 0x2000B404;
pub const ARM_TIMER_CTL: u32 = 0x2000B408;
pub const ARM_TIMER_CLI: u32 = 0x2000B40C;
pub const ARM_TIMER_RIS: u32 = 0x2000B410;
pub const ARM_TIMER_MIS: u32 = 0x2000B414;
pub const ARM_TIMER_RLD: u32 = 0x2000B418;
pub const ARM_TIMER_DIV: u32 = 0x2000B41C;
pub const ARM_TIMER_CNT: u32 = 0x2000B420;

pub const SYSTIMERCLO: u32 = 0x20003004;
pub const GPFSEL1: u32 = 0x20200004;
pub const GPSET0 : u32 = 0x2020001C;
pub const GPCLR0 : u32 = 0x20200028;

pub const IRQ_BASIC: u32 = 0x2000B200;
pub const IRQ_PEND1: u32 = 0x2000B204;
pub const IRQ_PEND2: u32 = 0x2000B208;
pub const IRQ_FIQ_CONTROL: u32 = 0x2000B210;
pub const IRQ_ENABLE_BASIC: u32 = 0x2000B218;
pub const IRQ_DISABLE_BASIC: u32 = 0x2000B224;
