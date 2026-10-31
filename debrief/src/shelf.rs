//! The Hotwire shelf — the host side of the Hotwire bridge
//! (hotwire SPEC, "Host contract").
//!
//! Hotwire (the converter) drops `.c35m` files into a models
//! folder; this module is the folder's consumer. With `--models
//! <dir>` the windowed host loads every model in the folder,
//! stands it on a shelf row along the street, and — polling the
//! folder about once a second — hot-swaps the geometry live when
//! a file changes: a successful reload replaces the slot's mesh,
//! a failed one keeps the old mesh and says so, a new file joins
//! the row, a deleted file's slot is removed and the row repacks.
//! The world never goes down for a bad export.
//!
//! Display only, by construction: a shelf model adds vertices to
//! the merged props upload and nothing else. It owns no collider
//! (the player's world is player_world(), untouched), it is never
//! an occluder (the light field's occluder list is block.rs's,
//! untouched), and nothing reads it back for gameplay — the field
//! is only SAMPLED at bake time, exactly as block.rs samples it
//! for the street's own vertices (the modeler bake's stated host
//! contract: the bake invents no light; the host's field supplies
//! it). Without `--models` none of this exists and every host
//! mode behaves exactly as before.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use c35::glam::Vec3;
use c35::modeler::{load_from, to_prop_mesh, Mesh};
use c35::props::PropMesh;

use crate::block::BLOCK_HALF_Z;
use crate::light::LightField;

/// The shelf row: the east side of the street at x = 5.5, first
/// anchor at z = 6.0 (just south of the operator's post at
/// (-2.0, 0, 3.0)), each next slot 2.5 m further toward -z —
/// down the hero camera's view of the street. Collision-free by
/// geography: the street slab spans x -7.2..7.2 (the row stands
/// on it, grounded at y 0); the wall boxes start at |x| = 10.4
/// and |z| = 48; the patrol's east leg is the line x = 2.5, his
/// west leg the sidewalk at x = -8.6, and the two crossings run
/// x = -2.0 and x = 8.9 — the row crosses none of them.
pub const SHELF_X: f32 = 5.5;
pub const SHELF_FIRST_Z: f32 = 6.0;
pub const SHELF_STEP_Z: f32 = -2.5;

/// World anchor of shelf slot `index` (grounding is per-model,
/// applied in [`Shelf::merged_props`]).
pub fn anchor(index: usize) -> Vec3 {
    Vec3::new(SHELF_X, 0.0, SHELF_FIRST_Z + SHELF_STEP_Z * index as f32)
}

/// Ground height under a point — the same street / sidewalk
/// split the windowed host's animator ground uses (sidewalk
/// slabs top at 0.12 inside the block, street at 0).
fn ground_height(x: f32, z: f32) -> f32 {
    if z.abs() <= BLOCK_HALF_Z && (7.2..=10.4).contains(&x.abs()) {
        0.12
    } else {
        0.0
    }
}

/// One placed model: the parsed mesh (survey counts read it),
/// its baked renderer geometry in MODEL space (translation to
/// the shelf happens at merge time, so a repack after a delete
/// is pure arithmetic), and the file state it was loaded from.
struct Slot {
    /// File name within the models folder (the sort key).
    name: String,
    /// mtime of the file version this slot was loaded from.
    mtime: SystemTime,
    mesh: Mesh,
    baked: PropMesh,
    /// Lowest vertex height in model space — the grounding datum.
    min_y: f32,
}

/// The models folder, watched. Slots are kept sorted by file
/// name at all times; the row's order is the folder's order.
pub struct Shelf {
    dir: PathBuf,
    slots: Vec<Slot>,
    /// (file name, mtime) of load attempts that failed — a bad
    /// file is reported once per version, not once per poll.
    failed: Vec<(String, SystemTime)>,
}

/// The folder's current `.c35m` census: (file name, path, mtime),
/// sorted by file name. A folder that cannot be read censuses as
/// empty (the caller has already reported why at open).
fn census(dir: &Path) -> Vec<(String, PathBuf, SystemTime)> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return out;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("c35m") {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let mtime = entry
            .metadata()
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);
        out.push((name.to_string(), path, mtime));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// Load + bake one model file. Parse failures and bake failures
