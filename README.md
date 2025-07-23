# Simple guest program in Rust

This repo provides very simple program in rust which biulds to Tier 2 rust compiler target `riscv32im-unknown-none-elf`. The program does not use `std` library because there is no support for std library for this compilation target.

## Build

Regular build:

`cargo build --release`

Build with additional [lower atomic pass](https://llvm.org/docs/Passes.html#lower-atomic-lower-atomic-intrinsics-to-non-atomic-form).

`RUSTFLAGS="-C passes=lower-atomic" cargo build --release`

The lowering atomic pass which eliminates atomic usage in the program is important for [`sp1`](https://github.com/succinctlabs/sp1/) and [`risc0`](https://github.com/risc0/risc0/) implementations as they support only `riscv32im` architecture (without A RISC-V extension).

## Compilation result

Using `objdump` command from [C++ riscv toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain) for `riscv32im` target (binary is delivered by risc0 project) we are able to display assembler contents of all sections of the binary.

`objdump -D target/riscv32im-unknown-none-elf/release/simple-guest-program-rust`

Comparision of two assemblers (w and w/o `passes=lower-atomic` flag usage ) can be found in [objdump.txt](./objdump.txt) and [objdump-la.txt](./objdump-la.txt).