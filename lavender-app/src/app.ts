import { Controller } from "./controller/EmulatorController";

// We have to import both of these because index doesn't export memory,
// but index_bg does. index exports the functions, but index_bg doesn't.
async function main() {
	const [emulator, { memory }] = await Promise.all([
		import("lavender"),
		import("lavender/target/index_bg.wasm"),
	]);

	// Load the rom into the emulator
	// fetch("/game/pokemon_emerald.gba")
	const response = await fetch("/rom_tests/bin/first.gba");
	const buffer = await response.arrayBuffer();
	emulator.init_emulation(new Uint8Array(buffer));

	// Create a controller to interact with the emulation
	new Controller(emulator, memory).enableDrawing();
}

void main();
