# Vendored engine

The contents of `engine/` are a vendored snapshot of the **CASM-PS15**
PlayStation 1.5 C11 + assembly engine (`https://github.com/nightwarelabz/CASM-PS15`),
upgraded with:

- 8-pixel perspective-correct opaque (`ps15_span_tex64`) and 4-mode PS1 ABR
  semi-transparent (`ps15_span_abr64`) SIMD rasterizer kernels (ARM64 NEON +
  x86_64 SSE2 + bit-identical C11 reference fallbacks)
- 2x nearest & Sony Trinitron CRT scanline upscalers (`ps15_upscale2x`, `ps15_upscale2x_crt`)
- Real-time planar water reflections (`render_mesh_mirror` + `render_water_plane`)
- Sculpted NSA Operator (matching the DeBrief banner portrait & rear gear sheet),
  Regime Trooper, and 3.55 m Type-1 Bipedal Patrol Mecha (`mecha_mesh_new`)
- Collision-resolved Third-Person Over-the-Shoulder camera (`tps_camera_step`),
  raycast-anything Nexuiz grapple (`grapple_fire_ray`), coyote/buffered jump,
  sprint crouch-slide, kinetic wall-run (`physics_wallrun_step`), and ledge
  auto-mantle (`physics_mantle_step`)
- Diegetic amber visor HUD (`hud_draw_visor`), crimson SIGINT night-optic visor
  (`fb_apply_visor`), and live procedural espionage audio synthesis

DeBrief builds and runs entirely against this self-contained copy — zero external
dependencies beyond `libc`/`libm` and native OS window/audio frameworks.

Layout:

- `engine/src/` — renderer, Greyport harbor world, animation, physics, audio,
  perception, persistence, asset loaders, platform layers, assembly hot loops
- `engine/assets/` — file-loaded assets (`crate.obj`, `brick.ppm`)
- `game/debrief.c` — *DeBrief: A City of Spies* playable third-person stealth-action game
