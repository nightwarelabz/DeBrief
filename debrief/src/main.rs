//! DeBrief — Slice 1 host: one Greyport block on the C35 engine.
//!
//! A headless proof runner in the house pattern (the native vista
//! host's rig, pointed at a city): PS1.5 presentation by default,
//! the governor pinned, the vista suppressed, the block placed as
//! props, and both men — the operator and the patrol — merged
//! through the engine's character slot, posed through the
//! PoseFrame seam.
//!
//! Modes:
//!   --mode still   hero street still + the patrol-echo still
//!   --mode poses   the soldier's pose vocabulary (action sheet #15)
//!                  as stills, + the HALT staged at the checkpoint
//!   --mode drive   the two scripted drives (open / shadow) with
//!                  the light read logged per frame against the
//!                  patrol's facing
//!   --mode survey  no renderer: block census, the grapple anchor
//!                  survey, and the light field's route profiles
//!   --mode windowed  the same block LIVE in a real window
//!                  (see windowed.rs; --proof / --smoke <secs> run
//!                  it unattended for the proof rig)
//!
//! Reality rig (same contract as the vista host — pins are read by
//! this host only, never by the engine crate):
//!   COMIGEN_PIN_ISO / COMIGEN_LAT / COMIGEN_LON / COMIGEN_UTC_OFFSET_MIN

mod block;
mod light;
mod soldier;
mod windowed;

use std::time::{Instant, SystemTime, UNIX_EPOCH};

use c35::glam::{Vec2, Vec3};
use c35::locomotion::{Agent, Animator, Controller, Gait, ScriptLeg};
use c35::operator::{build_operator_frame, OperatorMesh, PoseFrame};
use c35::world::{iso_utc, world_state, Observer, WorldState};

#[derive(Debug)]
struct Cli {
    mode: String,
    out_dir: String,
    width: u32,
    height: u32,
    present: String,
    proof: bool,
    smoke_secs: Option<f32>,
}

/// Proof-rig arguments. Unknown flags, a missing value, a zero
/// framebuffer, or a smoke/proof flag on a mode that ignores it are
/// errors — they used to fall through into a GPU init or an
/// unattended run that never exited.
fn parse_cli(args: &[String]) -> Result<Cli, String> {
    const VALUE_FLAGS: &[&str] = &[
        "--mode",
        "--out",
        "--width",
        "--height",
        "--present",
        "--smoke",
    ];
    const BOOL_FLAGS: &[&str] = &["--proof"];
    let mut mode = None;
    let mut out_dir = None;
    let mut width = None;
    let mut height = None;
    let mut present = None;
    let mut proof = false;
    let mut smoke_secs = None;
    let mut i = 1;
    while i < args.len() {
        let flag = args[i].as_str();
        if BOOL_FLAGS.contains(&flag) {
            proof = true;
            i += 1;
            continue;
        }
        if !VALUE_FLAGS.contains(&flag) {
            return Err(format!("unknown argument '{flag}'"));
        }
        let Some(raw) = args.get(i + 1) else {
            return Err(format!("{flag} needs a value"));
        };
        if raw.starts_with("--") {
            return Err(format!("{flag} needs a value"));
        }
        match flag {
            "--mode" => mode = Some(raw.clone()),
            "--out" => out_dir = Some(raw.clone()),
            "--width" => width = Some(parse_frame_extent("--width", raw)?),
            "--height" => height = Some(parse_frame_extent("--height", raw)?),
            "--present" => present = Some(raw.clone()),
            "--smoke" => {
                let secs: f32 = raw.parse().map_err(|_| {
                    format!("--smoke needs a non-negative number of seconds (got {raw})")
                })?;
                if !secs.is_finite() || secs < 0.0 {
                    return Err(format!(
                        "--smoke needs a non-negative number of seconds (got {raw})"
                    ));
                }
                smoke_secs = Some(secs);
            }
            _ => unreachable!("flag checked against VALUE_FLAGS"),
        }
        i += 2;
    }
    let mode = mode.unwrap_or_else(|| "survey".to_string());
    if !matches!(
        mode.as_str(),
        "still" | "poses" | "drive" | "survey" | "windowed"
    ) {
        return Err(format!(
            "unknown --mode '{mode}' (still|poses|drive|survey|windowed)"
        ));
    }
    let present = present.unwrap_or_else(|| "ps15".to_string());
    if !matches!(present.as_str(), "ps15" | "classic") {
        return Err(format!("unknown --present '{present}' (ps15|classic)"));
    }
    if (proof || smoke_secs.is_some()) && mode != "windowed" {
        return Err("--proof and --smoke apply to --mode windowed".to_string());
    }
    if proof && smoke_secs.is_some() {
        return Err(
            "--proof and --smoke cannot be combined; smoke would exit before the proof sequence finishes"
                .to_string(),
        );
    }
    Ok(Cli {
        mode,
        out_dir: out_dir.unwrap_or_else(|| ".".to_string()),
        width: width.unwrap_or(1280),
        height: height.unwrap_or(720),
        present,
        proof,
        smoke_secs,
    })
}

fn parse_frame_extent(flag: &str, raw: &str) -> Result<u32, String> {
    match raw.parse::<u32>() {
        Ok(n) if n >= 1 => Ok(n),
        _ => Err(format!("{flag} needs a positive integer (got {raw})")),
    }
}

fn env_f32(name: &str) -> Option<f32> {
    std::env::var(name).ok().and_then(|v| v.parse().ok())
}

