# Memory map

section name (description) | address space | total size | bus width | user permissions
-------------------------- | ------------- | ---------- | --------- | -----------
BIOS | 0x0000_0000-0x0000_3fff | 16kb | 32bit | --x
eRAM (on cartridge) | 0x0200_0000-0x0203_ffff | 256kb | 16bit | rwx
RAM (cpu embedded) | 0x0300_0000-0x0300_7fff | 32kb | 32 bit | rwx
IO (memory-mapped registers) | 0x0400_0000-0x0400_03ff | 1kb | 16 bit | rw-
Palette (2 palettes x 256 entries x 15 bit colors) | 0x0500_0000-0x0500_03ff | 1kb | 16 bit | rw-
VRAM | 0x0600_0000-0x0601_7fff | 96kb | 16 bit | r--
Object Attribute Memory | 0x0700_0000-0x0700_03ff | 1kb | 32 bit | rw-
ROM (bios probably points execution to here) | 0x0800_0000-0x0dff_ffff | 32mb | 16 bit | r-x
Persistent RAM (basically save files) | 0x0e00_0000-0x0e00_ffff (can theorhetically be bigger, but is unnecessary) | >= 64kb | 8 bit | rw-


Address Bus Width and CPU Read/Write Access Widths
Shows the Bus-Width, supported read and write widths, and the clock cycles for 8/16/32bit accesses.
  Region        Bus   Read      Write     Cycles
  BIOS ROM      32    8/16/32   -         1/1/1
  Work RAM 32K  32    8/16/32   8/16/32   1/1/1
  I/O           32    8/16/32   8/16/32   1/1/1
  OAM           32    8/16/32   16/32     1/1/1 *
  Work RAM 256K 16    8/16/32   8/16/32   3/3/6 **
  Palette RAM   16    8/16/32   16/32     1/1/2 *
  VRAM          16    8/16/32   16/32     1/1/2 *
  GamePak ROM   16    8/16/32   -         5/5/8 **/***
  GamePak Flash 16    8/16/32   16/32     5/5/8 **/***
  GamePak SRAM  8     8         8         5     **
Timing Notes:
  *   Plus 1 cycle if GBA accesses video memory at the same time.
  **  Default waitstate settings, see System Control chapter.
  *** Separate timings for sequential, and non-sequential accesses.
  One cycle equals approx. 59.59ns (ie. 16.78MHz clock).
All memory (except GamePak SRAM) can be accessed by 16bit and 32bit DMA.
