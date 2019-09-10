# Timing
For timing, Lavender aligns by frame and runs instructions in chunks, rather
than trying to accurately emulate the real time per cycle. I chose this
approach for a couple of reasons.

- The inaccuracy of trying to keep incredibly small timings synced correctly can
cause emulation problems and performance concerns.
- Running instructions in chunks decreases the total emulation overhead, since
we only need to align 60 times per second instead of trying to align on every
single clock cycle.
- The end result shouldn't be distinguishable from the real thing by someone
using the emulator, since they see the exact same thing: a display that gets
updated 60 times per second.
- WebAssembly doesn't really enable any sort of timing mechanisms, so the timing
has to be controlled from JavaScript. Calling into WebAssembly from JavaScript is
relatively expensive, so we want to get away with having to do it as little as
possible. On low performance systems, we could potentially step through instructions
in two frame intervals and attempt to reduce the overhead that way, but that has
yet to be seen, as we are still in early development.
