# Instruction Encoding

The decode bits of an instruction are defined to be bits[27:20] and bits[7:4].

In memory, instructions are stored as little endian numbers, just like all other
data in memory. Pay attention to this if you're ever looking at a hexdump of a
ROM. For example, a branch instruction will look like `xx xx xx ea` instead of
`ea xx xx xx`.
