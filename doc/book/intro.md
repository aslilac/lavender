# Introduction

The Game Boy Advance is a one of a kind console, and has a couple of things that
make it quite interesting to emulate. It also happens to be the console that I
played with by far the most growing up, and so I have a soft spot for it in my
heart.

## Why you should make a Game Boy Advance emulator

If you're reading this, it's probably because you already decided that you want
to make an emulator, but you might not be sure that the Game Boy Advance is
really what you want to emulate. There are a couple of reasons why I think that
the Game Boy Advance is the right choice for most people:

-   It isn't that much more complicated than something like the NES, SNES, or the
    original Game Boy.
-   You'll learn ARM assembly in the process: an architecture that is still in use
    to this day. The processor used in the Game Boy Advance is quite similar to the
    one in your smart phone today.
-   The games are (subjectively) more fun than ones for older consoles and are
    (objectively) nicer to look at. Our brains enjoy looking at nice things, and
    you will be looking at your emulator _a lot_.

To be clear, it is harder, but you will also learn more, be challenged more, and
enjoy the end product more as a result.

## Terminology and Notation

Through out this book, I will use the same notation as all of ARM's documentation
when referring to bit ranges in a value. That is, in a 32 bit number, the bits go
in order from bit 31 to bit 0.

For example,

> The decode bits of an instruction are defined to be bits[27:20] and bits[7:4].
