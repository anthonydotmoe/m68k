# m68k

A work in progress copy of the
[cortex-m](https://github.com/rust-embedded/cortex-m) crate for use with my
[68040pc](https://github.com/anthonydotmoe/68040pc) computer project.

## Status

I believe this won't work with mainline Rust. If I do anything that starts
getting stuff done then `rustc` [crashes](https://github.com/rust-lang/rust/issues/115886#issuecomment-1722200801).

I did have luck making something before implementing critical_section:

1. Install Rust nightly and `rust-src`
1. Install [m68k-elf-gcc](https://aur.archlinux.org/packages/m68k-elf-gcc)
1. Try to build minimal program: `cargo build -p m68k-rt --examples --release`
1. Inspect the output file: `m68k-elf-objdump -d target/m68k-unknown-none/release/examples/minimal`
1. Success!

```
/target/m68k-unknown-none/release/examples/minimal:     file format elf32-m68k


Disassembly of section .text:

00000040 <Reset>:
  40:   6000 0018       braw 5a <DefaultPreInit>
  44:   4eb9 0000 004c  jsr 4c <main>
  4a:   4afc            illegal

0000004c <main>:
  4c:   23fc 0000 01a4  movel #420,deadbeef <_ram_end+0xbeacbeef>
  52:   dead beef 
  56:   60fe            bras 56 <main+0xa>

00000058 <DefaultHandler_>:
  58:   60fe            bras 58 <DefaultHandler_>

0000005a <DefaultPreInit>:
  5a:   4e75            rts
```

## Problems

- `rustc` crashes with `SIGILL` when:
  - Building with M68040 CPU type
  - Building m68k
- Can't build in debug mode: The linker can't find a panic/unroll routine.