/// Days in a proleptic Gregorian month. Year 0 is a leap year, matching
/// the engine calendar (`days_from_civil`): divisible by 400, or by 4
/// but not by 100.
fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
            if leap {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// One `HH`, `HH:MM`, or `HH:MM:SS` field. A missing minute or second
/// is zero. A present field that does not parse is a bad pin — it must
/// not collapse to zero, which is what turned `-05:30\n` into `-05:00`.
fn parse_hms(tail: &str) -> Option<(f64, f64, f64)> {
    let mut parts = tail.split(':');
    let hh: f64 = parts.next()?.parse().ok()?;
    let mm: f64 = match parts.next() {
        None => 0.0,
        Some(p) => p.parse().ok()?,
    };
    let ss: f64 = match parts.next() {
        None => 0.0,
        Some(p) => p.parse().ok()?,
    };
    if parts.next().is_some() {
        return None;
    }
    if !(0.0..24.0).contains(&hh) || !(0.0..60.0).contains(&mm) || !(0.0..60.0).contains(&ss) {
        return None;
    }
    Some((hh, mm, ss))
}

/// Parse the rig's pin: ISO-8601 local time with an explicit offset
/// (`2026-01-15T08:00:00-06:00`, `...Z`, seconds optional). Returns
/// (unix seconds, offset minutes).
///
/// The civil date is the engine's calendar ([`c35::world::days_from_civil`]),
/// the same one the vista host and [`iso_utc`](c35::world::iso_utc) use,
/// so the pin cannot disagree with the ledger. Impossible months and
/// days are rejected: the old inline formula accepted month 13 and
/// produced a silent wrong instant. Whitespace around the pin is
/// ignored; a minute field that does not parse is rejected rather
/// than read as zero.
fn parse_pin_iso(s: &str) -> Option<(f64, f32)> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    let (dt, offset_min) = if let Some(rest) = s.strip_suffix('Z') {
        (rest.trim_end(), 0.0f32)
    } else if let Some(idx) = s.rfind(['+', '-']) {
        if idx < 10 {
            return None; // a date separator, not an offset
        }
        let tail = &s[idx..];
        let sign: f32 = if tail.starts_with('-') { -1.0 } else { 1.0 };
        let mut parts = tail[1..].split(':');
        let oh: f32 = parts.next()?.parse().ok()?;
        let om: f32 = match parts.next() {
            None => 0.0,
            Some(p) => p.parse().ok()?,
        };
        // Offset seconds are not part of the rig contract. A numeric
        // third field is ignored (the previous parser did the same);
        // garbage is a bad pin.
        if let Some(extra) = parts.next() {
            if extra.parse::<f32>().is_err() {
                return None;
            }
        }
        if parts.next().is_some() || !(0.0..24.0).contains(&oh) || !(0.0..60.0).contains(&om) {
            return None;
        }
        (&s[..idx], sign * (oh * 60.0 + om))
    } else {
        (s, 0.0f32)
    };
    let mut it = dt.split('T');
    let date = it.next()?.trim();
    let time = it.next().unwrap_or("00:00:00").trim();
    if it.next().is_some() || date.is_empty() {
        return None;
    }
    let mut dparts = date.split('-');
    let year: i64 = dparts.next()?.parse().ok()?;
    let month: u32 = dparts.next()?.parse().ok()?;
    let day: u32 = dparts.next()?.parse().ok()?;
    if dparts.next().is_some()
        || !(1..=12).contains(&month)
        || day == 0
        || day > days_in_month(year, month)
    {
        return None;
    }
    let (hh, mm, ss) = parse_hms(time)?;
    let days = c35::world::days_from_civil(year, month, day) as f64;
    let local = days * 86400.0 + hh * 3600.0 + mm * 60.0 + ss;
    Some((local - f64::from(offset_min) * 60.0, offset_min))
}

pub(crate) struct Reality {
    pub(crate) world: WorldState,
    clock_line: String,
    observer_line: String,
    solar_line: String,
}

pub(crate) fn resolve_reality() -> Reality {
    let mut offset_env = env_f32("COMIGEN_UTC_OFFSET_MIN");
    let (unix, clock_line) = match std::env::var("COMIGEN_PIN_ISO") {
        Ok(pin) => match parse_pin_iso(&pin) {
            Some((unix, pin_offset)) => {
                if offset_env.is_none() {
                    offset_env = Some(pin_offset);
                }
                (unix, format!("rig pin (COMIGEN_PIN_ISO={pin})"))
            }
            None => {
                eprintln!("warning: COMIGEN_PIN_ISO not parseable, using system clock");
                (system_now(), "system clock (pin unparseable)".to_string())
            }
        },
        Err(_) => (system_now(), "system clock".to_string()),
    };
    let lat = env_f32("COMIGEN_LAT");
    let lon = env_f32("COMIGEN_LON");
    let observer = Observer { latitude_deg: lat, longitude_deg: lon, utc_offset_minutes: offset_env };
    let world = world_state(unix, &observer);
    let observer_line = match (lat, lon) {
        (Some(la), Some(lo)) => format!("lat {la:.2} (COMIGEN_LAT), lon {lo:.2} (COMIGEN_LON)"),
        (Some(la), None) => format!("lat {la:.2} (COMIGEN_LAT), no longitude"),
        _ => "latitude unknown — equator fallback in force".to_string(),
    };
    let solar_line = if lon.is_some() {
        "longitude".to_string()
    } else if let Some(off) = offset_env {
        format!("utc offset {off:.0} min")
    } else {
        "UTC (no longitude or offset given)".to_string()
    };
    Reality { world, clock_line, observer_line, solar_line }
}

fn system_now() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
}

pub(crate) fn print_reality(r: &Reality) {
    let w = &r.world;
    println!("--- reality reading ---");
    println!("clock source     : {}", r.clock_line);
    println!("instant (UTC)    : {}", iso_utc(w.unix_seconds));
    println!("observer         : {}", r.observer_line);
    println!("solar time src   : {}", r.solar_line);
    println!(
        "sun              : elevation {:+.2} deg, azimuth {:.1} deg, day length {:.1} h",
        w.sun_elevation_deg, w.sun_azimuth_deg, w.day_length_hours
    );
    println!(
        "season           : cold {:.2} / autumn {:.2} / spring {:.2} / summer {:.2}",
        w.cold, w.autumn, w.spring, w.summer
    );
}

