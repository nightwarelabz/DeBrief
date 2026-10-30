//! One crossing of the block — the first op.
//!
//! The operator is the engine's traversal sim (walk, crouch,
//! sprint, jump, the grapple). The patrol is the engine's
//! conscript, priced from the block's own light. The op ends
//! when he reaches the south end, or when the file is closed
//! early. Either way the handler reads it back. There is no
//! reload.
//!
//! Nightjars perch on this block. Hearing one files nothing
//! but a count; finding one files the same count and no page.
//! The handler does not remark on them.

use c35::glam::Vec3;
use c35::locomotion::{AnimInput, Animator, CROUCH_SPEED, SPRINT_SPEED, WALK_SPEED};
use c35::perception::field::LightField as EngineField;
use c35::perception::pricing::Posture;
use c35::perception::soldier::{PostureSignal, SoldierInput, SoldierState, VoiceLine};
use c35::perception::sound::propagate;
use c35::scene::CameraPose;
use c35::traversal::events::StimulusKind;
use c35::traversal::physics::{BoxCollider, StaticWorld, FIXED_DT};
use c35::traversal::surface::{SurfaceKind, SurfaceRecord};
use c35::traversal::{BodyMode, TravInput, TravSim};
use c35::world::WorldState;

use crate::light::{LightField, Occluder};
use crate::sense::{self, Conscript, SOLDIER_EYE_Y};
use crate::soldier;

/// Par time for the grade the Agency keeps for a crossing that
/// left no trace at all. A crouch the length of the block fits
/// under it; waiting out a lamp may not, and that is the price.
pub const PAR_S: f32 = 90.0;

const EXTRACT_Z: f32 = 42.0;
const MAX_S: f32 = 180.0;
const HEAR_R: f32 = 14.0;
const FIND_R: f32 = 3.2;

/// The Agency's words for a file. The grade is a sentence, not
/// a score.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Grade {
    GhostThatNeverWas,
    Ghost,
    Surgeon,
    Butcher,
}

impl Grade {
    pub fn token(self) -> &'static str {
        match self {
            Grade::GhostThatNeverWas => "ghost-that-never-was",
            Grade::Ghost => "ghost",
            Grade::Surgeon => "surgeon",
            Grade::Butcher => "butcher",
        }
    }

    pub fn spoken(self) -> &'static str {
        match self {
            Grade::GhostThatNeverWas => "ghost that never was",
            Grade::Ghost => "ghost",
            Grade::Surgeon => "surgeon",
            Grade::Butcher => "butcher",
        }
    }
}

/// What the debrief is allowed to know. Built at close, graded
/// as a pure function, filed as text.
#[derive(Clone, Debug)]
pub struct OpFacts {
    pub duration_s: f32,
    pub alarmed: bool,
    pub suspicious: bool,
    pub shots: u32,
    pub bodies: u32,
    pub witnesses: u32,
    pub nightjars_heard: u32,
    pub nightjars_found: u32,
    pub closest_soldier_m: f32,
    pub extracted: bool,
    pub burned: bool,
}

pub fn grade(op: &OpFacts) -> Grade {
    if op.bodies > 0 {
        return Grade::Butcher;
    }
    if op.alarmed || op.shots > 0 || op.witnesses > 0 {
        return Grade::Surgeon;
    }
    if op.extracted && !op.suspicious && !op.burned && op.duration_s <= PAR_S {
        return Grade::GhostThatNeverWas;
    }
    Grade::Ghost
}

fn format_time(seconds: f32) -> String {
    let total = seconds.max(0.0).round() as u32;
    format!("{:02}:{:02}", total / 60, total % 60)
}

/// The handler, across the desk. Short, methodical, and done.
/// He does not mention the birds, and he does not answer a
/// mystery. The margin is not part of this speech.
pub fn handler_speech(op: &OpFacts, grade: Grade) -> String {
    let open = if op.extracted {
        "You crossed the block. North mouth to the checkpoint end."
    } else {
        "You were still on the block when the file closed."
    };
    let middle = match grade {
        Grade::GhostThatNeverWas => "The patrol's sheet has no line for you. No shot. No body.",
        Grade::Ghost => {
            "Something may have turned its head. The street did not keep it. No shot. No body."
        }
        Grade::Surgeon if op.shots > 0 && op.alarmed => {
            "You fired on the block. The patrol answered it. There is no body to put with it."
        }
        Grade::Surgeon if op.shots > 0 => {
            "You fired on the block. There is no body to put with it."
        }
        Grade::Surgeon if op.witnesses > 0 && !op.alarmed => {
            "Someone saw enough to file it. There is no body."
        }
        Grade::Surgeon => "He saw you. That goes on the sheet. You left no body.",
        Grade::Butcher if op.witnesses > 0 => {
            "The street saw the work. There is a body on the block."
        }
        Grade::Butcher => "There is a body on the block. The record prices that.",
    };
    let mut speech = format!(
        "{open} {middle} Time on the street: {}. The file will say: {}.",
        format_time(op.duration_s),
        grade.spoken()
    );
    if op.burned {
        speech.push_str(" The career ends here. The city keeps the body where it fell.");
    }
    speech
}

