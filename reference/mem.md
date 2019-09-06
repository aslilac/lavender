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
