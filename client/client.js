const drawingModes = [
  '[Object]\n    all 4 layers, no rotate or scale',
  '[Object]\n    layers [0..2], layer 2 rotate and scale',
  '[Object]\n    layers [2..3], both rotate and scale',
  '[Bitmap]\n    full resolution, full color, unbuffered',
  '[Bitmap]\n    full resolution, palette color, buffered',
  '[Bitmap]\n    160x128, full color, buffered'
];

let controller = null;

class Controller {
  constructor(emulator, rawMemory) {
    this.emulator = emulator;
    this.rawMemory = rawMemory;

    const ioAddress = emulator.get_io_address();
    const paletteAddress = emulator.get_palette_address();
    const vramAddress = emulator.get_vram_address();
    const objectAddress = emulator.get_object_address();

    this.memory = {
      io: new Uint8Array(rawMemory.buffer.slice(ioAddress, ioAddress + 1024)),
      palette: new Uint8Array(rawMemory.buffer.slice(paletteAddress, paletteAddress + 1024)),
      vram: new Uint8Array(rawMemory.buffer.slice(vramAddress, vramAddress + 96 * 1024)),
      object: new Uint8Array(rawMemory.buffer.slice(objectAddress, objectAddress + 1024)),
    };

    this.context = null;
    this.frame = 0;
    this.shouldRender = false;
    this.emulationTime = 0;
    this.frameEnd = 0;
    this.frameTime = 0;
  }

  enableDrawing() {
    const display = document.querySelector('#display');
    const step = document.querySelector('#step-instruction');
    const box = display.getBoundingClientRect();
    
    const _2d = display.getContext('2d');
    const dpr = window.devicePixelRatio || 1;
  
    display.width = box.width * dpr;
    display.height = box.height * dpr;
    _2d.scale(box.width * dpr / 240, box.height * dpr / 160);
  
    _2d.fillStyle = '#000000';
    _2d.fillRect(0, 0, 240, 160);
  
    // Allow spacebar to begin emulation
    window.addEventListener('keydown', event => {
      if (event.keyCode === 32) {
        this.shouldRender = !this.shouldRender;
        if (this.shouldRender) {
          console.log('Drawing enabled');
          requestAnimationFrame(() => this.render());
        }
      }
    });

    step.addEventListener('click', () => {
      this.emulator.step_instruction();
      this.updateRegisters();
    });

    this.context = _2d;
    this.shouldRender = true;
    // Render the frame once, and then pause until manually resumed. The wrap is
    // necessary so that the call is put on the event loop rather than executing
    // immediately. If it executes immediately it will attempt to call step_frame
    // while the mutex is still locked from enable_drawing.
    requestAnimationFrame(() => {
      console.log( "Beginning render" )
      // We don't want to start on page load, so we call render directly, and then
      // set shouldRender to false so that it will cancel that next animation frame. 
      this.render();
      this.shouldRender = false;
    });

    return this;
  }

  render() {
    if (!this.shouldRender) return;

    const beginning = Date.now();

    // For now this always 1, because my laptop screen has a 60Hz refresh rate
    // and can run the emulation at full speed. In the future, we need to
    // calculate this more exactly, so that if the computer is fast enough but
    // has a display running at 30Hz, we don't run at half speed, because that
    // would be sad.
    this.emulator.step_frames(1);
    this.emulationTime = Date.now() - beginning;

    const _2d = this.context;
    const io = this.memory.io;
    const vram = this.memory.vram;

    const displayControl = io[0] + (io[1] << 8);
    const displayMode = displayControl & 7;

    // I'm not a fan of any of this, but putImageData doesn't provide any scaling
    // support and this performs...okay, now that I've added some optimizations.
    // WebGL is definitely still the route we need to go down eventually.
    let prevColor = null;

    if (displayMode === 3) {
      for (let y = 0; y < 160; y++) {
        let beginX = 0;

        for (let x = 0; x < 240; x++) {
          const i = y * 240 + x;
          const rgb15 = vram[i * 2] + (vram[i * 2 + 1] << 8);

          if (rgb15 !== prevColor) {
            // Draw the rectangle before we change colors
            _2d.fillRect(beginX, y, x - beginX + 0.2, 1.2);

            _2d.fillStyle = 'rgba(' + (rgb15 & 0x001f) * 8
                              + ',' + (rgb15 >> 5 & 0x001f) * 8
                              + ',' + (rgb15 >> 10 & 0x001f) * 8
                              + ', 1)';
            prevColor = rgb15;
            beginX = x;
          }
        }

        // Draw to the end of the line.
        _2d.fillRect(beginX, y, 240.2 - beginX, 1.2);
      }
    }

    this.frameEnd = Date.now();
    this.frameTime = this.frameEnd - beginning;
    this.updateStatus();
    this.updateRegisters();
    requestAnimationFrame(() => this.render());
  }

  updateStatus() {
    const status = document.querySelector('#status');

    const emulationTime = this.emulationTime;
    const frameTime = this.frameTime;
    const emulationTimeColor = emulationTime > 11 ? '--red' : emulationTime < 7 ? '--green' : '--yellow';
    const frameTimeColor = frameTime > 13 ? '--red' : frameTime < 9 ? '--green' : '--yellow';
    const displayMode = this.memory.io[0] & 7;

    status.innerHTML = `Frame: ${this.frame++}
Emulation time: <span style="color: var(${emulationTimeColor})">${emulationTime}ms</span>
Frame time: <span style="color: var(${frameTimeColor})">${frameTime}ms</span>
Display mode: ${displayMode} <${drawingModes[displayMode]}>`;
  }

  updateRegisters() {
    const registers = document.querySelector('#registers');
    registers.innerHTML = Array.from(this.emulator.read_registers()).map(value =>
      `<span class="register">${value.toString(16)}</span>`
    ).join('&nbsp;');
  }
}

// We have to import both of these because index_bg doesn't correctly
// allow us to pass values bac and forth even though it does expose
// the functions. Idk if it's a bug or intended, but this way works.
Promise.all([
  import('../wasm'),
  import('../wasm/index_bg'),
]).then(([emulator, { memory }]) => {
  // Load the rom into the emulator
  fetch('/rom_tests/bin/first.gba')
  // fetch( '/game/pokemon_emerald.gba' )
    .then(response => response.arrayBuffer())
    .then(buffer => {
      emulator.start_emulation(new Uint8Array(buffer));
      controller = new Controller(emulator, memory).enableDrawing();
    });
});


