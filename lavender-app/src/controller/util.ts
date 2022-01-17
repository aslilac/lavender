export type MemoryView = {
	io: Uint8Array;
	palette: Uint8Array;
	vram: Uint8Array;
	object: Uint8Array;
};

export const createMemoryView = (
	emulator: Lv.Core,
	rawMemory: WebAssembly.Memory,
): MemoryView => {
	const buf = rawMemory.buffer;

	const ioAddr = emulator.get_io_address();
	const paletteAddr = emulator.get_palette_address();
	const vramAddr = emulator.get_vram_address();
	const objectAddr = emulator.get_object_address();

	return {
		io: new Uint8Array(buf.slice(ioAddr, ioAddr + 1024)),
		palette: new Uint8Array(buf.slice(paletteAddr, paletteAddr + 1024)),
		vram: new Uint8Array(buf.slice(vramAddr, vramAddr + 96 * 1024)),
		object: new Uint8Array(buf.slice(objectAddr, objectAddr + 1024)),
	};
};