/// The follow rig — the vista host's contract, unchanged (DESIGN
/// §8's open-ground law lives in these offsets).
pub(crate) fn follow_pose(anchor: Vec3, yaw: f32) -> c35::scene::CameraPose {
    let fwd = Vec3::new(yaw.sin(), 0.0, yaw.cos());
    let right = Vec3::new(-fwd.z, 0.0, fwd.x);
    c35::scene::CameraPose {
        eye: anchor + Vec3::new(0.0, 1.52, 0.0) - fwd * 3.4 + right * 0.62,
        target: anchor + Vec3::new(0.0, 1.48, 0.0) + fwd * 4.0,
        fov_y_deg: 50.0,
    }
}

pub(crate) fn flat_ground(_x: f32, _z: f32) -> f32 {
    0.0
}

/// The hero-street camera rig — the still's own composition,
/// shared verbatim with the windowed host's F1 bookmark (one
/// definition: if a still and the window disagree, the window is
/// wrong).
pub(crate) fn hero_pose() -> c35::scene::CameraPose {
    c35::scene::CameraPose {
        eye: Vec3::new(0.8, 1.6, 21.0),
        target: Vec3::new(-1.2, 2.0, -24.0),
        fov_y_deg: 50.0,
    }
}

/// The patrol-echo camera rig (golden #11's echo) — the still's
/// composition, shared with the windowed host's F2 bookmark.
pub(crate) fn echo_pose() -> c35::scene::CameraPose {
    c35::scene::CameraPose {
        eye: Vec3::new(-5.2, 0.9, 29.5),
        target: Vec3::new(3.2, 2.4, 4.0),
        fov_y_deg: 50.0,
    }
}

/// The two crossings' routes — start + scripted legs, shared
/// verbatim with the windowed host's live drives (F3 / F4).
/// Anything other than `open` or `shadow` is not a route: the old
/// `else` branch ran the shadow crossing under whatever name it
/// was given.
pub(crate) fn drive_route(which: &str) -> Option<(Vec2, Vec<ScriptLeg>)> {
    match which {
        "open" => Some((
            Vec2::new(-2.0, -44.0),
            vec![
                ScriptLeg::to(-2.0, -10.0, Gait::Walk),
                ScriptLeg::to(-2.0, 30.0, Gait::Run),
                ScriptLeg::to(-2.0, 44.0, Gait::Walk),
            ],
        )),
        "shadow" => Some((
            Vec2::new(8.9, -44.0),
            vec![ScriptLeg::to(8.9, 44.0, Gait::CrouchWalk)],
        )),
        _ => None,
    }
}

/// Chest sample for the light read. Crouch drops it; both hosts
/// call this so the logged height cannot drift.
pub(crate) fn light_sample(pos: Vec2, crouch: f32) -> Vec3 {
    Vec3::new(pos.x, 1.15 - 0.35 * crouch, pos.y)
}

/// Patrol facing against the operator, in the ground plane.
/// `(dot, distance)`. A coincident pair is dot 0, not a division by zero.
pub(crate) fn facing_dot(soldier_pos: Vec2, soldier_yaw: f32, op_pos: Vec2) -> (f32, f32) {
    let fwd = Vec2::new(soldier_yaw.sin(), soldier_yaw.cos());
    let to_op = op_pos - soldier_pos;
    let dist = to_op.length();
    let dot = if dist > 1e-4 {
        fwd.dot(to_op / dist)
    } else {
        0.0
    };
    (dot, dist)
}

pub(crate) const LIGHT_READ_HEADER: &str = "frame,t,op_x,op_z,gait,speed,exposure,lit,lamp,soldier_x,soldier_z,soldier_yaw_deg,facing_dot,soldier_dist\n";

pub(crate) fn light_read_row(
    frame: usize,
    op: &Agent,
    exposure: f32,
    lit: bool,
    lamp: usize,
    soldier: &Agent,
) -> String {
    let (dot, dist) = facing_dot(soldier.pos, soldier.yaw, op.pos);
    format!(
        "{},{:.3},{:.2},{:.2},{},{:.2},{:.3},{},{},{:.2},{:.2},{:.1},{:.3},{:.2}\n",
        frame,
        frame as f32 / 60.0,
        op.pos.x,
        op.pos.y,
        op.gait.name(),
        op.speed,
        exposure,
        if lit { 1 } else { 0 },
        lamp,
        soldier.pos.x,
        soldier.pos.y,
        soldier.yaw.to_degrees(),
        dot,
        dist
    )
}

/// Merge the two men's meshes into the engine's one character
/// slot: vertices concatenated, the second mesh's indices offset.
pub(crate) fn merge_meshes(a: &OperatorMesh, b: &OperatorMesh) -> OperatorMesh {
    let mut vertices = Vec::with_capacity(a.vertices.len() + b.vertices.len());
    vertices.extend_from_slice(&a.vertices);
    vertices.extend_from_slice(&b.vertices);
    let base = a.vertices.len() as u32;
    let mut indices = Vec::with_capacity(a.indices.len() + b.indices.len());
    indices.extend_from_slice(&a.indices);
    indices.extend(b.indices.iter().map(|i| i + base));
    let mut parts: Vec<c35::operator::PartInfo> = Vec::with_capacity(a.parts.len() + b.parts.len());
    for p in a.parts.iter().chain(b.parts.iter()) {
        parts.push(c35::operator::PartInfo { name: p.name, bone: p.bone, triangles: p.triangles });
    }
    OperatorMesh { vertices, indices, parts }
}

