# Vendored engine

The contents of `engine/` are a vendored snapshot of the **CASM-PS15**
engine (https://github.com/nightwarelabz/CASM-PS15), taken at commit
`f5e4ac0a3275d8e6f2d372c5b183dd489267337e` (engine v1 + windowed host + perspective-correct texture mapping).

DeBrief builds and runs entirely against this copy — no Rust, no Cargo,
no external engine checkout. To update the engine: copy the newer
`src/` and `assets/` trees from the CASM-PS15 repository over
`engine/src/` and `engine/assets/`, update the commit recorded above,
and rebuild.

Layout:

- `engine/src/` — renderer, animation, physics, audio, perception,
  persistence, asset loaders, platform layers, assembly loops
- `engine/assets/` — file-loaded test assets (OBJ crate, PPM brick)
- `game/` — the game itself, built on the engine
