#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use Os::allocator::HEAP_SIZE;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(Os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use Os::println;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

fn main(boot_info: &'static BootInfo) -> ! {
    use Os::allocator;
    use Os::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    Os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    loop {}
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}

#[test_case]
fn many_boxes_long_lived() {
    let long_lived = Box::new(1); // new
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1); // new
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Os::test_panic_handler(info)
}