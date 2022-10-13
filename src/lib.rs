//! # DA15431 Rust example project
//! 
//! This project contains a simple BLE application, which can control an LED and read the die temperature

#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use core::{
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};

use da14531_sdk::allocator::Da14531Allocator;

/// The actual application code and definition of interfaces for peripheral and BLE drivers
pub mod app;
/// Glue between SDK system and application code
pub mod app_impl;
/// BLE
pub mod ble;
/// HAL for peripherals
pub mod peripherals;

/// Global allocator (Needed to use heap, eg. for `Vec<T>`)
#[global_allocator]
static ALLOCATOR: Da14531Allocator = Da14531Allocator;

/// Panic handler in debug builds
#[cfg(debug_assertions)]
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use rtt_target::rprintln;

    use crate::app_impl::app;

    rprintln!("Panic!");
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
        app().feed_watchdog();
    }
}

/// Panic handler in release builds
#[cfg(not(debug_assertions))]
#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}