/// Map a copy-destination buffer and wait until the callback has
/// run. `get_mapped_range` panics if the map did not complete; the
/// old readback ignored the callback result and then mapped anyway.
pub(crate) fn map_read(device: &c35::wgpu::Device, buf: &c35::wgpu::Buffer) -> Result<(), String> {
    let (tx, rx) = std::sync::mpsc::channel();
    buf.slice(..)
        .map_async(c35::wgpu::MapMode::Read, move |res| {
            let _ = tx.send(res);
        });
    device
        .poll(c35::wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        })
        .map_err(|e| format!("gpu poll: {e}"))?;
    match rx.recv() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(format!("map readback: {e}")),
        Err(_) => Err("map readback callback dropped".to_string()),
    }
}

pub(crate) fn readback_bytes(width: u32, height: u32) -> Result<(u32, u64), String> {
    let row = (width as u64)
        .checked_mul(4)
        .ok_or("frame is too wide to read back")?;
    let bpr = row.div_ceil(256) * 256;
    if bpr > u64::from(u32::MAX) {
        return Err("frame is too wide to read back".to_string());
    }
    let size = bpr
        .checked_mul(u64::from(height))
        .ok_or("frame is too large to read back")?;
    Ok((bpr as u32, size))
}

fn save_png(
    renderer: &c35::Renderer,
    tex: &c35::wgpu::Texture,
    width: u32,
    height: u32,
    path: &str,
) -> Result<(), String> {
    let (bpr, size) = readback_bytes(width, height)?;
    let buf = renderer.device.create_buffer(&c35::wgpu::BufferDescriptor {
        label: Some("png readback"),
        size,
        usage: c35::wgpu::BufferUsages::MAP_READ | c35::wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut encoder =
        renderer
            .device
            .create_command_encoder(&c35::wgpu::CommandEncoderDescriptor {
                label: Some("png copy"),
            });
    encoder.copy_texture_to_buffer(
        c35::wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: 0,
            origin: c35::wgpu::Origin3d::ZERO,
            aspect: c35::wgpu::TextureAspect::All,
        },
        c35::wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: c35::wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bpr),
                rows_per_image: Some(height),
            },
        },
        c35::wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    renderer.queue.submit([encoder.finish()]);
    map_read(&renderer.device, &buf)?;
    let range = buf.slice(..).get_mapped_range();
    let mut pixels = Vec::with_capacity((width * height * 4) as usize);
    for row in 0..height as usize {
        let start = row * bpr as usize;
        pixels.extend_from_slice(&range[start..start + (width * 4) as usize]);
    }
    drop(range);
    buf.unmap();
    let img = image::RgbaImage::from_raw(width, height, pixels).ok_or("pixel buffer size")?;
    img.save(path).map_err(|e| format!("save {path}: {e}"))?;
    println!("saved {path}");
    Ok(())
}

