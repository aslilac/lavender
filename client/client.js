// import {
// } from 'three';

const drawingModes = [
  '[Object]\n    all 4 layers, no rotate or scale',
  '[Object]\n    layers [0..2], layer 2 rotate and scale',
  '[Object]\n    layers [2..3], both rotate and scale',
  '[Bitmap]\n    full resolution, full color, unbuffered',
  '[Bitmap]\n    full resolution, palette color, buffered',
  '[Bitmap]\n    160x128, full color, buffered'
];

function enable_drawing( emulator, memory ) {
  const ioAddress = emulator.get_io_address();
  const vramAddress = emulator.get_vram_address();

  const display = document.querySelector( '#display' );
  const output = document.querySelector( '#stats' );
  const box = display.getBoundingClientRect();
  
  const _2d = display.getContext( '2d' );
  const dpr = window.devicePixelRatio || 1;

  let frame = 0;
  let shouldRender = true;

  display.width = box.width * dpr;
  display.height = box.height * dpr;
  _2d.scale( box.width * dpr / 240, box.height * dpr / 160 );

  _2d.fillStyle = '#000000';
  _2d.fillRect( 0, 0, 240, 160 );

  function render() {
    if ( !shouldRender ) return;

    const beginning = Date.now();

    // For now this always 1, because my laptop screen has a 60Hz refresh rate
    // and can run the emulation at full speed. In the future, we need to
    // calculate this more exactly, so that if the computer is fast enough but
    // has a display running at 30Hz, we don't run at half speed, because that
    // would be sad.
    emulator.step_frames(1);
    const emulationTime = Date.now() - beginning;

    const io = new Uint8Array( memory.buffer.slice( ioAddress, ioAddress + 1024 ) );
    const vram = new Uint8Array( memory.buffer.slice( vramAddress, vramAddress + 96 * 1024 ) );

    const displayControl = io[0] + (io[1] << 8);
    const displayMode = displayControl & 7;

    // I'm not a fan of any of this, but putImageData doesn't provide any scaling
    // support and this performs...okay, now that I've added some optimizations.
    // WebGL is definitely still the route we need to go down eventually.
    let prevColor = null;

    if ( displayMode === 3 ) {
      for ( let y = 0; y < 160; y++ ) {
        let beginX = 0;

        for ( let x = 0; x < 240; x++ ) {
          const i = y * 240 + x;
          const rgb15 = vram[ i * 2 ] + (vram[ i * 2 + 1 ] << 8);

          if ( rgb15 !== prevColor ) {
            // Draw the rectangle before we change colors
            _2d.fillRect( beginX, y, x - beginX + 0.2, 1.2 );

            _2d.fillStyle = 'rgba(' + (rgb15 & 0x001f) * 8
                              + ',' + (rgb15 >> 5 & 0x001f) * 8
                              + ',' + (rgb15 >> 10 & 0x001f) * 8
                              + ', 1)';
            prevColor = rgb15;
            beginX = x;
          }
        }

        // Draw to the end of the line.
        _2d.fillRect( beginX, y, 240.2 - beginX, 1.2 );
      }
    }


    const frameTime = Date.now() - beginning;
    const emulationTimeColor = emulationTime > 11 ? '--red' : emulationTime < 7 ? '--green' : '--yellow';
    const frameTimeColor = frameTime > 13 ? '--red' : frameTime < 9 ? '--green' : '--yellow';

    output.innerHTML = `Frame: ${frame++}
Emulation time: <span style="color: var(${emulationTimeColor})">${emulationTime}ms</span>
Frame time: <span style="color: var(${frameTimeColor})">${frameTime}ms</span>
Display mode: ${displayMode} <${drawingModes[displayMode]}>`;

    requestAnimationFrame( render );
  }

  // Allow spacebar to begin emulation
  window.addEventListener( 'keydown', event => {
    if ( event.keyCode === 32 ) {
      shouldRender = !shouldRender;
      if ( shouldRender ) requestAnimationFrame( render );
    }

    console.log( 'Drawing enabled' );
  });

  // Render the frame once, and then pause until manually resumed. The wrap is
  // necessary so that the call is put on the event loop rather than executing
  // immediately. If it executes immediately it will attempt to call step_frame
  // while the mutex is still locked from enable_drawing.
  requestAnimationFrame( () => {
    console.log( "Beginning render" );
    // We don't want to start on page load, so we call render directly, and then
    // set shouldRender to false so that it will cancel that next animation frame. 
    render();
    shouldRender = false;
  });
}

// We have to import both of these because index_bg doesn't correctly
// allow us to pass values back and forth even though it does expose
// the functions. Idk if it's a bug or intended, but this way works.
Promise.all([
  import( '../wasm' ),
  import( '../wasm/index_bg' ),
]).then(([emulator, { memory }]) => {
  // Load the rom into the emulator
  fetch( '/rom_tests/bin/first.gba' )
  // fetch( '/game/pokemon_emerald.gba' )
    .then( response => response.arrayBuffer() )
    .then( buffer => emulator.start_gba( new Uint8Array( buffer ) ) )
    .then( () => enable_drawing( emulator, memory ) );
});


