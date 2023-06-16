# CPU Architecture

The CPSR register:

-   n: Negative or less than
-   z: Zero
-   c: Carry, borrow, extend
-   v: Overflow

The v4T architecture has 49 32-bit ARM instructions and 35 16-bit Thumb
instructions.

That may sound like a lot, but all of the Thumb instructions have direct
translations to the ARM instructions, so you really only have to program 49
instructions and a Thumb translator. The instructions have a lot of overlapping
complexity as well, so the first few are the hardest. Once you've got a couple
of them implemented the rest are actually pretty easy.

The ARM instructions are

-   adc
-   add
-   and
-   b
-   bic
-   bl
-   bx
-   cdp
-   cmn
-   cmp
-   eor
-   ldc
-   ldm
-   ldr
-   ldrb
-   ldrbt
-   ldrh
-   ldrsb
-   ldrsh
-   ldrt
-   mcr
-   mla
-   mov
-   mrc
-   mrs
-   msr
-   mul
-   mvn
-   or (also referred to as orr)
-   rsb
-   rsc
-   sbc
-   smlal
-   smull
-   stc
-   stm
-   str
-   strb
-   strbt
-   strh
-   strt
-   sub
-   swi
-   swp
-   swpb
-   teq
-   tst
-   umlal
-   umull

The Thumb instructions are

-   adc
-   add
-   and
-   asr
-   b
-   bic
-   bl
-   bx
-   cmn
-   cmp
-   eor
-   ldmia
-   ldr
-   ldrb
-   ldrh
-   ldrsb
-   ldrsh
-   lsl
-   lsr
-   mov
-   mul
-   mvn
-   neg
-   or
-   pop
-   push
-   ror
-   sbc
-   stmia
-   str
-   strb
-   strh
-   sub
-   swi
-   tst
