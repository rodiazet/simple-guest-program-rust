# Simple guest program in Rust

This repo provides very simple program in rust which biulds to Tier 2 rust compiler target `riscv32im-unknown-none-elf`. The program does not use `std` library because there is no support for std library for this compilation target. 

This program can be executed on zkvm sp1.

## Build

Rust compiler:

```
name: stable-aarch64-apple-darwin
active because: it's the default toolchain
installed targets:
  aarch64-apple-darwin
  riscv32im-unknown-none-elf
  
cargo 1.88.0 (873a06493 2025-05-10)
```

Build:

`cargo build --release`

## Run on sp1

Compiled program can be executed (proved?. To be verified) on sp1 zkvm. An example which loads and execute arbitrary program like this can be found in [sp1 repo fork](https://github.com/rodiazet/sp1/tree/general-elf/examples/general-elf/script) which onyl extends example set for this zkvm. It does not provide any additional changes to sp1 implemention.

Folow steps below to execute this program:
1. Build `simple-guest-program-rust` with command `cargo build --release`
2. Copy absolute path to result elf located in `target/riscv32im-unknown-none-elf/release/simple-guest-program-rust`
3. Go to directory `sp1/tree/general-elf/examples/general-elf/script`
4. Run `cargo run --release <path-to-copied-elf-file>`

Expected output:
```
executed: gas: 804852
opcode counts (30 total instructions):
    13 add
     4 auipc
     3 jalr
     2 lh
     2 sh
     1 sll
     1 srl
     1 lw
     1 sw
     1 bne
     1 ecall
syscall counts (1 total syscall instructions):
    1 halt
```