/// The record the archive reads. One op, one block of lines,
/// deterministic for the same facts.
pub fn format_record(op: &OpFacts, grade: Grade) -> String {
    format!(
        "\
DEBRIEF/1
op: block-cross
district: dockyards
extracted: {extracted}
burned: {burned}
duration_s: {duration:.2}
alarmed: {alarmed}
suspicious: {suspicious}
shots: {shots}
bodies: {bodies}
witnesses: {witnesses}
nightjars_heard: {heard}
nightjars_found: {found}
closest_soldier_m: {closest:.2}
grade: {grade}
",
        extracted = yesno(op.extracted),
        burned = yesno(op.burned),
        duration = op.duration_s,
        alarmed = yesno(op.alarmed),
        suspicious = yesno(op.suspicious),
        shots = op.shots,
        bodies = op.bodies,
        witnesses = op.witnesses,
        heard = op.nightjars_heard,
        found = op.nightjars_found,
        closest = op.closest_soldier_m,
        grade = grade.token(),
    )
}

fn yesno(v: bool) -> &'static str {
    if v {
        "yes"
    } else {
        "no"
    }
}

/// The margin, printed under the speech, not inside it.
pub fn margin_line(op: &OpFacts) -> String {
    format!("Nightjars: {}", op.nightjars_found)
}

#[derive(Copy, Clone, Debug)]
struct Perch {
    x: f32,
    z: f32,
}

fn perches() -> [Perch; 3] {
    [
        // The roller-door gantry, on the shadow side.
        Perch { x: 9.2, z: -36.0 },
        // A cross-span, over the middle of the street.
        Perch { x: 1.4, z: 13.5 },
        // The tenement parapet, on the lit side.
        Perch { x: -8.6, z: 4.0 },
    ]
}

/// Hear, then maybe find. Returns the one line the player is
/// owed, or nothing. No direction, no name, no count.
pub fn consider_birds(
    night: bool,
    x: f32,
    z: f32,
    speed: f32,
    crouch: bool,
    heard: &mut [bool; 3],
    found: &mut [bool; 3],
) -> Option<&'static str> {
    if !night {
        return None;
    }
    let spots = perches();
    for i in 0..3 {
        let d = (spots[i].x - x).hypot(spots[i].z - z);
        if !found[i] && d <= FIND_R && crouch && speed <= 0.5 {
            found[i] = true;
            heard[i] = true;
            return Some("You find the bird. You leave it where it is.");
        }
        if !heard[i] && d <= HEAR_R {
            heard[i] = true;
            return Some("A churring, low, from somewhere above the street.");
        }
    }
    None
}

/// One tick of intent. `wish` is horizontal world space, not
/// necessarily unit. `aim` is the look direction for the claw.
#[derive(Clone, Debug)]
pub struct Wish {
    pub wish: Vec3,
    pub speed: f32,
    pub crouch: bool,
    pub jump: bool,
    pub hook_press: bool,
    pub hook_held: bool,
    pub fire: bool,
    pub aim: Vec3,
}

impl Wish {
    pub fn still() -> Wish {
        Wish {
            wish: Vec3::ZERO,
            speed: 0.0,
            crouch: true,
            jump: false,
            hook_press: false,
            hook_held: false,
            fire: false,
            aim: Vec3::new(0.0, 0.0, 1.0),
        }
    }
}

/// Horizontal forward for the fly-convention yaw: 0 looks −Z
/// (north, toward the crane).
pub fn look_forward(yaw: f32, pitch: f32) -> Vec3 {
    let (sy, cy) = yaw.sin_cos();
    let (sp, cp) = pitch.sin_cos();
    Vec3::new(-sy * cp, sp, -cy * cp)
}

pub fn horizontal_facing(yaw: f32) -> Vec3 {
    let (sy, cy) = yaw.sin_cos();
    Vec3::new(-sy, 0.0, -cy)
}

/// WASD in the look frame, into a world-space wish.
pub fn wish_from_axes(
    yaw: f32,
    forward: f32,
    strafe: f32,
    crouch: bool,
    sprint: bool,
) -> (Vec3, f32) {
    let facing = horizontal_facing(yaw);
    let right = Vec3::new(facing.z, 0.0, -facing.x);
    let wish = facing * forward + right * strafe;
    let moving = wish.length() > 0.05;
    let speed = if !moving {
        0.0
    } else if crouch {
        CROUCH_SPEED
    } else if sprint {
        SPRINT_SPEED
    } else {
        WALK_SPEED
    };
    (wish, speed)
}

