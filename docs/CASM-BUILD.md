# Building and playing DeBrief (CASM-PS15)

DeBrief runs entirely on the zero-dependency **CASM-PS15** engine (C11 +
hand-written ARM64 NEON / x86_64 SSE2 assembly, vendored under `engine/` — see
`engine/VENDORED.md`). There is no Rust, Cargo, or GPU dependency in the build.

## Build & Run

```sh
make
./debrief-game
```

Requirements: a C11 compiler (`cc` / `clang` / `gcc`) and, on Linux, X11 + ALSA
+ pthreads (`-lX11 -lasound -lpthread`); on macOS, the built-in system
frameworks (`Cocoa`, `AudioToolbox`) only.

## Perspective & Gameplay

*DeBrief: A City of Spies* opens on a **live 3D interactive Banner Screen**
framing the NSA Operator on the rain-slicked Greyport West Quay. Pressing
`Space` or `LMB` sweeps the camera seamlessly around his right shoulder into
**collision-resolved Third-Person Over-the-Shoulder** gameplay — showing your
full silhouette in sodium light vs. deep harbor shadow, your grapple swing arc,
trooper greatcoat disguise state, and scale against the towering Type-1 Patrol
Mecha.

## Modern Controls

- **Space / LMB** (on Title Banner) — deploy seamlessly into Greyport
- **WASD + Mouse** — 360° camera-relative movement & over-the-shoulder look
  (**Tab** releases cursor)
- **RMB / Q** (hold) — tight over-the-shoulder precision ADS aim + Mecha telemetry
- **LMB / X / Return** — fire suppressed HK USP:
  - Shoot **sodium streetlamps / watchtower searchlight** to carve darkness
  - Shoot **walls** to create acoustic bullet-ping lures (`[?]`)
  - Land **headshots** on troopers or target Patrol Mecha **crimson sensor**,
    **leg actuators**, or **dorsal coolant spine**
- **E** (hold) — physical raycast-anything grapple (every solid surface
  grapples; hold **C / Ctrl** while tethered to lock rope length and pendulum-swing)
- **Space** — coyote/buffered jump, kinetic wall-run & wall-jump, and automatic
  ledge vaulting onto crates, containers & rooftops
- **Shift** — sprint (**Shift + C / Ctrl** triggers a momentum crouch-slide)
- **F** — silent CQC takedown on a nearby trooper; press **F** again near a
  downed trooper to **don/shed a Regime Greatcoat Disguise** (fools troopers at
  walking pace beyond 4.5 m; the Patrol Mecha's biometric sensor sees through cover)
- **R** — hack classified Nightjar Archive Terminals, reload HK USP, or launch
  next operation after extraction at the West Harbor Barge
- **N** (or **6**) — toggle Crimson SIGINT Night-Optic Visor
- **M** (or **8**) — open/close NSA Handler Debrief Dossier (`GHOST-THAT-NEVER-WAS`,
  `GHOST`, `SURGEON`, `OPERATIVE`, `BUTCHER`)
- **V** — toggle front-orbit character showcase view
- **1–5, 7** — live PS1.5 hardware showcase toggles (`1` Render stats, `2` Vertex
  snap, `3` Perspective/Affine UV, `4` Bayer RGB555 DAC, `5` Wireframe, `7`
  Sony Trinitron CRT scanlines)
- **Esc** — quit
