# Technical Specifications

- Interupts and exceptions always switch to ARM, and should automatically
switch back to Thumb on return
- Little endian by default
- You can only enter System mode from another privileged mode by modifying
the mode bits of the Current Program Status Register (CPSR).
- Exceptions have a priority
- Exceptions/traps have normal and high vectors, that are controlled by external signals.
- 14 hardware interupts
- While ARM can be bi-endian, it appears the Game Boy Advance only supported
little endian. This would probably make things easier in some ways.

## Processor and memory
It uses a 32-bit ARM7TDMI processor, which is built on the ARM v4T architecture.
It has 32KB of embedded RAM, 96KB of VRAM, and can address 256KB of RAM on the
cartridge. The T in v4T means that the processor has two instruction sets. The
ARM instruction set uses 32-bit word-aligned instructions, while the Thumb
instruction set uses 16-bit halfword-aligned instructions.

**Notes about Thumb:** Both instruction sets follow the same execution path
inside of the processor, and share a single implementation. Only the process of
decoding the instructions is different between the two. Everything else is
identical. The Thumb instruction set is useful when storing a program in an
area of memory that only has a 16-bit bus width.

## Display
The display has a resolution of 240x160 with a refresh rate of 59.73Hz. It uses
a 15-bit color encoding (with 32,768 possible colors) and 5:5:5 chroma subsampling.

## Sound
The sound hardware of the Game Boy Advance is really interesting, but I don't
know enough to fill this section out all the way right now.