/// Static boxes the capsule resolves against: the street, the
/// two sidewalks, and the block's own occluders (warehouses,
/// tenements, the kiosk, the end caps).
pub fn street_physics(occluders: &[Occluder]) -> StaticWorld {
    let mut world = StaticWorld::new();
    world.add(BoxCollider::new(
        Vec3::new(-30.0, -1.0, -80.0),
        Vec3::new(30.0, 0.0, 80.0),
        SurfaceRecord::dry(SurfaceKind::Asphalt),
    ));
    world.add(BoxCollider::new(
        Vec3::new(-10.4, 0.0, -48.0),
        Vec3::new(-7.2, 0.12, 48.0),
        SurfaceRecord::dry(SurfaceKind::Concrete),
    ));
    world.add(BoxCollider::new(
        Vec3::new(7.2, 0.0, -48.0),
        Vec3::new(10.4, 0.12, 48.0),
        SurfaceRecord::dry(SurfaceKind::Concrete),
    ));
    for o in occluders {
        let kind = if o.max.y < 4.0 {
            SurfaceKind::Wood
        } else {
            SurfaceKind::Brick
        };
        world.add(BoxCollider::new(o.min, o.max, SurfaceRecord::dry(kind)));
    }
    world
}

pub struct Start {
    pub night: bool,
    pub rain: f32,
    pub cold: f32,
}

/// The live op.
pub struct Crossing {
    pub trav: TravSim,
    physics: StaticWorld,
    engine_field: EngineField,
    pub soldier: Conscript,
    anim: Animator,
    pub last_pose: c35::operator::PoseFrame,
    pub last_posture: PostureSignal,
    event_cursor: usize,
    step_acc: f32,
    pub look_yaw: f32,
    pub look_pitch: f32,
    pub fov: f32,
    pub crouching: bool,
    pub sprinting: bool,
    /// Right-mouse aim, set by the host before the tick. Narrows
    /// the eye. The sim does not read the mouse itself.
    pub aiming: bool,
    fire_cd: f32,
    /// Seconds a shot keeps ringing in nearby ears. One tick of
    /// audio is a footfall; a shot has to last long enough for a
    /// conscript to believe it.
    shot_echo: f32,
    shot_pos: Vec3,
    pub shots: u32,
    pub t: f32,
    pub alarmed: bool,
    pub suspicious: bool,
    pub closest: f32,
    pub extracted: bool,
    pub night: bool,
    pub rain: f32,
    pub cold: f32,
    heard_bird: [bool; 3],
    found_bird: [bool; 3],
    pub notices: Vec<&'static str>,
    pub voices: Vec<VoiceLine>,
}

impl Crossing {
    /// North mouth, on the east sidewalk, facing south.
    pub fn begin(occluders: &[Occluder], field: &LightField, start: Start) -> Crossing {
        let physics = street_physics(occluders);
        let origin = Vec3::new(8.4, 0.2, -40.0);
        let mut trav = TravSim::new(origin);
        if let Some(g) = physics.ground_at(origin.x, origin.z) {
            trav.body.feet.y = g.height;
            trav.body.grounded = true;
        }
        let mut anim = Animator::new();
        anim.set_ground_fn(Some(crate::flat_ground));
        let soldier = Conscript::on_route(sense::block_route());
        let warm = anim.update(
            0.0,
            &AnimInput {
                pos: soldier.pos,
                yaw: soldier.yaw,
                speed: 0.0,
                crouch: 0.0,
                aim: 0.0,
                slide: 0.0,
                carry: true,
            },
        );
        Crossing {
            trav,
            physics,
            engine_field: sense::engine_field(field),
            soldier,
            anim,
            last_pose: warm,
            last_posture: PostureSignal::AtEase,
            event_cursor: 0,
            step_acc: 0.0,
            look_yaw: std::f32::consts::PI,
            look_pitch: 0.0,
            fov: 50.0,
            crouching: false,
            sprinting: false,
            aiming: false,
            fire_cd: 0.0,
            shot_echo: 0.0,
            shot_pos: Vec3::ZERO,
            shots: 0,
            t: 0.0,
            alarmed: false,
            suspicious: false,
            closest: f32::MAX,
            extracted: false,
            night: start.night,
            rain: start.rain.clamp(0.0, 1.0),
            cold: start.cold.clamp(0.0, 1.0),
            heard_bird: [false; 3],
            found_bird: [false; 3],
            notices: Vec::new(),
            voices: Vec::new(),
        }
    }

    pub fn finished(&self) -> bool {
        self.extracted || self.t >= MAX_S
    }

    pub fn eye(&self) -> Vec3 {
        let drop = if self.crouching { 0.48 } else { 0.0 };
        self.trav.body.feet + Vec3::new(0.0, c35::operator::VIEWMODEL_EYE_Y - drop, 0.0)
    }

