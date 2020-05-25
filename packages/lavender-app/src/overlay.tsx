import React, { useEffect, useState } from "react";

import type { Controller, Emulator } from "./app";

const drawingModes = [
	"[Object]\n    all 4 layers, no rotate or scale",
	"[Object]\n    layers [0..2], layer 2 rotate and scale",
	"[Object]\n    layers [2..3], both rotate and scale",
	"[Bitmap]\n    full resolution, full color, unbuffered",
	"[Bitmap]\n    full resolution, palette color, buffered",
	"[Bitmap]\n    160x128, full color, buffered",
];

const color = (x: number, yellow: number, red: number): string =>
	x > red ? "var(--red)" : x > yellow ? "var(--yellow)" : "var(--green)";

type OverlayProps = {
	controller: Controller;
	emulator: Emulator;
};

export const Overlay = (props: OverlayProps) => {
	const { controller, emulator } = props;
	const { emulationTime, renderTime } = controller;
	const emulationTimeColor = color(emulationTime, 7, 11);
	const renderTimeColor = color(renderTime, 3, 5);
	const displayMode = controller.memory.io[0] & 7;

	let step = () => {
		try {
			emulator.step_instruction();
		} catch (e) {
			console.error("Something went wrong in the emulation step", e);
		}

		try {
			controller.render();
		} catch (e) {
			console.error("Something went wrong in the render step", e);
		}
	};

	return (
		<section id="overlay">
			<h6>Status</h6>
			<pre id="status">
				Frame: {controller.frame}
				<br />
				Emulation time:{" "}
				<span style={{ color: emulationTimeColor }}>
					{emulationTime}ms
				</span>
				<br />
				Frame time:{" "}
				<span style={{ color: renderTimeColor }}>{renderTime}ms</span>
				<br />
				Display mode: {displayMode} &lt;{drawingModes[displayMode]}&gt;
				<br />
			</pre>
			<button onClick={step}>Step &rarr;</button>

			<h6>Registers</h6>
			<div id="registers">
				{Array.from(emulator.read_registers()).map((value, id) => (
					<span key={id} className="internal register">
						{value.toString(16).padStart(8, "0")}
						<sub className="register-label">r{id}</sub>
					</span>
				))}
				<span className="internal register">
					{emulator.read_cpsr().toString(16).padStart(8, "0")}
					<sub className="register-label">cpsr</sub>
				</span>
			</div>

			<h6>Memory</h6>
			<label>Instruction prefetch</label>
			<br />
			<code id="next-instruction" className="internal">
				{emulator.read_next_instruction().toString(2).padStart(32, "0")}
			</code>

			<h6>IO</h6>
			<label>KeyboardEvent.code</label>
			<br />
			<KeyObserver className="internal" />
			<br />

			<hr />

			<div>
				<a href="/target/book/">Book</a>
				<br />
				<a href="/target/doc/lavender/">Documentation</a>
				<br />
				<a href="https://github.com/partheseas/lavender">GitHub</a>
			</div>

			<p>
				&copy; 2020 &hearts;{" "}
				<a href="https://mckay.la" style={{ color: "white" }}>
					McKayla
				</a>
			</p>
		</section>
	);
};

const KeyObserver = (props: React.ComponentProps<"code">) => {
	const [keyName, setKeyName] = useState("...");

	useEffect(() => {
		let update = (key: KeyboardEvent) => setKeyName(key.code);
		window.addEventListener("keydown", update);
		return () => window.removeEventListener("keydown", update);
	});

	return <code {...props}>{keyName}</code>;
};
