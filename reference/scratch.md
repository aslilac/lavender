# About
This chapter is basically a scratch pad for things that I have found interesting
during the development of Lavender. It is somewhat disorganized currently, but
I will get around to cleaning it up later in development when I have a more
complete picture of how everything works together.

## Notes
- 32KB of RAM
- 96KB of VRAM
- 256KB of external addressable memory
- ARM7TDMI CPU at 16.78 MHz
  - 16-bit Thumb ISA
  - 32-bit ARM ISA
  - Switch between ISA with the BX instruction
  - Interupts and exceptions always switch to ARM, and should automatically
  switch back to Thumb on return
  - Little endian by default
  - You can only enter System mode from another privileged mode by modifying
  the mode bits of the Current Program Status Register (CPSR).
- 240x160 display
  - 15-bit RGB (5 bits depth per channel)
  - capable of displaying 512 simultaneous colors in "character mode"
  - 32,768 simultaneous colors in "bitmap mode"
- Exceptions have a priority
- Exceptions/traps have normal and high vectors, that are controlled by external signals.
- 14 hardware interupts
- While ARM can be bi-endian, it appears the Game Boy Advance only supported little endian. This would probably make things easier in some ways.

# Memory map

section name (description) | address space | total size | bus width | permissions |
-------------------------- | ------------- | ---------- | --------- | -----------
system rom/bios | 0x0000_0000-0x0000_3fff | 16kb | 32bit | --x
external ram (thumb) | 0x0200_0000-0x0203_ffff | 256kb | 16bit | rwx
internal ram (cpu embed) | 0x0300_0000-0x0300_7fff | 32kb | 32 bit | rwx
io (device registers) | 0x0400_0000-0x0400_03ff | 1kb | 16 bit | rw-
palette ram (2 palettes x 256 entries x 15 bit colors) | 0x0500_0000-0x0500_03ff | 1kb | 16 bit | rw-
vram | 0x0600_0000-0x0601_7fff | 96kb | 16 bit | r--
object attribute memory (sprite map) | 0x0700_0000-0x0700_03ff | 1kb | 32 bit | rw-
game rom (bios points execution to here) | 0x0800_0000-0x0dff_ffff | 32mb | 16 bit | r-x
game ram (basically save files) | 0x0e00_0000-0x0e00_ffff (but theorhetically could be bigger) | >= 64kb | 8 bit | rw-

> When an exception occurs, the ARM processor halts execution in a defined manner
> and begins execution at one of a number of fixed addresses in memory, known as
> the exception vectors. There is a separate vector location for each exception,
> including reset. Behavior is defined for normal running systems (see section
> A2.6) and debug events (see Chapter D3 Coprocessor 14, the Debug Coprocessor)
> An operating system installs a handler on every exception at initialization.
> Privileged operating system tasks are normally run in System mode to allow
> exceptions to occur within the operating system without state loss.
> 
> [ARM v5 Architecture Reference Manual] Page A1-4

> The ARM instruction set can be divided into six broad classes of > instruction:
> - Branch instructions
> - Data-processing instructions on page A1-7
> - Status register transfer instructions on page A1-8
> - Load and store instructions on page A1-8
> - Coprocessor instructions on page A1-10
> - Exception-generating instructions on page A1-10.

> The data-processing instructions perform calculations on the general-purpose
> registers. There are five types of data-processing instructions:
> - Arithmetic/logic instructions
> - Comparison instructions
> - Single Instruction Multiple Data (SIMD) instructions
> - Multiply instructions on page A1-8
> - Miscellaneous Data Processing instructions on page A1-8.

> Load and Store Register instructions have three primary addressing modes, all of which use a base register and an offset specified by the instruction:
> - In offset addressing, the memory address is formed by adding or subtracting an offset to or from the base register value.
> - In pre-indexed addressing, the memory address is formed in the same way as for offset addressing. As a side effect, the memory address is also written back to the base register.
> - In post-indexed addressing, the memory address is the base register value. As a side effect, an offset is added to or subtracted from the base register value and the result is written back to the base register.

B, BL, BLX, BX
Branch, and Branch with Link. See B, BL on page A4-10.
Branch with Link and Exchange. See BLX (1) on page A4-16 and BLX (2) on page A4-18. Branch and Exchange Instruction Set. See BX on page A4-20.