    pub fn camera(&self) -> CameraPose {
        let eye = self.eye();
        CameraPose {
            eye,
            target: eye + look_forward(self.look_yaw, self.look_pitch),
            fov_y_deg: self.fov,
        }
    }

    pub fn soldier_mesh(&self) -> c35::operator::OperatorMesh {
        soldier::soldier_mesh(
            &self.last_pose,
            self.soldier.pos,
            self.anim.ground_y(),
            self.soldier.yaw.to_degrees(),
        )
    }

    /// Where the claw is committed, the viewmodel shows the
    /// launcher. Otherwise the USP, low or aimed.
    pub fn launcher_up(&self) -> bool {
        self.trav.mode == BodyMode::Grapple || self.trav.grapple.hands_committed()
    }

    pub fn launcher_swinging(&self) -> bool {
        self.trav.grapple.is_latched() && self.trav.body.horizontal_speed() > 1.5
    }

    pub fn facts(&self) -> OpFacts {
        let closest = if self.closest.is_finite() {
            self.closest
        } else {
            999.0
        };
        OpFacts {
            duration_s: self.t,
            alarmed: self.alarmed,
            suspicious: self.suspicious,
            shots: self.shots,
            bodies: 0,
            witnesses: 0,
            nightjars_heard: self.heard_bird.iter().filter(|h| **h).count() as u32,
            nightjars_found: self.found_bird.iter().filter(|h| **h).count() as u32,
            closest_soldier_m: closest,
            extracted: self.extracted,
            burned: false,
        }
    }

