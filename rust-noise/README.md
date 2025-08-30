# Rust Noise

A Rust port of the C noise.c program that creates an animated noise pattern with distortion effects.

## Description

This program creates a 320x240 window displaying animated noise. The center 60x60 area features a swirling distortion effect based on polar coordinates, while the rest of the window shows random noise.

## Dependencies

- `winit` - Cross-platform window creation and event handling
- `softbuffer` - Software buffer for rendering pixels to windows
- `rand` - Random number generation

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

Note: This requires a graphical environment (X11, Wayland, or similar) to run.

## Original C Code

The original C implementation used the `fenster.h` library for window management. This Rust version uses `winit` and `softbuffer` for cross-platform compatibility.

## Algorithm

1. Generate a 512x512 noise map with random values
2. In each frame:
   - For pixels in the center 60x60 area: apply polar coordinate distortion
   - For other pixels: generate random noise
   - Update the display buffer
   - Request redraw for animation