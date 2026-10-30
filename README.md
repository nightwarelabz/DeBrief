![DeBrief — A city of spies](debrief-banner.jpg)

# DeBrief — A city of spies

You are an NSA special agent. Every entry in the series opens with the op and closes with the debriefing.

1987. The Cold War did not end — it curdled. The first entry is one block of Greyport: wet concrete, sodium lamps, a Regime patrol, and a file that is written when you reach the other end. The design lives in [docs/](docs/DESIGN.md). The concept art in [concept-art/](concept-art/) is the golden standard the picture is judged against.

Made with the **C35 engine** (Comigen) — a from-scratch engine aiming at a PlayStation 1.5 look: better than PS1, not quite PS2. Low-poly, low-res, clean.

Status: in development. The engine lives at [nightwarelabz/C35](https://github.com/nightwarelabz/C35) and is fetched from `main` when you build.

AGPL-3.0 — see [LICENSE](LICENSE).

## Build

You need Rust (stable, 1.85 or newer), git, and a network path to GitHub the first time — the game depends on C35 by git. Headless modes do not need a GPU. The window does: wgpu on Vulkan, Metal, or GL.

```bash
cargo run -p debrief -- --mode survey
cargo test -p debrief
```

`survey` prints the block (triangles, lamps, grapple-anchor candidates, the light along both routes) and exits. It does not open a window.

## Play

The op is one crossing. You start at the north mouth, by the crane, and the file closes when you reach the south end, where the checkpoint furniture is. The patrol is a conscript: he sees with the same light the street is drawn in, he hesitates, and rain degrades him. Crouch on the warehouse side (the east sidewalk). The two east lamps are real. Sprint the middle of the street and he will have you.

A headless tape, no window, prints the debrief:

```bash
cargo run -p debrief -- --mode play --tape careful
cargo run -p debrief -- --mode play --tape lit
cargo run -p debrief -- --mode play --tape loud --night
```

`careful` crouches the shadow and waits when the next step would be seen. `lit` sprints the lamp spine. `loud` fires once. `--night` forces night so the birds call even if your clock says day; without it, night follows the reality layer (sun more than 6° down). `--rain 1` is a downpour, and the conscript is worse at his job.

In a window:

```bash
cargo run -p debrief -- --mode windowed
```

Click to capture the mouse. **F6** steps onto the street.

| Key | On the street |
| --- | --- |
| W A S D | Walk |
| Mouse | Look |
| Ctrl | Crouch |
| Shift | Sprint |
| Space | Jump |
| E (hold) | Grapple — the claw bites brick, concrete, the crane |
| F | Fire the sidearm. A shot is on the file. |
| Right mouse | Aim |
| C | Street coat / assault kit |
| Enter | Close the file where you stand |

Reach the south end and the handler reads it back in the terminal. The record is also written to `debrief-record.txt` in `--out` (default the working directory). There is no reload. What happened is the file.

Three birds perch on the block. At night you hear a churring with no direction on it. Crouch, and go slowly, and you may find one. The count is a margin on the file. Nothing explains them, and the handler does not mention them.

The fly rig is still there for looking at the block: WASD and Q/E, Shift to move faster, Esc to release the mouse, F1 and F2 for the two still cameras, F3 and F4 to watch the scripted crossings, P to save a frame.

## Other modes

```bash
cargo run -p debrief -- --mode sense
cargo run -p debrief -- --mode still --out out
cargo run -p debrief -- --mode poses --out out
cargo run -p debrief -- --mode drive --out out
```

`sense` walks the two scripted routes against the conscript and prints how long he spent patroling, suspicious, searching, and alarmed. `still`, `poses`, and `drive` render frames; they need a GPU, and they are the proof rig, not the game. `--proof` and `--smoke 5` on `windowed` run that rig unattended.

The clock and the latitude are the engine's reality layer. `COMIGEN_PIN_ISO`, `COMIGEN_LAT`, `COMIGEN_LON`, and `COMIGEN_UTC_OFFSET_MIN` pin them. Leave them unset and the window uses the machine's clock.

## Where to read

- [docs/DESIGN.md](docs/DESIGN.md) — the design bible
- [docs/WORLD.md](docs/WORLD.md) — Greyport, the Regime, the exile service
- [docs/STORY-BIBLE.md](docs/STORY-BIBLE.md) — the story, with proposals marked as proposals
- [docs/SLICE-1-BRIEF.md](docs/SLICE-1-BRIEF.md) — the block this repo currently is
- [docs/FIRST-PERSON.md](docs/FIRST-PERSON.md) — the camera
- [docs/COMPENDIUM.md](docs/COMPENDIUM.md) — the archive, specified, not all of it built