    /// One 240 Hz step.
    pub fn tick(&mut self, wish: &Wish, field: &LightField) {
        if self.finished() {
            return;
        }
        self.crouching = wish.crouch;
        self.sprinting = wish.speed >= SPRINT_SPEED - 0.01 && !wish.crouch;
        self.fire_cd = (self.fire_cd - FIXED_DT).max(0.0);
        let facing = horizontal_facing(self.look_yaw);
        let wish_h = Vec3::new(wish.wish.x, 0.0, wish.wish.z);
        let wish_n = if wish_h.length() > 1e-4 {
            wish_h.normalize()
        } else {
            Vec3::ZERO
        };
        let speed = if wish_n == Vec3::ZERO {
            0.0
        } else {
            wish.speed
        };
        {
            let physics = &self.physics;
            self.trav.tick(
                TravInput {
                    wish: wish_n,
                    facing,
                    speed,
                    jump: wish.jump,
                    hook_press: wish.hook_press,
                    hook_held: wish.hook_held,
                    crouch: wish.crouch,
                    aim: if wish.aim.length() > 1e-4 {
                        wish.aim.normalize()
                    } else {
                        facing
                    },
                },
                physics,
            );
        }
        // Ground footfalls. Wall-run already prices its own.
        // Crouch cuts the gain: the engine's surface record does
        // not know a crouch, and a crouch-walk is the quiet gait.
        if self.trav.mode != BodyMode::WallRun
            && self.trav.body.grounded
            && self.trav.body.horizontal_speed() > 0.3
        {
            self.step_acc += FIXED_DT;
            let interval = if wish.crouch { 0.55 } else { 0.38 };
            if self.step_acc >= interval {
                self.step_acc = 0.0;
                let surface = self
                    .physics
                    .ground_at(self.trav.body.feet.x, self.trav.body.feet.z)
                    .map(|g| g.surface)
                    .unwrap_or_else(|| SurfaceRecord::dry(SurfaceKind::Asphalt));
                let quiet = if wish.crouch { 0.45 } else { 1.0 };
                let mag = surface.footstep_gain(self.trav.body.horizontal_speed()) * quiet;
                let tick = self.trav.tick;
                let feet = self.trav.body.feet;
                self.trav
                    .events
                    .push(tick, StimulusKind::Footstep, feet, surface, mag);
            }
        }
        if wish.fire && self.fire_cd <= 0.0 {
            self.shots += 1;
            self.fire_cd = 0.45;
            // The armoury milestone owns a gunshot kind. Until
            // then a shot is carried as a strike — louder than a
            // footfall, same propagation law, same ears.
            let tick = self.trav.tick;
            let surface = SurfaceRecord::dry(SurfaceKind::Asphalt);
            let at = self.trav.body.feet;
            self.trav
                .events
                .push(tick, StimulusKind::Strike, at, surface, 3.0);
            self.shot_echo = 1.0;
            self.shot_pos = at;
        }
        let fresh_from = self.event_cursor;
        let ear = self.soldier.pos + Vec3::new(0.0, SOLDIER_EYE_Y, 0.0);
        let mut heard = {
            let events = &self.trav.events.events[fresh_from..];
            propagate(events, ear, &self.engine_field)
        };
        self.event_cursor = self.trav.events.events.len();
        // A shot rings for a second in earshot. The strike event
        // itself is one tick, and one tick is not enough for a
        // conscript's attention to believe it.
        if self.shot_echo > 0.0 {
            let d = (self.shot_pos - ear).length();
            let radius = c35::perception::sound::hear_radius(StimulusKind::Strike);
            if d < radius {
                let fall = 1.0 - d / radius;
                let dir = if d > 1e-4 {
                    (self.shot_pos - ear) / d
                } else {
                    Vec3::Z
                };
                heard.push(c35::perception::sound::HeardEvent {
                    kind: StimulusKind::Strike,
                    pos: self.shot_pos,
                    heard: (2.0 * fall).max(0.45),
                    dir,
                });
            }
            self.shot_echo = (self.shot_echo - FIXED_DT).max(0.0);
        }

        let feet = self.trav.body.feet;
        let crouch = if wish.crouch { 1.0 } else { 0.0 };
        let sight = sense::price_sight(
            field,
            self.soldier.pos,
            self.soldier.yaw,
            feet,
            self.trav.body.horizontal_speed(),
            crouch,
        );
        let seen = sight.visibility > 0.0;
        let out = self.soldier.tick(
            SoldierInput {
                visibility: sight.visibility,
                target_pos: if seen {
                    Some(Vec3::new(feet.x, sense::chest_y(crouch), feet.z))
                } else {
                    None
                },
                target_posture: Some(if wish.crouch {
                    Posture::Crouch
                } else {
                    Posture::Upright
                }),
                heard,
                rain: self.rain,
                cold: self.cold,
                ..SoldierInput::default()
            },
            FIXED_DT,
        );
        if let Some(v) = out.voice {
            self.voices.push(v);
        }
        match out.state {
            SoldierState::Suspicious | SoldierState::Search => self.suspicious = true,
            SoldierState::Alarmed | SoldierState::Combat | SoldierState::BodyFound => {
                self.suspicious = true;
                self.alarmed = true;
            }
            _ => {}
        }
        self.last_posture = out.posture;
        let aim = sense::aim_for(out.posture);
        if sense::uses_halt(out.posture) {
            self.last_pose = soldier::pose_halt();
        } else {
            self.last_pose = self.anim.update(
                FIXED_DT,
                &AnimInput {
                    pos: self.soldier.pos,
                    yaw: self.soldier.yaw,
                    speed: self.soldier.speed,
                    crouch: 0.0,
                    aim,
                    slide: 0.0,
                    carry: true,
                },
            );
        }
        let dist = (self.soldier.pos.x - feet.x).hypot(self.soldier.pos.z - feet.z);
        self.closest = self.closest.min(dist);
        if let Some(line) = consider_birds(
            self.night,
            feet.x,
            feet.z,
            self.trav.body.horizontal_speed(),
            wish.crouch,
            &mut self.heard_bird,
            &mut self.found_bird,
        ) {
            self.notices.push(line);
        }
        self.t += FIXED_DT;
        if feet.z >= EXTRACT_Z && feet.x > -10.0 && feet.x < 10.5 {
            self.extracted = true;
        }
        let target_fov = if self.aiming {
            45.0
        } else if self.sprinting {
            55.0
        } else {
            50.0
        };
        let k = 1.0 - (-FIXED_DT / 0.12).exp();
        self.fov += (target_fov - self.fov) * k;
    }
}

/// A careful crouch down the warehouse shadow, waiting when the
/// next step would put him in the conscript's read.
pub struct CarefulTape {
    pub waited: f32,
}

impl CarefulTape {
    pub fn wish(&mut self, c: &Crossing, field: &LightField) -> Wish {
        let feet = c.trav.body.feet;
        let target_x = if (26.0..34.0).contains(&feet.z) {
            5.6
        } else {
            8.4
        };
        let next = Vec3::new(target_x, feet.y, feet.z + 0.5);
        let preview =
            sense::price_sight(field, c.soldier.pos, c.soldier.yaw, next, CROUCH_SPEED, 1.0);
        let notice = c35::perception::pricing::Attention::soldier().notice;
        let mut wish = Wish::still();
        wish.aim = horizontal_facing(std::f32::consts::PI);
        if preview.visibility > notice && self.waited < 12.0 && !c.alarmed {
            self.waited += FIXED_DT;
            wish.speed = 0.0;
            wish.wish = Vec3::ZERO;
        } else {
            if preview.visibility <= notice {
                self.waited = 0.0;
            }
            wish.wish = Vec3::new(target_x - feet.x, 0.0, 1.0);
            wish.speed = CROUCH_SPEED;
        }
        wish.crouch = true;
        wish
    }
}

