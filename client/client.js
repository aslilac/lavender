// import {
// } from 'three';

let frame = 0;

function enable_drawing( ioAddress, vramAddress ) {
  const display = document.querySelector( '#display' );
  const output = document.querySelector( '#output' );
  const box = display.getBoundingClientRect();
  
  const _2d = display.getContext( '2d' );
  const dpr = window.devicePixelRatio || 1;

  display.width = box.width * dpr;
  display.height = box.height * dpr;
  _2d.scale( box.width * dpr / 240, box.height * dpr / 160 );

  _2d.fillStyle = '#ffffff';
  _2d.fillRect( 0, 0, 240, 160 );

  function render() {
    const beginning = Date.now();

    const io = new Uint8Array( window.memory.buffer.slice( ioAddress, ioAddress + 1024 ) );
    const vram = new Uint8Array( window.memory.buffer.slice( vramAddress, vramAddress + 96 * 1024 ) );

    const displayMode = io[0] + (io[1] << 8);

    // Eventually all of the computation needs to either be done in Rust or GLSL,
    // with JavaScript only serving to pass values inbetween the two of them.
    // The end total amount of JavaScript should be essentially negligable.

    // We currently just kind of assume that the display is in mode three. We
    // probably need to knock that off at somepoint.
    // for ( let i = 0; i < 240 * 160; i++ ) {
    //   // Convert the 15 bit rgb colors stored in memory to 32 bit rgba colors.

    //   data[ i * 4 ] = ;
    //   data[ i * 4 + 1 ] = ;
    //   data[ i * 4 + 2 ] = ;
    //   data[ i * 4 + 3 ] = 255;
    // }

    // Frankly, this is just....bad.  It's incredibly slow, but the built in
    // putImageData doesn't have any scaling support. WebGL will be so much
    // better when I get that backend working properly, but in the mean time this
    // allows development to continue on other parts of the emulator, at least
    // until it becomes a bottleneck (it already is, with frame times of ~24ms).
    let prevColor = null
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

    output.innerHTML = `Frame: ${frame++}\nFrame time: ${Date.now()-beginning}ms\nDisplay mode: 0x${displayMode.toString(16)}`;
    // requestAnimationFrame( render );
  }

  render();
}

// This is so that wasm-bindgen can import this properly
window.enable_drawing = enable_drawing;

// We have to import both of these because index_bg doesn't correctly
// allow us to pass values back and forth even though it does expose
// the functions. Idk if it's a bug or intended, but this way works.
Promise.all([
  import( '../wasm/index_bg' ),
  import( '../wasm' )
]).then( ([ module, lavender ]) => {
    // The GBA always needs at least 8 pages for memory allocation
    window.memory = module.memory;

    // Load the rom into the emulator
    fetch( '/rom_tests/bin/first.gba' )
    // fetch( '/game/pokemon_emerald.gba' )
    .then( response => response.arrayBuffer() )
      .then( buffer => lavender.start_gba( new Uint8Array( buffer ) ) );
    
  })


