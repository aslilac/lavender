import React from 'react';
import ReactDOM from 'react-dom';

import Overlay from './overlay';

interface Memory {
  io: Uint8Array,
  palette: Uint8Array,
  vram: Uint8Array,
  object: Uint8Array,
}

export class Controller {
  emulator: any;
  rawMemory: WebAssembly.Memory;
  memory: Memory;

  context: CanvasRenderingContext2D;
  frame: number;
  shouldEmulate: boolean;
  emulationTime: number;
  frameEnd: number;
  renderTime: number;

  constructor(emulator, rawMemory: WebAssembly.Memory) {
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
    const overlay = document.querySelector<HTMLElement>('#overlay');
    if (true) overlay.style.display = 'none';

    const display = document.querySelector<HTMLCanvasElement>('#display');
    const box = display.getBoundingClientRect();
    
    const _2d = this.context = display.getContext('2d');
    const dpr = window.devicePixelRatio || 1;
  
    display.width = box.width * dpr;
    display.height = box.height * dpr;
    _2d.scale(box.width * dpr / 240, box.height * dpr / 160);
  
    _2d.fillStyle = '#000000';
    _2d.fillRect(0, 0, 240, 160);
  
    // Allow spacebar to begin emulation
    window.addEventListener('keydown', event => {
      if (event.code === 'Space') {
        this.shouldEmulate = !this.shouldEmulate;
        if (this.shouldEmulate) {
          console.log('Drawing enabled');
          requestAnimationFrame(() => this.emulate());
        }
      }

      else if (event.code === 'Backquote') {
        overlay.style.display = overlay.style.display === 'none' ? 'unset' : 'none';
      }
    });

    // Render the frame once, and then pause until manually resumed. The wrap is
    // necessary so that the call is put on the event loop rather than executing
    // immediately. If it executes immediately it will attempt to call step_frame
    // while the mutex is still locked from enable_drawing.
    requestAnimationFrame(() => this.render());

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
    ReactDOM.render(
      <Overlay
        controller={this}
        emulator={this.emulator}
      />,
      document.querySelector('#overlay')
    );
  }
}

// We have to import both of these because index_bg doesn't correctly
// allow us to pass values bac and forth even though it does expose
// the functions. Idk if it's a bug or intended, but this way works.
Promise.all([
  import('../target/wasm-pack'),
  import('../target/wasm-pack/index_bg'),
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