/// Sprint the lit spine. This is how a file becomes a surgeon.
pub fn lit_wish(c: &Crossing) -> Wish {
    let feet = c.trav.body.feet;
    Wish {
        wish: Vec3::new(-2.0 - feet.x, 0.0, 1.0),
        speed: SPRINT_SPEED,
        crouch: false,
        jump: false,
        hook_press: false,
        hook_held: false,
        fire: false,
        aim: horizontal_facing(std::f32::consts::PI),
    }
}

pub fn debrief_text(c: &Crossing) -> String {
    let facts = c.facts();
    let g = grade(&facts);
    format!("{}\n\n{}", handler_speech(&facts, g), margin_line(&facts))
}

/// Night from the reality layer: the sun more than 6° down.
pub fn is_night(world: &WorldState) -> bool {
    world.sun_elevation_deg < -6.0
}

/// One sample of the engine's weather, pinned by a fixed seed
/// and the world's clock, so the same instant is the same rain.
pub fn sky_rain(world: &WorldState) -> (f32, f32) {
    let hours = (world.unix_seconds / 3600.0) as f32;
    let day = world.day_of_year.round().max(1.0) as u32;
    let mut sky = c35::weather::WeatherState::new(0x6D_C0_1D, hours, day);
    sky.tick(0.0, hours, day, world.daylight, world.cold);
    (sky.rain(), world.cold)
}

