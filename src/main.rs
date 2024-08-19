#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rc_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rc_os::println;

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rc_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rc_os::test_panic_handler(info)
}

// #[no_mangle] // don't mangle the name of this function
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
//     // this function is the entry point, since the linker looks for a function
//     // named `_start` by default
//     println!("Hello World{}", "!");
//     // panic!("Some panic message");
//
//     rc_os::init();
//
//     // x86_64::instructions::interrupts::int3();  // invoke a breakpoint exception
//
//
//     // unsafe {
//     //     *(0xdeadbeef as *mut u8) = 42;    // trigger a page fault
//     // }
//
//     // fn stack_overflow() {
//     //     stack_overflow();    // for each recursion, the return address is pushed
//     // }
//     //
//     // stack_overflow();
//
//     // let ptr = 0x20420c as *mut u8;
//
//     // unsafe { let _x = *ptr; }
//     // println!("read worked"); // read from a code page
//
//     // unsafe { *ptr = 42; }
//     // println!("write worked"); // write to a code page
//
//     // use x86_64::registers::control::Cr3;
//     //
//     // let (level_4_page_table, _) = Cr3::read();
//     // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
//
//     #[cfg(test)]
//     test_main();
//
//     println!("It did not crash!");
//     rc_os::hlt_loop();
// }

entry_point!(kernel_main);

/// Entry point for the kernel
fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello World{}", "!");
    // panic!("Some panic message");

    rc_os::init();

    // x86_64::instructions::interrupts::int3();  // invoke a breakpoint exception

    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;    // trigger a page fault
    // }

    // fn stack_overflow() {
    //     stack_overflow();    // for each recursion, the return address is pushed
    // }
    //
    // stack_overflow();

    // let ptr = 0x20420c as *mut u8;

    // unsafe { let _x = *ptr; }
    // println!("read worked"); // read from a code page

    // unsafe { *ptr = 42; }
    // println!("write worked"); // write to a code page

    // use x86_64::registers::control::Cr3;

    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    // use x86_64::{structures::paging::PageTable, VirtAddr};
    // use rc_os::memory::active_level_4_table;

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);
    //
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };
    //
    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("  L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    // use rc_os::memory;
    // use rc_os::memory::BootInfoFrameAllocator;
    // use x86_64::VirtAddr;

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];
    //
    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // map an unused page
    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // use rc_os::allocator;
    // use rc_os::memory::{self, BootInfoFrameAllocator};
    // use x86_64::VirtAddr;
    //
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };
    //
    // allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    //
    // // allocate a number on the heap
    // let heap_value = Box::new(41);
    // println!("heap_value at {:p}", heap_value);
    //
    // // create a dynamically sized vector
    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());
    //
    // // create a reference counted vector -> will be deallocated when count reaches 0
    // let reference_counted = Rc::new(vec![1, 2, 3]);
    // let cloned_reference = reference_counted.clone();
    // println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    // core::mem::drop(reference_counted);
    // println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rc_os::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
