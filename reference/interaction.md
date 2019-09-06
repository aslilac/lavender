# Interacting with the outside world

We've talked a lot about how the Game Boy Advance works, and if you've been
following along with the book you probably have a pretty sizable code base
already. Eventually we need the ability to see what is on the screen of our
emulated system, and interact with it via the 10 hardware buttons: Left, right,
up, down, L, R, A, B, Select, and Start.

As we discussed briefly in the chapter on memory, there are two sections of
memory that we need access to from outside of the emulator. We need to be able
to write into IO memory so that the emulator knows when buttons are being pressed,
and we need to be able to read from VRAM so that the data contained there can
be drawn onto the screen.