pub fn run_headless(
    args: &[String],
    field: &LightField,
    occluders: &[Occluder],
    world: &WorldState,
) {
    let tape = args
        .windows(2)
        .find(|w| w[0] == "--tape")
        .map(|w| w[1].as_str())
        .unwrap_or("careful");
    let force_night = args.iter().any(|a| a == "--night");
    let rain_arg = args
        .windows(2)
        .find(|w| w[0] == "--rain")
        .and_then(|w| w[1].parse().ok());
    let (sky_rain, cold) = sky_rain(world);
    let rain = rain_arg.unwrap_or(sky_rain);
    let night = force_night || is_night(world);
    println!(
        "crossing         : tape {tape}, night {}, rain {rain:.2}, cold {cold:.2}",
        yesno(night)
    );
    let mut crossing = Crossing::begin(occluders, field, Start { night, rain, cold });
    // The tape faces south from the first tick. The camera yaw
    // is the wish's facing, already south (π).
    crossing.look_yaw = std::f32::consts::PI;
    let mut careful = CarefulTape { waited: 0.0 };
    let mut fired = false;
    while !crossing.finished() {
        let mut wish = match tape {
            "lit" => lit_wish(&crossing),
            "loud" => careful.wish(&crossing, field),
            _ => careful.wish(&crossing, field),
        };
        if tape == "loud" && !fired {
            wish.fire = true;
            fired = true;
        }
        crossing.tick(&wish, field);
    }
    for line in &crossing.notices {
        println!("{line}");
    }
    for voice in &crossing.voices {
        println!("[patrol] {}", sense::voice_line(*voice));
    }
    println!("{}", debrief_text(&crossing));
    let facts = crossing.facts();
    let g = grade(&facts);
    println!("grade token     : {}", g.token());
    println!(
        "op               : {:.1} s, extracted {}, alarmed {}, suspicious {}, shots {}, nightjars {}/{}",
        facts.duration_s,
        yesno(facts.extracted),
        yesno(facts.alarmed),
        yesno(facts.suspicious),
        facts.shots,
        facts.nightjars_heard,
        facts.nightjars_found
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use c35::traversal::events::StimulusKind;

    fn facts_clean(duration: f32, extracted: bool) -> OpFacts {
        OpFacts {
            duration_s: duration,
            alarmed: false,
            suspicious: false,
            shots: 0,
            bodies: 0,
            witnesses: 0,
            nightjars_heard: 0,
            nightjars_found: 0,
            closest_soldier_m: 12.0,
            extracted,
            burned: false,
        }
    }

    #[test]
    fn grades_follow_the_file_not_a_score() {
        let mut clean = facts_clean(40.0, true);
        assert_eq!(grade(&clean), Grade::GhostThatNeverWas);
        clean.duration_s = 120.0;
        assert_eq!(grade(&clean), Grade::Ghost);
        clean.duration_s = 40.0;
        clean.suspicious = true;
        assert_eq!(grade(&clean), Grade::Ghost);
        clean.suspicious = false;
        clean.extracted = false;
        assert_eq!(grade(&clean), Grade::Ghost);
        clean.extracted = true;
        clean.burned = true;
        assert_eq!(grade(&clean), Grade::Ghost);
        clean.burned = false;
        clean.alarmed = true;
        assert_eq!(grade(&clean), Grade::Surgeon);
        clean.alarmed = false;
        clean.shots = 1;
        assert_eq!(grade(&clean), Grade::Surgeon);
        clean.shots = 0;
        clean.witnesses = 1;
        assert_eq!(grade(&clean), Grade::Surgeon);
        clean.witnesses = 0;
        clean.bodies = 1;
        assert_eq!(grade(&clean), Grade::Butcher);
        clean.shots = 4;
        clean.alarmed = true;
        assert_eq!(grade(&clean), Grade::Butcher);
    }

    #[test]
    fn the_handler_does_not_file_the_mysteries_or_the_birds() {
        let samples = [
            facts_clean(40.0, true),
            {
                let mut f = facts_clean(40.0, true);
                f.alarmed = true;
                f.shots = 1;
                f.nightjars_found = 2;
                f.nightjars_heard = 3;
                f
            },
            {
                let mut f = facts_clean(50.0, true);
                f.bodies = 2;
                f.witnesses = 1;
                f.burned = true;
                f
            },
        ];
        let banned = [
            "nightjar", "Nightjar", "bird", "churr", "Control", "Wanderer", "Mister", "sword",
            "Type 2", "SYSTEMZ",
        ];
        for facts in &samples {
            let speech = handler_speech(facts, grade(facts));
            for word in banned {
                assert!(!speech.contains(word), "speech mentioned {word}: {speech}");
            }
            assert!(speech.contains("The file will say:"));
        }
        let never = handler_speech(&facts_clean(40.0, true), Grade::GhostThatNeverWas);
        assert!(never.contains("ghost that never was"));
        assert!(never.contains("00:40"));
    }

    #[test]
    fn the_record_is_stable_and_the_margin_is_only_the_count() {
        let mut facts = facts_clean(81.0, true);
        facts.nightjars_heard = 2;
        facts.nightjars_found = 1;
        facts.suspicious = true;
        let g = grade(&facts);
        assert_eq!(g, Grade::Ghost);
        let text = format_record(&facts, g);
        assert!(text.starts_with("DEBRIEF/1\n"));
        assert!(text.contains("grade: ghost\n"));
        assert!(text.contains("nightjars_found: 1\n"));
        assert!(text.contains("suspicious: yes\n"));
        assert_eq!(text, format_record(&facts, g));
        assert_eq!(margin_line(&facts), "Nightjars: 1");
        assert!(!handler_speech(&facts, g).contains("Nightjars"));
    }

    #[test]
    fn birds_are_heard_before_they_are_found_and_silent_by_day() {
        let mut heard = [false; 3];
        let mut found = [false; 3];
        let day = consider_birds(false, 9.2, -36.0, 0.0, true, &mut heard, &mut found);
        assert!(day.is_none());
        assert_eq!(found.iter().filter(|b| **b).count(), 0);
        let far = consider_birds(true, 9.2, -28.0, 1.1, true, &mut heard, &mut found);
        assert_eq!(
            far,
            Some("A churring, low, from somewhere above the street.")
        );
        assert!(!far.unwrap().contains("nightjar"));
        assert_eq!(found.iter().filter(|b| **b).count(), 0);
        let again = consider_birds(true, 9.2, -28.0, 1.1, true, &mut heard, &mut found);
        assert!(
            again.is_none(),
            "a heard bird does not announce itself twice"
        );
        let found_line = consider_birds(true, 9.2, -36.0, 0.2, true, &mut heard, &mut found);
        assert_eq!(
            found_line,
            Some("You find the bird. You leave it where it is.")
        );
        assert_eq!(found.iter().filter(|b| **b).count(), 1);
        // Running through the perch does not find it.
        let mut heard2 = [false; 3];
        let mut found2 = [false; 3];
        let _ = consider_birds(true, 1.4, 13.5, 4.0, true, &mut heard2, &mut found2);
        assert_eq!(found2.iter().filter(|b| **b).count(), 0);
        // Upright and still does not find it either.
        let _ = consider_birds(true, 1.4, 13.5, 0.0, false, &mut heard2, &mut found2);
        assert_eq!(found2.iter().filter(|b| **b).count(), 0);
    }

    #[test]
    fn south_is_forward_when_the_yaw_faces_the_checkpoint() {
        let facing = horizontal_facing(std::f32::consts::PI);
        assert!(facing.z > 0.9, "yaw π should look +Z, got {facing:?}");
        assert!(facing.x.abs() < 0.05);
        let (wish, speed) = wish_from_axes(std::f32::consts::PI, 1.0, 0.0, true, false);
        assert!(wish.z > 0.0);
        assert!((speed - CROUCH_SPEED).abs() < 1e-4);
        let (_, sprint) = wish_from_axes(0.0, 1.0, 0.0, false, true);
        assert!((sprint - SPRINT_SPEED).abs() < 1e-4);
        let (_, still) = wish_from_axes(0.0, 0.0, 0.0, false, true);
        assert_eq!(still, 0.0);
    }

    #[test]
    fn the_warehouse_wall_stops_the_capsule() {
        let blk = crate::block::build_block();
        let mut c = Crossing::begin(
            &blk.field.occluders,
            &blk.field,
            Start {
                night: true,
                rain: 0.0,
                cold: 0.0,
            },
        );
        let mut wish = Wish::still();
        wish.crouch = false;
        wish.speed = WALK_SPEED;
        wish.wish = Vec3::new(1.0, 0.0, 0.0);
        for _ in 0..240 * 3 {
            c.tick(&wish, &blk.field);
        }
        assert!(
            c.trav.body.feet.x < 10.4,
            "walked into the warehouse at x {:.2}",
            c.trav.body.feet.x
        );
        assert!(
            c.trav.body.feet.y < 1.0,
            "left the street, y {:.2}",
            c.trav.body.feet.y
        );
    }

    #[test]
    fn the_hook_bites_the_warehouse() {
        let blk = crate::block::build_block();
        let mut c = Crossing::begin(
            &blk.field.occluders,
            &blk.field,
            Start {
                night: false,
                rain: 0.0,
                cold: 0.0,
            },
        );
        // Stand in the street, look east at the warehouse face.
        c.trav.body.feet = Vec3::new(4.0, 0.0, -10.0);
        c.look_yaw = -std::f32::consts::FRAC_PI_2; // fly yaw −π/2 looks +X
        let aim = horizontal_facing(c.look_yaw);
        assert!(aim.x > 0.9, "expected to face +X, got {aim:?}");
        let mut latched = false;
        for i in 0..90 {
            let wish = Wish {
                wish: Vec3::ZERO,
                speed: 0.0,
                crouch: false,
                jump: false,
                hook_press: i == 0,
                hook_held: true,
                fire: false,
                aim,
            };
            c.tick(&wish, &blk.field);
            if c.trav.grapple.is_latched() {
                latched = true;
                break;
            }
        }
        assert!(latched, "the claw never bit, mode {:?}", c.trav.mode);
        assert!(c
            .trav
            .events
            .events
            .iter()
            .any(|e| e.kind == StimulusKind::Strike));
    }

    #[test]
    fn a_shot_is_on_the_record_and_the_lit_spine_is_a_surgeon() {
        let blk = crate::block::build_block();
        let mut c = Crossing::begin(
            &blk.field.occluders,
            &blk.field,
            Start {
                night: true,
                rain: 0.0,
                cold: 0.0,
            },
        );
        let mut wish = Wish::still();
        wish.fire = true;
        c.tick(&wish, &blk.field);
        assert_eq!(c.facts().shots, 1);
        assert_eq!(grade(&c.facts()), Grade::Surgeon);
        // Held fire does not empty a magazine in one breath.
        for _ in 0..10 {
            c.tick(&wish, &blk.field);
        }
        assert_eq!(c.shots, 1);

        let mut lit = Crossing::begin(
            &blk.field.occluders,
            &blk.field,
            Start {
                night: false,
                rain: 0.0,
                cold: 0.0,
            },
        );
        while !lit.finished() {
            let w = lit_wish(&lit);
            lit.tick(&w, &blk.field);
        }
        assert!(
            lit.extracted,
            "lit tape did not extract, z {:.1}",
            lit.trav.body.feet.z
        );
        assert!(
            lit.alarmed,
            "sprinting the lit spine never alarmed the patrol"
        );
        assert_eq!(grade(&lit.facts()), Grade::Surgeon);
        let speech = handler_speech(&lit.facts(), Grade::Surgeon);
        assert!(speech.contains("He saw you"));
    }

    #[test]
    fn the_careful_tape_extracts_without_a_butchers_file() {
        let blk = crate::block::build_block();
        let mut c = Crossing::begin(
            &blk.field.occluders,
            &blk.field,
            Start {
                night: true,
                rain: 0.0,
                cold: 0.0,
            },
        );
        let mut tape = CarefulTape { waited: 0.0 };
        while !c.finished() {
            let w = tape.wish(&c, &blk.field);
            c.tick(&w, &blk.field);
        }
        assert!(
            c.extracted,
            "careful tape stuck at ({:.1}, {:.1})",
            c.trav.body.feet.x, c.trav.body.feet.z
        );
        let g = grade(&c.facts());
        assert_ne!(g, Grade::Butcher);
        assert_ne!(
            g,
            Grade::Surgeon,
            "the shadow crouch was filed as a surgeon\n{}",
            debrief_text(&c)
        );
        let again = {
            let mut c2 = Crossing::begin(
                &blk.field.occluders,
                &blk.field,
                Start {
                    night: true,
                    rain: 0.0,
                    cold: 0.0,
                },
            );
            let mut tape2 = CarefulTape { waited: 0.0 };
            while !c2.finished() {
                let w = tape2.wish(&c2, &blk.field);
                c2.tick(&w, &blk.field);
            }
            c2.facts().duration_s.to_bits()
        };
        assert_eq!(c.facts().duration_s.to_bits(), again);
    }
}
