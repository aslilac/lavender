# About
This chapter is basically a scratch pad for things that I have found interesting
during the development of Lavender. It is somewhat disorganized currently, but
I will get around to cleaning it up later in development when I have a more
complete picture of how everything works together.

## Notes
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

> The ARM instruction set can be divided into six broad classes of instruction:
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

> Adding this to the contents of the PC, which contains the address of the branch instruction plus 8 bytes.
Why is the address plus 8 bytes? If they increment the PC after reading the
instruction then I understand why it might be 4 ahead, but 8 doesn't make sense.
