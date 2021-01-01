class Three {
	constructor(gl) {
		this.context = gl;
		this.program = gl.createProgram();

		this.VERTEX_SHADER = gl.VERTEX_SHADER;
		this.FRAGMENT_SHADER = gl.FRAGMENT_SHADER;

		gl.clearColor(0, 0, 0, 1);
		gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
	}

	attachShaders(...shaders) {
		const gl = this.context;

		shaders.forEach((shader) => gl.attachShader(this.program, shader));
		gl.linkProgram(this.program);

		if (!gl.getProgramParameter(this.program, gl.LINK_STATUS)) {
			console.error(gl.getProgramInfoLog(this.program));
		}
	}

	validate() {
		const gl = this.context;

		gl.validateProgram(this.program);

		if (!gl.getProgramParameter(this.program, gl.VALIDATE_STATUS)) {
			console.error(gl.getProgramInfoLog(this.program));
		}
	}

	compile(type, source) {
		const gl = this.context;
		const shader = gl.createShader(type);

		gl.shaderSource(shader, source);
		gl.compileShader(shader);

		if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
			console.error(gl.getShaderInfoLog(shader));
		}

		return shader;
	}

	compileVertex(source) {
		return this.compile(this.VERTEX_SHADER, source);
	}

	compileFragment(source) {
		return this.compile(this.FRAGMENT_SHADER, source);
	}

	setVertices(...vertices) {
		const gl = this.context;

		const master = vertices[0];
		const shape = Object.entries(master).map(([name, value]) => {
			console.assert(Array.isArray(value) || typeof value === "number");
			return [name, Array.isArray(value) ? value.length : 1];
		});
		const vertexSize = shape.reduce((sum, [_name, size]) => sum + size, 0);

		// Allocate the number of elements we need times 4 bytes each, and then
		// declare the interface arrays for each type we need.
		const intermediate = new ArrayBuffer(vertexSize * vertices.length * 4);
		const floatIntermediate = new Float32Array(intermediate);
		const integerIntermediate = new Uint32Array(intermediate);

		let i = 0;
		vertices.forEach((vertex) => {
			shape.forEach(([name, size]) => {
				const value = vertex[name];

				console.assert(Array.isArray(value) || typeof value === "number");
				if (Array.isArray(value)) console.assert(value.length === size);

				if (Array.isArray(value))
					value.forEach((piece) => (floatIntermediate[i++] = piece));
				else integerIntermediate[i++] = value;
			});
		});

		const buffer = gl.createBuffer();
		gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
		gl.bufferData(gl.ARRAY_BUFFER, intermediate, gl.STATIC_DRAW);

		let j = 0;
		shape.forEach(([name, size]) => {
			const al = gl.getAttribLocation(this.program, name);
			if (size > 1) {
				gl.vertexAttribPointer(
					al,
					size,
					gl.FLOAT,
					gl.FALSE,
					vertexSize * 4,
					j * 4,
				);

				gl.enableVertexAttribArray(al);
			} else {
				gl.uniform1i(al, 80);
			}

			j += size;
		});
	}

	render() {
		const gl = this.context;

		gl.useProgram(this.program);
		gl.drawArrays(gl.TRIANGLES, 0, 3);
	}
}

window.addEventListener("DOMContentLoaded", () => {
	const canvas = document.querySelector("#display");
	const gl = canvas.getContext("webgl");

	// const aspect = 1.6;
	const aspect = 1;
	const height = 850;
	const width = height * aspect;

	canvas.width = width;
	canvas.height = height;
	gl.viewport(0, 0, width, height);

	const three = new Three(gl);

	const vs = three.compileVertex(`
		precision mediump float;
		attribute vec2 vertPosition;
		attribute vec3 vertColor;
		varying vec3 fragColor;

		void main() {
			fragColor = vertColor;
			gl_Position = vec4(vertPosition, 0.0, 1.0);
		}
	`);

	const fs = three.compileFragment(`
		precision mediump float;
		varying vec3 fragColor;

		void main() {
			gl_FragColor = vec4(fragColor, 1.0);
		}
	`);

	// const fs = three.compileFragment(`
	// 	precision mediump float;
	// 	uniform uint redAmount;
	// 	varying vec3 fragColor;

	// 	void main() {
	// 		gl_FragColor = vec4( redAmount / 255, 0.0, 0.0, 1.0 );
	// 	}
	// `);

	three.attachShaders(vs, fs);
	three.validate();

	three.setVertices(
		{
			vertPosition: [0, 0.5],
			vertColor: [1, 1, 0],
		},
		{
			vertPosition: [-0.6, -0.5],
			vertColor: [0.7, 0, 1],
		},
		{
			vertPosition: [0.6, -0.5],
			vertColor: [0.1, 1, 0.6],
		},
	);

	three.setVertices(
		{
			vertPosition: [-0.6, 0.5],
			vertColor: [1, 1, 0],
		},
		{
			vertPosition: [-0.6, -0.5],
			vertColor: [0.7, 0, 1],
		},
		{
			vertPosition: [0.6, -0.5],
			vertColor: [0.1, 1, 0.6],
		},
	);

	three.render();
});