/// (a material the engine's registry does not know) are distinct
/// messages; both are plain errors to the caller, never panics.
fn load_slot(name: &str, path: &Path, mtime: SystemTime) -> Result<Slot, String> {
    let mesh = load_from(path).map_err(|e| format!("load: {}", e.message))?;
    let baked = to_prop_mesh(&mesh).map_err(|e| format!("bake: {}", e.message))?;
    let min_y = mesh
        .verts
        .iter()
        .map(|v| v[1])
        .fold(f32::INFINITY, f32::min);
    Ok(Slot {
        name: name.to_string(),
        mtime,
        mesh,
        baked,
        min_y: if min_y.is_finite() { min_y } else { 0.0 },
    })
}

impl Shelf {
    /// Open the folder and place every model that loads. Bad
    /// files are reported and skipped; a missing folder reports
    /// once and shelves nothing (the session runs on, unshelved).
    pub fn open(dir: &Path) -> Shelf {
        let mut shelf = Shelf {
            dir: dir.to_path_buf(),
            slots: Vec::new(),
            failed: Vec::new(),
        };
        if !dir.is_dir() {
            println!("[shelf] {} is not a folder — shelf empty", dir.display());
            return shelf;
        }
        shelf.reconcile();
        println!(
            "[shelf] {} model(s) on the shelf from {}",
            shelf.slots.len(),
            dir.display()
        );
        shelf
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// One watch poll: reconcile the slots against the folder's
    /// current census. Returns true when the RENDERED set changed
    /// (a slot added, removed, or successfully reloaded) — the
    /// caller re-uploads props only then. A changed file whose
    /// reload fails keeps its old geometry, is reported once,
    /// and does not count as a change.
    pub fn poll(&mut self) -> bool {
        self.reconcile()
    }

    fn reconcile(&mut self) -> bool {
        let listing = census(&self.dir);
        let mut old: Vec<Slot> = std::mem::take(&mut self.slots);
        let old_failed = std::mem::take(&mut self.failed);
        let mut next_slots: Vec<Slot> = Vec::new();
        let mut next_failed: Vec<(String, SystemTime)> = Vec::new();
        let mut changed = false;
        for (name, path, mtime) in &listing {
            // Untouched file, slot already current: carry over.
            if let Some(pos) = old
                .iter()
                .position(|s| &s.name == name && s.mtime == *mtime)
            {
                next_slots.push(old.remove(pos));
                continue;
            }
            // This exact version already failed once: keep any
            // older geometry, stay silent, wait for a new version.
            if old_failed.iter().any(|(n, m)| n == name && m == mtime) {
                next_failed.push((name.clone(), *mtime));
                if let Some(pos) = old.iter().position(|s| &s.name == name) {
                    next_slots.push(old.remove(pos));
                }
                continue;
            }
            match load_slot(name, path, *mtime) {
                Ok(slot) => {
                    println!(
                        "[shelf] {}: {} vertices, {} faces",
                        slot.mesh.name,
                        slot.mesh.verts.len(),
                        slot.mesh.faces.len()
                    );
                    next_slots.push(slot);
                    changed = true;
                }
                Err(e) => {
                    if old.iter().any(|s| &s.name == name) {
                        println!("[shelf] {name}: reload failed ({e}) — keeping the old mesh");
                    } else {
                        println!("[shelf] {name}: skipped ({e})");
                    }
                    next_failed.push((name.clone(), *mtime));
                    if let Some(pos) = old.iter().position(|s| &s.name == name) {
                        next_slots.push(old.remove(pos));
                    }
                }
            }
        }
        // Whatever is left in `old` was deleted from the folder:
        // its slot goes away and the row repacks.
        for slot in &old {
            println!("[shelf] {}: removed", slot.name);
            changed = true;
        }
        self.slots = next_slots;
        self.failed = next_failed;
        changed
    }

    /// The props upload with the shelf stood on it: the block's
    /// own mesh first (unchanged, byte for byte), then each slot's
    /// baked geometry translated to its anchor — grounded so the
    /// model's lowest vertex rests on the surface under the
    /// anchor — with the baked lamp term sampled from the host's
    /// light field at each vertex's world position, exactly as
    /// block.rs bakes the street's vertices.
    pub fn merged_props(&self, base: &PropMesh, field: &LightField) -> PropMesh {
        let mut out = PropMesh {
            vertices: base.vertices.clone(),
            indices: base.indices.clone(),
        };
        for (i, slot) in self.slots.iter().enumerate() {
            let a = anchor(i);
            let off = Vec3::new(a.x, ground_height(a.x, a.z) - slot.min_y, a.z);
            let base_idx = out.vertices.len() as u32;
            for v in &slot.baked.vertices {
                let pos = Vec3::from_array(v.position) + off;
                let mut nv = *v;
                nv.position = pos.to_array();
                nv.light = field.exposure(pos);
                out.vertices.push(nv);
            }
            out.indices
                .extend(slot.baked.indices.iter().map(|ix| ix + base_idx));
        }
        out
    }
}

/// The survey's shelf section: one line per model file in the
/// folder — name, vertices, faces — straight from the engine's
/// own loader, so the seam is verifiable without a window. A
/// file that parses but cannot bake (unknown material) still
/// gets its census line, plus the reason it cannot be placed;
/// a file that does not parse is reported and skipped.
pub fn print_survey(dir: &Path) {
    println!("--- hotwire shelf ({}) ---", dir.display());
    if !dir.is_dir() {
        println!("[shelf] {} is not a folder", dir.display());
        return;
    }
    let listing = census(dir);
    if listing.is_empty() {
        println!("[shelf] no .c35m models in folder");
        return;
    }
    for (name, path, _mtime) in &listing {
        match load_from(path) {
            Ok(mesh) => {
                println!(
                    "{}: {} vertices, {} faces",
                    mesh.name,
                    mesh.verts.len(),
                    mesh.faces.len()
                );
                if let Err(e) = to_prop_mesh(&mesh) {
                    println!("[shelf] {name}: cannot place ({})", e.message);
                }
            }
            Err(e) => println!("[shelf] {name}: skipped (load: {})", e.message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use c35::modeler::save;
    use std::time::Duration;

    /// A writable one-metre box model file (painted-metal —
    /// a material the engine's registry knows).
    fn write_box(dir: &Path, file: &str, name: &str, size: f32) {
        let mut m = Mesh::new(name);
        let mat = m.add_material("painted-metal");
        m.add_box([0.0, 0.0, 0.0], [size, size, size], mat);
        std::fs::write(dir.join(file), save(&m)).unwrap();
    }

    /// Set a file's mtime explicitly — poll tests must not race
    /// the filesystem's timestamp granularity.
    fn bump(path: &Path, secs: u64) {
        let f = std::fs::File::options().write(true).open(path).unwrap();
        f.set_modified(SystemTime::UNIX_EPOCH + Duration::from_secs(secs))
            .unwrap();
    }

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("debrief-shelf-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn empty_field() -> LightField {
        LightField {
            lamps: Vec::new(),
            occluders: Vec::new(),
        }
    }

    #[test]
    fn open_places_models_sorted_by_file_name() {
        let dir = scratch("sorted");
        write_box(&dir, "b-box.c35m", "b", 1.0);
        write_box(&dir, "a-box.c35m", "a", 2.0);
        let shelf = Shelf::open(&dir);
        assert_eq!(shelf.slots.len(), 2);
        assert_eq!(shelf.slots[0].name, "a-box.c35m");
        assert_eq!(shelf.slots[1].name, "b-box.c35m");
        // Anchors step down the street at the fixed spacing.
        assert_eq!(anchor(0).z - anchor(1).z, -SHELF_STEP_Z);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn garbage_files_are_skipped_never_fatal() {
        let dir = scratch("garbage");
        std::fs::write(dir.join("junk.c35m"), "this is not a model\n").unwrap();
        std::fs::write(dir.join("half.c35m"), "c35-model 1\nname x\nv 0 0").unwrap();
        write_box(&dir, "good.c35m", "good", 1.0);
        let shelf = Shelf::open(&dir);
        assert_eq!(shelf.slots.len(), 1);
        assert_eq!(shelf.slots[0].name, "good.c35m");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn poll_swaps_changed_and_keeps_old_on_garbage() {
        let dir = scratch("swap");
        let path = dir.join("box.c35m");
        write_box(&dir, "box.c35m", "box", 1.0);
        bump(&path, 1_000_000);
        let mut shelf = Shelf::open(&dir);
        assert_eq!(shelf.slots.len(), 1);
        let verts_before = shelf.slots[0].baked.vertices.len();

        // No change on disk: no change reported.
        assert!(!shelf.poll());

        // A real change (a subdivided box = more geometry):
        // the swap lands and the poll reports it.
        let mut m = Mesh::new("box");
        let mat = m.add_material("painted-metal");
        m.add_box([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], mat);
        m.add_box([2.0, 0.0, 0.0], [3.0, 1.0, 1.0], mat);
        std::fs::write(&path, save(&m)).unwrap();
        bump(&path, 1_000_100);
        assert!(shelf.poll());
        assert!(shelf.slots[0].baked.vertices.len() > verts_before);
        let verts_after_swap = shelf.slots[0].baked.vertices.len();

        // The export goes bad mid-write (truncated): the old
        // mesh stays, the poll reports no rendered change, and
        // the failure is not retried until the file changes.
        std::fs::write(&path, "c35-model 1\nname box\nv 1 2").unwrap();
        bump(&path, 1_000_200);
        assert!(!shelf.poll());
        assert_eq!(shelf.slots.len(), 1);
        assert_eq!(shelf.slots[0].baked.vertices.len(), verts_after_swap);
        assert!(!shelf.poll());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn poll_adds_and_removes_and_repacks() {
        let dir = scratch("repack");
        write_box(&dir, "a.c35m", "a", 1.0);
        write_box(&dir, "b.c35m", "b", 1.0);
        bump(&dir.join("a.c35m"), 2_000_000);
        bump(&dir.join("b.c35m"), 2_000_000);
        let mut shelf = Shelf::open(&dir);
        assert_eq!(shelf.slots.len(), 2);

        write_box(&dir, "c.c35m", "c", 1.0);
        bump(&dir.join("c.c35m"), 2_000_100);
        assert!(shelf.poll());
        assert_eq!(shelf.slots.len(), 3);

        // Deleting the first file repacks the row: the former
        // second slot now stands at anchor 0 (verified through
        // the merged geometry's centroid: the unit box's centre
        // rides 0.5 in from its origin corner).
        std::fs::remove_file(dir.join("a.c35m")).unwrap();
        assert!(shelf.poll());
        assert_eq!(shelf.slots.len(), 2);
        assert_eq!(shelf.slots[0].name, "b.c35m");
        let merged = shelf.merged_props(&PropMesh::new(), &empty_field());
        let n = shelf.slots[0].baked.vertices.len();
        let c = merged.vertices[..n].iter().fold([0.0f32; 3], |a, v| {
            [
                a[0] + v.position[0] / n as f32,
                a[1] + v.position[1] / n as f32,
                a[2] + v.position[2] / n as f32,
            ]
        });
        assert!((c[0] - (SHELF_X + 0.5)).abs() < 0.01, "centroid {c:?}");
        assert!(
            (c[2] - (SHELF_FIRST_Z + 0.5)).abs() < 0.01,
            "centroid {c:?}"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn merged_props_grounds_models_on_the_street() {
        let dir = scratch("ground");
        // A model authored floating: its lowest vertex at y 2.
        let mut m = Mesh::new("float");
        let mat = m.add_material("painted-metal");
        m.add_box([0.0, 2.0, 0.0], [1.0, 3.0, 1.0], mat);
        std::fs::write(dir.join("float.c35m"), save(&m)).unwrap();
        let shelf = Shelf::open(&dir);
        let merged = shelf.merged_props(&PropMesh::new(), &empty_field());
        let min_y = merged
            .vertices
            .iter()
            .map(|v| v.position[1])
            .fold(f32::INFINITY, f32::min);
        // The shelf stands on the street (x 5.5): ground is y 0.
        assert!(min_y.abs() < 1e-6, "grounded min_y = {min_y}");
        // The base mesh passes through untouched in front.
        let base = PropMesh::new();
        let with_base = shelf.merged_props(&base, &empty_field());
        assert_eq!(
            with_base.vertices.len(),
            base.vertices.len() + merged.vertices.len()
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
