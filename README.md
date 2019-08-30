# Lavender
A Game Boy Advance emulator written in Rust that runs in the browser.

## Goals

### Emulate the Game Boy Advance with speed and precision
All games should play at a native 60fps. This probably isn't feasible by
myself for literally *every* game, but if I come across a bug then I will do my
best to fix it. If you come across a bug the file an issue or a pull request.

The emulator should also be fully unit tested. This is kind of a small detail,
but it is something that I haven't seen in any of the open source emulators
that I am familiar with. No random breaking with new releases, no surprises
during development.

### Provide intricate documentation of the Game Boy Advance interals
There isn't currently a lot of recent or well written documentation about how
the Game Boy Advance works. The materials that do exist are either old, hard to
find, and full of dead links, or are incomplete, inaccurate, or hard to
understand.

This particular goal comes in two parts:
  - A book that compiles everything into one easy to reference place
  with clear descriptions and structure. There won't be any risk of dead links,
  because everything is self contained. Links to the resources that I have
  gathered information from will be provided for those who want it, but if the
  links die some years from now it won't affect the integrity of the book.
  - Inline code documentation to clarify on any code that may seem unclear to
  programmers who are unfamiliar with Rust (but don't be scared, it isn't that
  bad) or for particularly complicated parts. Follow along through the
  code while you're reading the book to gain a deeper understanding of the
  Game Boy Advance and Lavender itself.

Lots of other documentation and emulators use super gross abbreviations that
are cute for engineers, but sad for a lowly Computer Science major like myself.
Even sadder for someone who doesn't have any formal engineering education.
- No messy C code with #define everywhere and magic voodoo headers.
- Constant names should prefer being clear and descriptive over being short
- Function names should describe what they do. Avoid one word function names.
One word is not a description.

## Usage
This project uses Webpack as its base. It bundles the JavaScript code and
integrates with wasm-pack to automatically compile/recompile our Rust source
code, which enables quick iteration.

**Note:** When Parcel 2 comes out with better WebAssembly support, I will likely
switch to that.

To get set up for development you need to install the following tools...

- [Node.js](https://nodejs.org)
- [Yarn](https://yarnpkg.com)
- [Rust](https://rustup.rs)
- [wasm-pack cli](https://rustwasm.github.io/wasm-pack/installer/)

...and then run Yarn to install dependencies from npm.

To start a development server that will automatically recompile updated files
and refresh your browser, simply run...

```Shell
yarn dev
```

You should then be able to access the emulator at `localhost:1234`
