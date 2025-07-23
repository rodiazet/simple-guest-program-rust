#![no_std]
#![no_main]
use core::panic;
use core::sync::atomic::AtomicU16;
use core::sync::atomic::Ordering;

#[panic_handler]
fn panic_impl(_panic_info: &core::panic::PanicInfo) -> ! {
     loop {}
}

fn main() {
    let a: AtomicU16 = core::hint::black_box(AtomicU16::new(5));
    let b: AtomicU16 = core::hint::black_box(AtomicU16::new(7));

    if a.load(Ordering::SeqCst) + b.load(Ordering::SeqCst) != 12 {
        panic!("Something went wrong!");
    }
}

#[unsafe(no_mangle)]
fn _start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    return 0;
}
