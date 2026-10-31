# Building and playing DeBrief (CASM-PS15)

DeBrief runs entirely on the CASM-PS15 engine (C + assembly, vendored
under `engine/` — see `engine/VENDORED.md`). There is no Rust or Cargo
in the build.

## Build

```sh
make
./debrief
```

Requirements: a C compiler and, on Linux, X11 + ALSA development
libraries; on macOS, the system frameworks (Cocoa, AudioToolbox) only.

## Controls

- **WASD + mouse** — move and look (first person by default)
- **V** — toggle first/third person (third-person camera is
  collision-resolved)
- **Shift** — sprint · **Ctrl** — crouch · **Space** — jump
- **E** — grapple: press fires at the lamp or tower in the view cone,
  hold keeps the rope, release detaches with momentum
- **Esc** — quit

Live synthesized audio (lamp hum, footsteps, grapple, alarm) plays
through the platform audio device (ALSA on Linux, CoreAudio on macOS).
File assets load from `engine/assets/` at startup, with procedural
generation as the fallback.
