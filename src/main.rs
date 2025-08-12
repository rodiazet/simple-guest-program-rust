#![no_std]
#![no_main]
extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicU16;

#[cfg(feature = "sp1-guest")]
mod sp1;
#[cfg(feature = "risc0-guest")]
mod risc0;

fn main() {
    let a: AtomicU16 = core::hint::black_box(AtomicU16::new(5));
    let b: AtomicU16 = core::hint::black_box(AtomicU16::new(7));

    if a.load(Ordering::SeqCst) + b.load(Ordering::SeqCst) != 12 {
        panic!("Something went wrong!");
    }

    let mut v: Vec<AtomicU16> = Vec::new();
    v.push(AtomicU16::new(5));
    v.push(AtomicU16::new(7));

    if v[0].load(Ordering::SeqCst) + v[1].load(Ordering::SeqCst) != 12 {
        panic!("Something went wrong!");
    }
}



