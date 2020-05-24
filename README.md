# Lavender

[![build status](https://github.com/partheseas/lavender/workflows/main/badge.svg)](https://github.com/partheseas/lavender/actions)
![package version](https://img.shields.io/badge/lavender-v0.0.1-9394e4.svg)
[![code style: prettier](https://img.shields.io/badge/code_style-prettier-ff69b4.svg)](https://github.com/prettier/prettier)

A Game Boy Advance emulator written in Rust that runs in the browser.

## Goals

### Emulate the Game Boy Advance with speed and precision

All games should play at a native 60fps. This probably isn't feasible by
myself for literally _every_ game, but if I come across a bug then I will do my
best to fix it. If you come across a bug the file an issue or a pull request.

The emulator should also be fully unit tested. This is kind of a small detail,
but it is something that it missing from a lot of emulator projects,
especially small ones (like me!). No random breaking with new releases, no
surprises during development.

### Provide intricate documentation of the Game Boy Advance interals

There isn't currently a lot of recent or well written documentation about how
the Game Boy Advance works. The materials that do exist are either old, hard to
find, and full of dead links, or are incomplete, inaccurate, or hard to
understand.

This particular goal comes in two parts:

-   A book that compiles everything into one easy to reference place
    with clear descriptions and structure. There won't be any risk of dead links,
    because everything is self contained. Links to the resources that I have
    gathered information from will be provided for those who want it, but if the
    links die some years from now it won't affect the integrity of the book.
-   Inline code documentation to clarify on any code that may seem unclear to
    programmers who are unfamiliar with Rust (but don't be scared, it isn't that
    bad) or for particularly complicated parts. Follow along through the
    code while you're reading the book to gain a deeper understanding of the
    Game Boy Advance and Lavender itself.

Lots of other documentation and emulators use super gross abbreviations that
are cute for engineers, but sad for a lowly Computer Science major like myself.
Even sadder for someone who doesn't have any formal engineering education.

-   No messy C code with #define everywhere and magic voodoo headers. No
    unsanitized macros. No other gross C things.
-   Variable and constant names should prefer being clear and descriptive over being short.
-   Function names should describe what they do. Avoid one word function names.
    One word is not a description.

## Usage

To get set up for development you need to install all of the following tools...

-   [Node.js](https://nodejs.org)
-   [Yarn](https://yarnpkg.com)
-   [Rust](https://rustup.rs)

Then run `yarn` and `cargo build` to install all the other dependencies from
npm and Cargo.

Once all of that is done, to start a development server that will automatically
recompile and refresh your browser when files are updated, simply run...

```Shell
yarn dev
```

If you want to run it in release mode (optimized and high-performance, basically
if you want to actually play games) then run...

```Shell
yarn release
```

You should then be able to access the emulator at `http://localhost:1234` after
starting either script.

## Progress

### ARM v4T Emulation

-   [ ] 32-bit ARM instructions
    -   [x] Decoding
    -   [ ] Disassembly
    -   [ ] Behavior
-   [ ] 16-bit Thumb instructions
    -   [ ] Decoding
    -   [ ] Disassembly
    -   [ ] Behavior

### Graphics Emulation

-   [ ] Objects
    -   [ ] Mode 0 (all 4 layers, no rotate or scale)
    -   [ ] Mode 1 (layers [0..2], layer 2 rotate and scale)
    -   [ ] Mode 2 (layers [2..3], both rotate and scale)
-   [ ] Bitmaps
    -   [x] Mode 3 (240x160, full color, unbuffered)
    -   [ ] Mode 4 (240x160, palette color, buffered)
    -   [ ] Mode 5 (160x128, full color, buffered)

### Audio Emulation

-   [ ] Channels 1 & 2 (square wave)
-   [ ] Channel 3 (sampled)
-   [ ] Channel 4 (noise)
-   [ ] Channel A & B (direct sound)

### Miscellaneous

-   [ ] DMA
-   [ ] WebGL
