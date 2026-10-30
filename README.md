![DeBrief — A city of spies](debrief-banner.jpg)

# DeBrief — A city of spies

You are an NSA special agent. Every entry in the series opens with the op and closes with the debriefing.

This tree is **Slice 1**: one Greyport street block on the [C35 engine](https://github.com/nightwarelabz/C35) (Comigen). The block is procedural — no image assets. A headless proof host renders stills and two scripted crossings, and a windowed host shows the same street live. The window is a camera on that street, not the full game yet: the operator and the Regime patrol are not under player locomotion. In free-fly the camera carries a first-person viewmodel.

Presentation defaults to the engine's PS1.5 look: low-poly, low-resolution, clean. Better than PS1, not quite PS2.

The engine is a git dependency on `nightwarelabz/C35` `main`. It is not vendored here.

## Build, test, run

The engine's crates need a current stable Rust (1.99 or newer). From the repo root:

```sh
cargo build
cargo test
cargo clippy -- -D warnings
```

`cargo test` covers the pin parser, the patrol loop, and the drive routes. It does not open a window.

Run the host with `cargo run -p debrief --` and any of these. `--out` defaults to the current directory, `--width` / `--height` default to 1280×720, and `--present` defaults to `ps15`.

| `--mode` | What it does |
| --- | --- |
| `survey` (default) | No renderer. Prints the block census, the grapple-anchor CSV, and exposure along both drive routes. |
| `still` | Hero-street and patrol-echo stills (`hero-street.png`, `echo-patrol.png`). |
| `poses` | The patrol's pose sheet, plus the HALT staged at the checkpoint. |
| `drive` | Open crossing, then shadow crossing. Writes a PNG and a light-read CSV per route. |
| `windowed` | The same block in a window. `--smoke <secs>` runs unattended and exits. `--proof` captures F1, F2, then both crossings, and exits. |

```sh
cargo run -p debrief -- --mode survey
cargo run -p debrief -- --mode still --out out
cargo run -p debrief -- --mode drive --out out
cargo run -p debrief -- --mode windowed
cargo run -p debrief -- --mode windowed --smoke 2
```

An unknown `--mode` exits 2. Stills, poses, and drives need a GPU backend (Vulkan, GL, or Metal). `survey` does not.

### Reality rig

The hosts read these environment variables. The engine crate does not.

| Variable | Meaning |
| --- | --- |
| `COMIGEN_PIN_ISO` | UTC instant, ISO-8601 with an explicit offset: `2026-01-15T08:00:00-06:00`, or `Z` / `z`. A pin that does not parse falls back to the system clock. |
| `COMIGEN_LAT` | Observer latitude, degrees, +north. Omitted → equator fallback. |
| `COMIGEN_LON` | Observer longitude, degrees, +east. Preferred source of solar time. |
| `COMIGEN_UTC_OFFSET_MIN` | Minutes east of UTC, used for solar time only when longitude is absent. |

## Windowed controls

Click captures the mouse. Esc releases it.

| Input | Action |
| --- | --- |
| WASD | Fly along the view |
| E / Q | Fly up / down |
| Shift | Faster fly |
| Mouse | Look, while captured |
| Right mouse | Aim the viewmodel (fly mode) |
| C | Toggle the viewmodel kit: street / assault |
| F1 / F2 | Hold the hero-street / patrol-echo camera |
| F3 / F4 | Run the open / shadow crossing, follow camera |
| 0–4 | Pin the governor at that level (the session starts pinned at 0) |
| `` ` `` | Release the governor pin |
| P | Save a PNG of the current frame into `--out` |

F1 and F2 do nothing while a crossing owns the camera. Fly input drops a held bookmark and keeps the view.

## Project layout

```
debrief/           Slice 1 host (debrief/src: main, block, light, soldier, windowed)
docs/              Design bible, story, world, first-person note, slice brief
concept-art/       Reference sheets and the banner studies
debrief-banner.jpg Series banner
```

North in the block is −Z (the dockyard crane). The street runs along Z, warehouses on +X, tenements on −X.

AGPL-3.0 — see [LICENSE](LICENSE).
