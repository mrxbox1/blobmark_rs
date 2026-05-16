## Note for context
This is _not_ a benchmark for measuring the performance of Rust. 
The actual intended purpose of this benchmark is to see whether it's worth it for me to use Macroquad at all in an upcoming project.
Other graphics libraries or game dev frameworks (such as wgpu, ggez, bevy, piston, sdl3, and et cetera.) may be slower or faster.

## What is this?
A benchmark to measure the performance of some graphics libraries and game dev frameworks I would like to try out.
I'm willing to test MonoGame/XNA next with a ported version of my own code.

## "Performance"
My computer has an Intel i7 with integrated Intel HD 630 graphics, 32 GB of RAM (pre-crisis).
I also have a GTX 1080 Ti, but it's very likely that it wasn't used to render the blobmark.

On the release build, at 20000 blobs, my computer persisted at around over 60 FPS. 
10% of the CPU with only around 50 MB of memory was used.
I'm pretty happy with this result, though I'm curious to see how MonoGame will perform.
