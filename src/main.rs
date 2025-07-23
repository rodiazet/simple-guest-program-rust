#![no_std]
#![no_main]
use core::sync::atomic::Ordering;
use portable_atomic::AtomicU16;
use portable_atomic_util::Arc;

#[panic_handler]
fn panic_impl(_panic_info: &core::panic::PanicInfo) -> ! {
     loop {}
}

struct MyStruct {
    value1 : Arc<AtomicU16>,
    value2 : Arc<AtomicU16>,
}

fn main() {

    let a: Arc<AtomicU16> = Arc::new(core::hint::black_box(AtomicU16::new(5)));
    let b: Arc<AtomicU16> = Arc::new(core::hint::black_box(AtomicU16::new(7)));

    if a.load(Ordering::SeqCst) + b.load(Ordering::SeqCst) != 12 {
        panic!("Something went wrong!");
    }

    let foo = Arc::new(MyStruct{value1: a.clone(), value2: b.clone()});

    if foo.value1.load(Ordering::SeqCst) + foo.value2.load(Ordering::SeqCst) != 12 {
        panic!("Something went wrong!");
    }
}

#[unsafe(no_mangle)]
fn _start(_argc: isize, _argv: *const *const u8) -> isize {
    main();
    return 0;
}

use core::alloc::{GlobalAlloc, Layout};

/// A simple heap allocator.
///
/// Allocates memory from left to right, without any deallocation.
struct SimpleAlloc;

unsafe impl GlobalAlloc for SimpleAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            sys_alloc_aligned(layout.size(), layout.align())
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {}
}

#[global_allocator]
static HEAP: SimpleAlloc = SimpleAlloc;

pub const MAX_MEMORY: usize = 0x78000000;
static mut HEAP_POS: usize = 0;
#[allow(clippy::missing_safety_doc)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sys_alloc_aligned(bytes: usize, align: usize) -> *mut u8 {
    unsafe extern "C" {
        // https://lld.llvm.org/ELF/linker_script.html#sections-command
        unsafe static _end: u8;
    }

    // SAFETY: Single threaded, so nothing else can touch this while we're working.
    let mut heap_pos = unsafe { HEAP_POS };

    if heap_pos == 0 {
        heap_pos = unsafe { (&_end) as *const u8 as usize };
    }

    let offset = heap_pos & (align - 1);
    if offset != 0 {
        heap_pos += align - offset;
    }

    let ptr = heap_pos as *mut u8;
    let (heap_pos, overflowed) = heap_pos.overflowing_add(bytes);

    if overflowed || MAX_MEMORY < heap_pos {
        panic!("Memory limit exceeded (0x78000000)");
    }

    unsafe { HEAP_POS = heap_pos };
    ptr
}