fn save_png_or_exit(
    renderer: &c35::Renderer,
    tex: &c35::wgpu::Texture,
    width: u32,
    height: u32,
    path: &str,
) {
    if let Err(e) = save_png(renderer, tex, width, height, path) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

/// THE scene build — one definition for every host mode: the
/// presentation, the pinned governor, the suppressed vista, the
/// block placed as props, and the wet pass's lamp set mirrored
/// from the light field's own list (so the pools in the frame and
/// the exposures in the light read are one geography). The
/// headless rig and the windowed host both configure through
/// here; a mode that wants a different scene is a different mode,
/// not a quiet edit.
pub(crate) fn configure_scene(renderer: &mut c35::Renderer, blk: &block::Block, present: &str) {
    if present == "ps15" {
        renderer.set_present_mode(c35::PresentMode::Ps15, 2.0);
    }
    renderer.pin_governor(Some(0));
    renderer.set_vista_suppressed(true);
    renderer.set_props(Some(&blk.mesh));
    let lamp_set: Vec<[f32; 4]> = blk
        .field
        .lamps
        .iter()
        .map(|l| [l.head.x, l.head.y, l.head.z, light::LAMP_INTENSITY])
        .collect();
    renderer.set_lamp_set(&lamp_set);
}

struct Rig {
    renderer: c35::Renderer,
    out_tex: c35::wgpu::Texture,
    width: u32,
    height: u32,
}

impl Rig {
    fn new(width: u32, height: u32, present: &str, blk: &block::Block) -> Rig {
        let instance = c35::create_instance();
        let mut renderer = c35::pollster::block_on(c35::Renderer::new(
            &instance,
            c35::wgpu::TextureFormat::Rgba8UnormSrgb,
            width,
            height,
            1.0,
        ));
        configure_scene(&mut renderer, blk, present);
        let out_tex = renderer.device.create_texture(&c35::wgpu::TextureDescriptor {
            label: Some("debrief output"),
            size: c35::wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: c35::wgpu::TextureDimension::D2,
            format: c35::wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: c35::wgpu::TextureUsages::RENDER_ATTACHMENT | c35::wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        Rig { renderer, out_tex, width, height }
    }

    fn still(&mut self, world: &WorldState, pose: c35::scene::CameraPose, mesh: Option<&OperatorMesh>, path: &str) {
        match mesh {
            Some(m) => self.renderer.update_operator(m),
            None => self.renderer.clear_operator(),
        }
        let view = self.out_tex.create_view(&Default::default());
        for _ in 0..12 {
            self.renderer.render(&view, &pose, world, 0.0);
            let _ = self.renderer.device.poll(c35::wgpu::PollType::Wait { submission_index: None, timeout: None });
        }
        save_png_or_exit(&self.renderer, &self.out_tex, self.width, self.height, path);
    }

    fn ledger(&self, label: &str) {
        let r = &self.renderer;
        let (iw, ih) = r.internal_size();
        println!("=== DEBRIEF COST LEDGER ({label}) ===");
        println!("backend          : {}", r.backend_name());
        println!("output           : {}x{} (internal {}x{})", self.width, self.height, iw, ih);
        println!("presentation     : {}", r.present_description());
        println!("draw calls/frame : {}", r.draw_calls_per_frame());
        println!("triangles/frame  : {}", r.triangles_per_frame());
        println!(
            "tracked vram     : {:.2} MiB (engine-tracked allocations, estimate)",
            r.tracked_vram_bytes() as f64 / (1024.0 * 1024.0)
        );
        println!("governor         : pinned at level 0 (proof rig law)");
    }
}

pub(crate) fn op_mesh(frame: &PoseFrame, spot: Vec3, yaw_deg: f32) -> OperatorMesh {
    build_operator_frame(frame, spot, 0.0, yaw_deg)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cli = match parse_cli(&args) {
        Ok(cli) => cli,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };
    let Cli {
        mode,
        out_dir,
        width,
        height,
        present,
        proof,
        smoke_secs,
    } = cli;
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("create out dir {out_dir}: {e}");
        std::process::exit(1);
    }

    let blk = block::build_block();
    println!("{}", blk.census);
    println!(
        "light field      : {} lamps, lit threshold {:.2}",
        blk.field.lamps.len(),
        light::LIT_THRESHOLD
    );

    if mode == "survey" {
        println!("--- anchor survey ({} candidates) ---", blk.anchors.len());
        print!("{}", block::anchors_csv(&blk.anchors));
        println!("--- light profiles (exposure along the drive routes) ---");
        for which in ["open", "shadow"] {
            let x = drive_route(which).expect("named drive").0.x;
            let mut line = format!("{which} route x={x:.1}: ");
            let mut z = -44.0f32;
            while z <= 44.0 {
                let e = blk.field.exposure(light_sample(Vec2::new(x, z), 0.0));
                line.push_str(&format!(
                    "[{z:+.0}:{e:.2}{}] ",
                    if e >= light::LIT_THRESHOLD { "L" } else { "." }
                ));
                z += 4.0;
            }
            println!("{line}");
        }
        return;
    }

    let reality = resolve_reality();
    let world = reality.world;

    if mode == "windowed" {
        let params = windowed::Params {
            out_dir: out_dir.clone(),
            present: present.clone(),
            width,
            height,
            proof,
            smoke_secs,
        };
        windowed::run(blk, &reality, params);
        return;
    }

    let mut rig = Rig::new(width, height, &present, &blk);

    match mode.as_str() {
        "still" => {
            // HERO: down the street from the south mouth — lamp
            // pools receding, the crane in fog at the far end, both
            // men in the street.
            let op = op_mesh(
                &PoseFrame::from_pose(c35::OperatorPose::StandReady),
                Vec3::new(-2.0, 0.0, 3.0),
                168.0,
            );
            let so = soldier::soldier_mesh(
                &soldier::pose_low_ready(),
                Vec3::new(-8.4, 0.0, -9.0),
                0.0,
                190.0,
            );
            let both = merge_meshes(&op, &so);
            rig.still(
                &world,
                hero_pose(),
                Some(&both),
                &format!("{out_dir}/hero-street.png"),
            );
            // ECHO of golden #11: low camera, the patrol posted under
            // the east lamp with the banner on the wall behind him,
            // the operator crouched in the dark corner, foreground.
            let op = op_mesh(
                &PoseFrame::from_pose(c35::OperatorPose::Crouch),
                Vec3::new(-7.4, 0.0, 21.0),
                180.0,
            );
            let so = soldier::soldier_mesh(
                &soldier::pose_low_ready(),
                Vec3::new(6.0, 0.0, 19.2),
                0.0,
                0.0,
            );
            let both = merge_meshes(&op, &so);
            rig.still(
                &world,
                echo_pose(),
                Some(&both),
                &format!("{out_dir}/echo-patrol.png"),
            );
            rig.ledger("stills");
        }
        "poses" => {
            for (name, frame) in soldier::pose_vocabulary() {
                let so = soldier::soldier_mesh(&frame, Vec3::new(-2.0, 0.0, 0.0), 0.0, 0.0);
                rig.still(
                    &world,
                    c35::scene::CameraPose {
                        eye: Vec3::new(1.3, 1.25, 4.6),
                        target: Vec3::new(-2.0, 1.0, 0.0),
                        fov_y_deg: 50.0,
                    },
                    Some(&so),
                    &format!("{out_dir}/pose-{name}.png"),
                );
            }
            // The HALT staged where it belongs: at the checkpoint,
            // in the lane on the barrier's near side, facing the
            // traffic he is stopping — the camera sits in that
            // traffic, with nothing between lens and raised hand
            // (the first staging put the barrier arm across his
            // chest and the verb read as hiding).
            let so =
                soldier::soldier_mesh(&soldier::pose_halt(), Vec3::new(6.6, 0.0, 27.2), 0.0, 0.0);
            rig.still(
                &world,
                c35::scene::CameraPose {
                    eye: Vec3::new(4.6, 1.5, 21.0),
                    target: Vec3::new(6.6, 1.2, 27.2),
                    fov_y_deg: 50.0,
                },
                Some(&so),
                &format!("{out_dir}/pose-halt-checkpoint.png"),
            );
            rig.ledger("poses");
        }
        "drive" => {
            run_drive(&mut rig, &world, &blk, &out_dir, "open");
            run_drive(&mut rig, &world, &blk, &out_dir, "shadow");
            rig.ledger("drives");
        }
        other => {
            eprintln!("unknown --mode '{other}' (still|poses|drive|survey|windowed)");
            std::process::exit(2);
        }
    }
    print_reality(&reality);
}

/// The patrol's loop (UNAWARE): the west sidewalk north, across the
/// north end, back down the street's east lane, across the south —
/// a rectangle that closes on its own start, so the loop restarts
/// seamlessly.
pub(crate) fn patrol_legs() -> Vec<ScriptLeg> {
    vec![
        ScriptLeg::to(-8.6, 40.0, Gait::Walk),
        ScriptLeg::to(2.5, 40.0, Gait::Walk),
        ScriptLeg::to(2.5, -40.0, Gait::Walk),
        ScriptLeg::to(-8.6, -40.0, Gait::Walk),
    ]
}

pub(crate) fn patrol_start() -> Vec2 {
    Vec2::new(-8.6, -40.0)
}

pub(crate) fn new_patrol() -> Controller {
    Controller::new(patrol_start(), patrol_legs())
}

/// Rewind the loop in place. A fresh controller snaps yaw onto the
/// first leg and zeroes speed, which is a pop at the corner, and the
/// windowed host was sampling that fresh pose on the arrival frame
/// while the animator still held the arrival step. Position stays
/// where the arrival check left him; the next step turns him at the
/// gait's own rate.
pub(crate) fn restart_patrol(ctrl: &mut Controller) {
    ctrl.agent.done = false;
    ctrl.agent.leg = 0;
    ctrl.agent.gait = Gait::Walk;
}

fn run_drive(rig: &mut Rig, world: &WorldState, blk: &block::Block, out_dir: &str, which: &str) {
    let Some((start, legs)) = drive_route(which) else {
        eprintln!("unknown drive '{which}' (open|shadow)");
        std::process::exit(2);
    };
    let mut ctrl = Controller::new(start, legs);
    let mut anim = Animator::new();
    anim.set_ground_fn(Some(flat_ground));
    let mut p_ctrl = new_patrol();
    let mut p_anim = Animator::new();
    p_anim.set_ground_fn(Some(flat_ground));

    let view = rig.out_tex.create_view(&Default::default());
    // Warm both animators standing, place both men, warm the pipes.
    let a0 = *ctrl.agent();
    let warm = c35::locomotion::AnimInput {
        pos: Vec3::new(a0.pos.x, 0.0, a0.pos.y),
        yaw: a0.yaw,
        speed: 0.0,
        crouch: 0.0,
        aim: 0.0,
        slide: 0.0,
        carry: true,
    };
    let f0 = anim.update(0.0, &warm);
    let pa0 = *p_ctrl.agent();
    let pwarm = c35::locomotion::AnimInput {
        pos: Vec3::new(pa0.pos.x, 0.0, pa0.pos.y),
        yaw: pa0.yaw,
        ..warm
    };
    let pf0 = p_anim.update(0.0, &pwarm);
    let m0 = merge_meshes(
        &build_operator_frame(
            &f0,
            Vec3::new(a0.pos.x, 0.0, a0.pos.y),
            0.0,
            a0.yaw.to_degrees(),
        ),
        &soldier::soldier_mesh(
            &pf0,
            Vec3::new(pa0.pos.x, 0.0, pa0.pos.y),
            0.0,
            pa0.yaw.to_degrees(),
        ),
    );
    rig.renderer.update_operator(&m0);
    let mut anchor = Vec3::new(a0.pos.x, 0.0, a0.pos.y);
    for _ in 0..10 {
        rig.renderer
            .render(&view, &follow_pose(anchor, a0.yaw), world, 0.0);
        let _ = rig.renderer.device.poll(c35::wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
    }
    let _ = rig.renderer.drain_gpu_samples();

    let mut csv = String::from(LIGHT_READ_HEADER);
    let mut frame_ms: Vec<f64> = Vec::new();
    let mut settle = 0usize;
    let mut frames_run = 0usize;
    let mut lit_frames = 0usize;
    let dt = 1.0f32 / 60.0;
    for i in 0..20000usize {
        let input = ctrl.update(dt);
        let pframe = anim.update(dt, &input);
        let a = *ctrl.agent();
        if p_ctrl.agent().done {
            restart_patrol(&mut p_ctrl);
        }
        let p_input = p_ctrl.update(dt);
        let p_frame = p_anim.update(dt, &p_input);
        let pa = *p_ctrl.agent();
        let op_pos = Vec3::new(a.pos.x, 0.0, a.pos.y);
        let so_pos = Vec3::new(pa.pos.x, 0.0, pa.pos.y);
        let mesh = merge_meshes(
            &build_operator_frame(&pframe, op_pos, anim.ground_y(), a.yaw.to_degrees()),
            &soldier::soldier_mesh(&p_frame, so_pos, p_anim.ground_y(), pa.yaw.to_degrees()),
        );
        rig.renderer.update_operator(&mesh);
        let target_anchor = Vec3::new(a.pos.x, anim.ground_y(), a.pos.y);
        let k = 1.0 - (-dt / 0.14f32).exp();
        anchor = anchor + (target_anchor - anchor) * k;
        let pose = follow_pose(anchor, a.yaw);
        let t0 = Instant::now();
        rig.renderer.render(&view, &pose, world, i as f32 / 60.0);
        let _ = rig.renderer.device.poll(c35::wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        });
        frame_ms.push(t0.elapsed().as_secs_f64() * 1000.0);
        rig.renderer
            .observe_frame_ms(frame_ms.last().copied().unwrap_or(0.0));
        frames_run = i + 1;

        // THE LIGHT READ — the same field that baked the street,
        // sampled at the operator's chest, logged against the
        // patrol's facing. Not an AI: a measurement.
        let sample = light_sample(a.pos, input.crouch);
        let exposure = blk.field.exposure(sample);
        let lit = blk.field.lit(sample);
        if lit {
            lit_frames += 1;
        }
        let lamp = blk
            .field
            .nearest_lamp(sample)
            .map(|(idx, _)| idx)
            .unwrap_or(usize::MAX);
        csv.push_str(&light_read_row(i, &a, exposure, lit, lamp, &pa));
        if i % 300 == 0 || (a.done && settle == 0) {
            save_png_or_exit(
                &rig.renderer,
                &rig.out_tex,
                rig.width,
                rig.height,
                &format!("{out_dir}/drive-{which}-f{i}.png"),
            );
        }
        if a.done && a.speed < 0.05 {
            settle += 1;
            if settle >= 45 {
                break;
            }
        }
    }
    save_png_or_exit(
        &rig.renderer,
        &rig.out_tex,
        rig.width,
        rig.height,
        &format!("{out_dir}/drive-{which}-final.png"),
    );
    let csv_path = format!("{out_dir}/drive-{which}-lightread.csv");
    if let Err(e) = std::fs::write(&csv_path, &csv) {
        eprintln!("write {csv_path}: {e}");
        std::process::exit(1);
    }
    println!(
        "drive {which}: {frames_run} frames, {:.1} m travelled, lit {}/{} frames ({:.1}%), frame mean {:.2} ms p95 {:.2} ms",
        ctrl.agent().distance,
        lit_frames,
        frames_run,
        100.0 * lit_frames as f64 / frames_run.max(1) as f64,
        c35::mean(&frame_ms),
        c35::percentile(&frame_ms, 95.0)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_z_is_unix_zero() {
        let (unix, off) = parse_pin_iso("1970-01-01T00:00:00Z").unwrap();
        assert_eq!(off, 0.0);
        assert_eq!(unix, 0.0);
    }

    #[test]
    fn negative_offset_is_added_to_reach_utc() {
        let (unix, off) = parse_pin_iso("2026-01-15T08:00:00-06:00").unwrap();
        assert_eq!(off, -360.0);
        assert_eq!(c35::world::iso_utc(unix), "2026-01-15T14:00:00Z");
    }

    #[test]
    fn half_hour_offset_and_the_engine_calendar_agree() {
        let (unix, off) = parse_pin_iso("2026-01-15T08:00:00+05:30").unwrap();
        assert_eq!(off, 330.0);
        assert_eq!(c35::world::iso_utc(unix), "2026-01-15T02:30:00Z");
        let (midnight, _) = parse_pin_iso("2026-01-15T00:00:00Z").unwrap();
        let days = c35::world::days_from_civil(2026, 1, 15) as f64;
        assert_eq!(midnight, days * 86400.0);
    }

    #[test]
    fn offset_before_the_epoch_lands_on_it() {
        // 1969-12-31 23:00 at UTC-1 is 1970-01-01 00:00Z.
        let (unix, off) = parse_pin_iso("1969-12-31T23:00:00-01:00").unwrap();
        assert_eq!(off, -60.0);
        assert_eq!(unix, 0.0);
    }

    #[test]
    fn trailing_newline_keeps_the_minutes() {
        let clean = parse_pin_iso("2026-01-15T08:00:00-05:30").unwrap();
        let messy = parse_pin_iso("  2026-01-15T08:00:00-05:30\n").unwrap();
        assert_eq!(clean.1, -330.0);
        assert_eq!(clean.0, messy.0);
        assert_eq!(clean.1, messy.1);
    }

    #[test]
    fn fractional_second_is_kept() {
        let (unix, _) = parse_pin_iso("1970-01-01T00:00:00.5Z").unwrap();
        assert!((unix - 0.5).abs() < 1e-9);
    }

    #[test]
    fn seconds_and_minutes_may_be_omitted() {
        let (full, _) = parse_pin_iso("1970-01-01T01:02:03Z").unwrap();
        let (short, _) = parse_pin_iso("1970-01-01T01:02Z").unwrap();
        let (hour, _) = parse_pin_iso("1970-01-01T01Z").unwrap();
        assert!((full - (3600.0 + 120.0 + 3.0)).abs() < 1e-9);
        assert!((short - (3600.0 + 120.0)).abs() < 1e-9);
        assert!((hour - 3600.0).abs() < 1e-9);
    }

    #[test]
    fn leap_days_follow_the_gregorian_rule() {
        assert!(parse_pin_iso("2024-02-29T00:00:00Z").is_some());
        assert!(parse_pin_iso("2000-02-29T00:00:00Z").is_some());
        assert!(parse_pin_iso("2026-02-29T00:00:00Z").is_none());
        assert!(parse_pin_iso("1900-02-29T00:00:00Z").is_none());
    }

    #[test]
    fn impossible_pins_are_rejected() {
        for pin in [
            "2026-13-01T00:00:00Z",
            "2026-00-01T00:00:00Z",
            "2026-04-31T00:00:00Z",
            "2026-01-00T00:00:00Z",
            "2026-01-15T24:00:00Z",
            "2026-01-15T08:60:00Z",
            "2026-01-15T08:00:60Z",
            "2026-01-15T08:00:00-05:99",
            "2026-01-15T08:00:00-05:xx",
            "2026-01-15T08:aa:00Z",
            "2026-01-15T08:00:00Zulu",
            "not a pin",
            "",
            "   ",
            // A date hyphen is not an offset. The rig requires a zone.
            "2026-01-15T08:00:00",
        ] {
            assert!(parse_pin_iso(pin).is_none(), "{pin} should not parse");
        }
    }

    fn cli(args: &[&str]) -> Result<Cli, String> {
        let mut v = vec!["debrief".to_string()];
        v.extend(args.iter().map(|s| (*s).to_string()));
        parse_cli(&v)
    }

    #[test]
    fn default_cli_is_the_survey() {
        let c = cli(&[]).unwrap();
        assert_eq!(c.mode, "survey");
        assert_eq!(c.width, 1280);
        assert_eq!(c.height, 720);
        assert_eq!(c.present, "ps15");
        assert!(!c.proof);
        assert_eq!(c.smoke_secs, None);
    }

    #[test]
    fn bad_cli_fails_before_any_gpu() {
        assert!(cli(&["--mode", "fly"])
            .unwrap_err()
            .contains("unknown --mode"));
        assert!(cli(&["--modee", "survey"]).is_err());
        assert!(cli(&["--width", "0"]).is_err());
        assert!(cli(&["--width"]).is_err());
        assert!(cli(&["--height", "nope"]).is_err());
        assert!(cli(&["--present", "ps1"])
            .unwrap_err()
            .contains("unknown --present"));
        assert!(cli(&["--smoke", "foo"]).is_err());
        assert!(cli(&["--smoke", "-1", "--mode", "windowed"]).is_err());
        assert!(cli(&["--mode", "still", "--proof"])
            .unwrap_err()
            .contains("windowed"));
        assert!(cli(&["--mode", "windowed", "--proof", "--smoke", "1"])
            .unwrap_err()
            .contains("cannot be combined"));
        let c = cli(&[
            "--mode",
            "windowed",
            "--smoke",
            "0.5",
            "--present",
            "classic",
            "--width",
            "640",
        ])
        .unwrap();
        assert_eq!(c.smoke_secs, Some(0.5));
        assert_eq!(c.present, "classic");
        assert_eq!(c.width, 640);
    }

    #[test]
    fn drive_routes_are_only_the_two_names() {
        let (open, open_legs) = drive_route("open").unwrap();
        assert_eq!(open, Vec2::new(-2.0, -44.0));
        assert_eq!(open_legs.last().unwrap().target, Vec2::new(-2.0, 44.0));
        let (shadow, shadow_legs) = drive_route("shadow").unwrap();
        assert_eq!(shadow, Vec2::new(8.9, -44.0));
        assert_eq!(shadow_legs.len(), 1);
        assert!(drive_route("Shadow").is_none());
        assert!(drive_route("open ").is_none());
    }

    #[test]
    fn crouch_drops_the_light_sample_and_facing_is_signed() {
        let stand = light_sample(Vec2::new(1.0, 2.0), 0.0);
        let crouch = light_sample(Vec2::new(1.0, 2.0), 1.0);
        assert_eq!(stand, Vec3::new(1.0, 1.15, 2.0));
        assert!((stand.y - crouch.y - 0.35).abs() < 1e-6);
        let (dot, dist) = facing_dot(Vec2::ZERO, 0.0, Vec2::new(0.0, 5.0));
        assert!((dist - 5.0).abs() < 1e-5);
        assert!((dot - 1.0).abs() < 1e-5);
        let (behind, _) = facing_dot(Vec2::ZERO, 0.0, Vec2::new(0.0, -5.0));
        assert!((behind + 1.0).abs() < 1e-5);
        let (same, d0) = facing_dot(Vec2::ZERO, 1.2, Vec2::ZERO);
        assert_eq!(d0, 0.0);
        assert_eq!(same, 0.0);
    }

    #[test]
    fn patrol_loop_turns_the_corner_instead_of_snapping() {
        let mut ctrl = new_patrol();
        let dt = 1.0 / 60.0;
        let mut guard = 0;
        while !ctrl.agent().done {
            ctrl.update(dt);
            guard += 1;
            assert!(guard < 20_000, "patrol did not close the rectangle");
        }
        assert!(
            ctrl.agent().pos.distance(patrol_start()) < 0.4,
            "loop should close on its start, at {:?}",
            ctrl.agent().pos
        );
        restart_patrol(&mut ctrl);
        let before = *ctrl.agent();
        ctrl.update(dt);
        let after = *ctrl.agent();
        let yaw_delta = (after.yaw - before.yaw).abs();
        assert!(
            yaw_delta < 0.2,
            "yaw snapped by {yaw_delta} rad at the corner"
        );
        let step = after.pos.distance(before.pos);
        assert!(step < 0.05, "position jumped {step} m at the corner");
        assert!(!after.done);
        assert_eq!(after.leg, 0);
    }

    #[test]
    fn merge_offsets_the_second_figures_indices() {
        let a = op_mesh(
            &PoseFrame::from_pose(c35::OperatorPose::StandReady),
            Vec3::ZERO,
            0.0,
        );
        let b = soldier::soldier_mesh(
            &soldier::pose_low_ready(),
            Vec3::new(3.0, 0.0, 0.0),
            0.0,
            0.0,
        );
        let m = merge_meshes(&a, &b);
        assert_eq!(m.vertices.len(), a.vertices.len() + b.vertices.len());
        assert_eq!(m.indices.len(), a.indices.len() + b.indices.len());
        let base = a.vertices.len() as u32;
        for (got, idx) in m.indices[a.indices.len()..].iter().zip(b.indices.iter()) {
            assert_eq!(*got, idx + base);
        }
        assert!(m.indices.iter().all(|i| (*i as usize) < m.vertices.len()));
        let tris: u32 = m.parts.iter().map(|p| p.triangles).sum();
        assert_eq!(tris as usize * 3, m.indices.len());
        let row = light_read_row(
            0,
            &Agent {
                pos: Vec2::new(-2.0, -44.0),
                yaw: 0.0,
                speed: 0.0,
                leg: 0,
                gait: Gait::Walk,
                sliding: false,
                done: false,
                distance: 0.0,
            },
            0.5,
            true,
            3,
            &Agent {
                pos: Vec2::new(-8.6, -40.0),
                yaw: 0.0,
                speed: 1.5,
                leg: 0,
                gait: Gait::Walk,
                sliding: false,
                done: false,
                distance: 0.0,
            },
        );
        assert!(row.starts_with("0,0.000,-2.00,-44.00,walk,"));
        assert!(LIGHT_READ_HEADER.starts_with("frame,t,op_x,"));
    }
}
