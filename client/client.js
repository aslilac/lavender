import React from 'react';
import ReactDOM from 'react-dom';

const color = (actual, yellow, red) => actual > red ? '--red' : actual < yellow ? '--green' : '--yellow';

const drawingModes = [
  '[Object]\n    all 4 layers, no rotate or scale',
  '[Object]\n    layers [0..2], layer 2 rotate and scale',
  '[Object]\n    layers [2..3], both rotate and scale',
  '[Bitmap]\n    full resolution, full color, unbuffered',
  '[Bitmap]\n    full resolution, palette color, buffered',
  '[Bitmap]\n    160x128, full color, buffered'
];

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
    this.shouldEmulate = false;
    this.emulationTime = 0;
    this.frameEnd = 0;
    this.renderTime = 0;
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
        this.shouldEmulate = !this.shouldEmulate;
        if (this.shouldEmulate) {
          console.log('Drawing enabled');
          requestAnimationFrame(() => this.emulate());
        }
      }

      else if (event.keyCode === 27) {
        const overlay = document.querySelector('#overlay');
        overlay.style.display = overlay.style.display === 'none' ? 'unset' : 'none';
      }

      else {
        const keycode = document.querySelector('#keycode');
        keycode.innerHTML = event.keyCode;
      }
    });

    this.context = _2d;

    // Render the frame once, and then pause until manually resumed. The wrap is
    // necessary so that the call is put on the event loop rather than executing
    // immediately. If it executes immediately it will attempt to call step_frame
    // while the mutex is still locked from enable_drawing.
    requestAnimationFrame(() => {
      console.log('Beginning render');
      // We don't want to start on page load, so we call render directly, and then
      // set shouldEmulate to false so that it will cancel that next animation frame. 
      this.render();
    });

    return this;
  }

  emulate() {
    if (!this.shouldEmulate) return;

    const emulationBeginning = Date.now();
    // For now this always 1, because my laptop screen has a 60Hz refresh rate
    // and can run the emulation at full speed. In the future, we need to
    // calculate this more exactly, so that if the computer is fast enough but
    // has a display running at 30Hz, we don't run at half speed, because that
    // would be sad.
    this.emulator.step_frames(1);
    this.emulationTime = Date.now() - emulationBeginning;

    const renderBeginning = Date.now();
    this.render();
    this.frameEnd = Date.now();
    this.renderTime = this.frameEnd - renderBeginning;
    this.frame++;
    
    requestAnimationFrame(() => this.emulate());
  }

  render() {
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

            // Translate from 15-bit color to 32-bit color
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

    this.updateOverlay();
  }

  updateOverlay() {
    const emulationTime = this.emulationTime;
    const renderTime = this.renderTime;
    const emulationTimeColor = `var(${color(emulationTime, 7, 11)})`;
    const renderTimeColor = `var(${color(renderTime, 3, 5)})`;
    const displayMode = this.memory.io[0] & 7;

    ReactDOM.render(
      <>
        <h6>Status</h6>
        <pre id="status">
          Frame: {this.frame}<br />
          Emulation time: <span style={{color: emulationTimeColor}}>{emulationTime}ms</span><br />
          Frame time: <span style={{color: renderTimeColor}}>{renderTime}ms</span><br />
          Display mode: {displayMode} &lt;{drawingModes[displayMode]}&gt;<br />
        </pre>
        <button id="step-instruction" onClick={() => {
          this.emulator.step_instruction();
          this.render();
          this.updateOverlay();
        }}>Step &rarr;</button>

        <h6>Registers</h6>
        <div id="registers">
          {
            Array.from(this.emulator.read_registers()).map((value,id) =>
              <span key={id} className="register">{value.toString(16)}<sub className="register-label">r{id}</sub></span>
            )
          }
          <span className="register">{this.emulator.read_cpsr().toString(16)}<sub className="register-label">cpsr</sub></span>
        </div>

        <h6>Memory</h6>
        <label>Instruction prefetch</label>
        <code id="next-instruction">{this.emulator.read_next_instruction().toString(2).padStart(32, '0')}</code>
        
        <h6>IO</h6>
        <label>Keycode</label>
        <code id="keycode">0</code>

        <h6>Links</h6>
        <div>
          <a href="/target/book/">Book</a><br />
          <a href="/target/doc/lavender/">Documentation</a>
        </div>
      </>,
      document.querySelector('#overlay')
    );
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
      new Controller(emulator, memory).enableDrawing();
    });
});


