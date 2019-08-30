// import {
// } from 'three';

let frame = 0;

function enable_drawing( ioAddress, vramAddress ) {
  const display =document.querySelector( '#display' );
  const output = document.querySelector( '#output' );
  const box = display.getBoundingClientRect();

  const io = new Uint8Array( window.memory.buffer.slice( ioAddress, ioAddress + 1024 ) );
  const vram = new Uint8Array( window.memory.buffer.slice( vramAddress, vramAddress + 96 * 1024 ) );
  
  const _2d = display.getContext( '2d' );
  const dpr = window.devicePixelRatio || 1;

  display.width = box.width * dpr;
  display.height = box.height * dpr;
  _2d.scale( box.width * dpr / 240, box.height * dpr / 160 );

  _2d.fillStyle = '#ffffff';
  _2d.fillRect( 0, 0, 240, 160 );

  const image = _2d.createImageData( 240, 160 );

  function render() {
    const beginning = Date.now();

    const displayMode = io[0] + (io[1] << 8);
    const data = image.data;
    const height = image.height;
    const width = image.width;

    for ( let i = 0; i < 240 * 160; i++ ) {
      const rgb15 = vram[ i * 2 ] + (vram[ i * 2 + 1 ] << 8);

      data[ i * 4 ] = (rgb15 & 0x001f) * 8;
      data[ i * 4 + 1 ] = ((rgb15 & 0x03e0) >> 5) * 8;
      data[ i * 4 + 2 ] = ((rgb15 & 0x7c00) >> 10) * 8;
      data[ i * 4 + 3 ] = 255;
    }

    // Frankly, this is just....bad.  It's incredibly slow, but the built in
    // putImageData doesn't have any scaling support. WebGL will be so much
    // better when I get that backend working properly, but in the mean time this
    // allows development to continue on other parts of the emulator, at least
    // until it becomes a bottleneck (it already is, with frame times of ~24ms).
    for ( let y = 0; y < height; y++ ) {
      for ( let x = 0; x < width; x++ ) {
        const i = y * width + x;

        _2d.fillStyle = 'rgba(' + data[ i * 4 ]
                          + ',' + data[ i * 4 + 1 ]
                          + ',' + data[ i * 4 + 2 ]
                          + ', 1)';
        _2d.fillRect( x, y, 1.2, 1.2 );
      }
    }

    output.innerHTML = `Frame: ${frame++}\nFrame time: ${Date.now()-beginning}ms\nDisplay mode: 0x${displayMode.toString(16)}`;
    // requestAnimationFrame( render );
  }

  // requestAnimationFrame( render );
  render();
}

// This is so that wasm-bindgen can import this properly
window.enable_drawing = enable_drawing;

Promise.all([
  import( '../wasm/index_bg' ),
  import( '../wasm' )
]).then( ([ module, lavender ]) => {
    // The GBA always needs at least 8 pages for memory allocation
    window.memory = module.memory;

    // Load the rom into the emulator
    fetch( '/rom_tests/bin/first.gba' )
      .then( response => response.arrayBuffer() )
      .then( buffer => {
        lavender.start_gba( new Uint8Array( buffer ) );
      });
    
    // fetch( '/game/pokemon_emerald.gba' )
    //   .then( response => response.arrayBuffer() )
    //   .then( buffer => {
    //     let data = new Uint8Array( buffer );

    //     console.log( lavender );
    //     lavender.start_gba( data.slice( 0, 10 ) );
    //   });
  })


