//! The host's duty between the block's light and the engine's brain.
//!
//! The street is baked from [`crate::light::LightField`]. The
//! soldier's brain never samples a private number. This module
//! prices a sighting from that field — exposure at the chest, the
//! field's own occluders for the line, and whether he is facing —
//! and hands the product to `c35::perception::pricing::visibility`
//! and [`SoldierBrain`]. The brain decides. The host walks the
//! intent and plays the posture. Nothing here draws an icon.

use c35::glam::Vec3;
use c35::locomotion::{Controller, WALK_SPEED};
use c35::perception::field::{
    Emitter, LightField as EngineField, Occluder as EngineOccluder, Tone,
};
use c35::perception::pricing::{self, Posture};
use c35::perception::soldier::{
    PostureSignal, SoldierBrain, SoldierInput, SoldierOutput, SoldierState, VoiceLine,
};
use c35::perception::sound::propagate;
use c35::traversal::events::{StimulusEvent, StimulusKind};
use c35::traversal::physics::FIXED_DT;
use c35::traversal::surface::{SurfaceKind, SurfaceRecord};

use crate::light::{LightField, FIELD_RADIUS, LAMP_INTENSITY};

/// Forward cone. Below this dot the target is outside the read
/// and visibility is zero. About 75° off the nose still counts;
/// the back of the helmet does not.
pub const FACING_DOT: f32 = 0.25;

/// Eye height on the soldier. The helmet centre sits near 1.69;
/// the eye is the goggle band.
pub const SOLDIER_EYE_Y: f32 = 1.65;

/// Chest height the drives already sample, crouch folding it
/// down. One formula, so the ledger and the brain agree.
pub fn chest_y(crouch: f32) -> f32 {
    1.15 - 0.35 * crouch.clamp(0.0, 1.0)
}

/// A priced sighting. `visibility` is what the brain receives:
/// zero when the line is blocked or he is not facing, otherwise
/// the engine's light × motion × posture product. The other
/// fields are the read that produced it — hosts and tests
/// inspect them; the brain does not.
#[derive(Copy, Clone, Debug)]
#[allow(dead_code)]
pub struct Sight {
    pub exposure: f32,
    pub los_clear: bool,
    pub facing_dot: f32,
    pub visibility: f32,
}

/// Price one sighting from the block's field.
///
/// `soldier_feet` is where he stands. `target_feet` is the
/// operator. Crouch is 0..1, speed is horizontal m/s. The light
/// number is [`LightField::exposure`] — the same function that
/// baked the street — never a second lamp formula.
pub fn price_sight(
    field: &LightField,
    soldier_feet: Vec3,
    soldier_yaw: f32,
    target_feet: Vec3,
    speed: f32,
    crouch: f32,
) -> Sight {
    let chest = Vec3::new(target_feet.x, chest_y(crouch), target_feet.z);
    let eye = soldier_feet + Vec3::new(0.0, SOLDIER_EYE_Y, 0.0);
    let exposure = field.exposure(chest);
    let los_clear = !field.blocks_sight(eye, chest);
    let to = Vec3::new(chest.x - eye.x, 0.0, chest.z - eye.z);
    let dist = to.length();
    let fwd = Vec3::new(soldier_yaw.sin(), 0.0, soldier_yaw.cos());
    let facing_dot = if dist > 1e-4 { fwd.dot(to / dist) } else { 1.0 };
    let light = if los_clear && facing_dot >= FACING_DOT {
        exposure
    } else {
        0.0
    };
    let posture = if crouch >= 0.5 {
        Posture::Crouch
    } else {
        Posture::Upright
    };
    let visibility = if light <= 0.0 {
        0.0
    } else {
        pricing::visibility(light, speed, posture)
    };
    Sight {
        exposure,
        los_clear,
        facing_dot,
        visibility,
    }
}

/// The engine field built from the block's own lamps and
/// occluders. Sound propagation and any later sample read this.
/// Vision's brightness stays on [`price_sight`] (the bake). The
/// positions are the one-field duty: same heads, same boxes.
pub fn engine_field(field: &LightField) -> EngineField {
    let mut out = EngineField::new(0.02);
    for lamp in &field.lamps {
        out.add_emitter(Emitter {
            pos: lamp.head,
            radius: FIELD_RADIUS,
            intensity: LAMP_INTENSITY,
            tone: Tone::Sodium,
        });
    }
    for o in &field.occluders {
        out.add_occluder(EngineOccluder {
            min: o.min,
            max: o.max,
        });
    }
    out
}

/// Aim blend the animator should hold for a posture signal.
/// Halt is not an aim: [`uses_halt`] replaces the frame.
pub fn aim_for(posture: PostureSignal) -> f32 {
    match posture {
        PostureSignal::AtEase => 0.0,
        PostureSignal::Alert | PostureSignal::Yielding => 0.45,
        PostureSignal::Searching => 0.7,
        PostureSignal::Aiming => 1.0,
        PostureSignal::Questioning | PostureSignal::Detaining => 0.0,
    }
}

/// The checkpoint palm. The animator has no halt; the pose
/// vocabulary does.
pub fn uses_halt(posture: PostureSignal) -> bool {
    matches!(
        posture,
        PostureSignal::Questioning | PostureSignal::Detaining
    )
}

/// What he says. The line is the sound. It is not a state icon.
pub fn voice_line(voice: VoiceLine) -> &'static str {
    match voice {
        VoiceLine::Halt => "Halt.",
        VoiceLine::Papers => "Papers.",
        VoiceLine::StepAside => "Step aside.",
        VoiceLine::SearchCall => "Search the street.",
        VoiceLine::ContactCall => "Contact — on the block.",
        VoiceLine::StandDown => "Stand down.",
        VoiceLine::YieldCall => "Clear the arc.",
    }
}

fn turn_toward(yaw: f32, want: f32, max_step: f32) -> f32 {
    let tau = std::f32::consts::TAU;
    let mut d = (want - yaw).rem_euclid(tau);
    if d > std::f32::consts::PI {
        d -= tau;
    }
    yaw + d.clamp(-max_step, max_step)
}

/// The patrol route, taken from the host's own loop so the
/// brain walks the rectangle the drives already use.
pub fn block_route() -> Vec<Vec3> {
    let start = Vec3::new(-8.6, 0.0, -40.0);
    let mut v = vec![start];
    for leg in crate::patrol_legs() {
        v.push(Vec3::new(leg.target.x, 0.0, leg.target.y));
    }
    if let Some(last) = v.last().copied() {
        if (last.x - start.x).abs() < 0.01 && (last.z - start.z).abs() < 0.01 {
            v.pop();
        }
    }
    v
}

/// The conscript on the street: the engine's brain, plus the
/// feet the host is responsible for walking.
pub struct Conscript {
    pub brain: SoldierBrain,
    pub pos: Vec3,
    pub yaw: f32,
    pub speed: f32,
}

impl Conscript {
    pub fn on_route(route: Vec<Vec3>) -> Conscript {
        let pos = route.first().copied().unwrap_or(Vec3::ZERO);
        let yaw = route
            .get(1)
            .map(|p| (p.x - pos.x).atan2(p.z - pos.z))
            .unwrap_or(0.0);
        let mut brain = SoldierBrain::new(1, pos, route);
        if brain.route.len() > 1 {
            brain.route_i = 1;
        }
        Conscript {
            brain,
            pos,
            yaw,
            speed: 0.0,
        }
    }

    /// One fixed step. `dt` is [`FIXED_DT`]. The brain ticks
    /// once; the feet follow `move_to` at a walk, and a patrol
    /// waypoint reached advances the route.
    pub fn tick(&mut self, input: SoldierInput, dt: f32) -> SoldierOutput {
        let out = self.brain.tick(input);
        self.brain.pos = self.pos;
        if let Some(goal) = out.move_to {
            let to = Vec3::new(goal.x - self.pos.x, 0.0, goal.z - self.pos.z);
            let dist = to.length();
            if dist > 0.55 {
                let dir = to / dist;
                let want = dir.x.atan2(dir.z);
                self.yaw = turn_toward(self.yaw, want, 1.75 * dt);
                let fwd = Vec3::new(self.yaw.sin(), 0.0, self.yaw.cos());
                if fwd.dot(dir) > 0.5 {
                    self.speed = WALK_SPEED;
                    self.pos += fwd * self.speed * dt;
                } else {
                    self.speed = 0.0;
                }
            } else {
                self.speed = 0.0;
                if self.brain.state == SoldierState::Patrol {
                    if let Some(wp) = self.brain.route.get(self.brain.route_i).copied() {
                        let at = (wp.x - self.pos.x).hypot(wp.z - self.pos.z);
                        if at <= 0.55 {
                            let n = self.brain.route.len();
                            if n > 0 {
                                self.brain.route_i = (self.brain.route_i + 1) % n;
                            }
                        }
                    }
                }
            }
        } else {
            self.speed = 0.0;
        }
        self.pos.y = 0.0;
        out
    }
}

/// How a scripted crossing sat in the conscript's mind.
#[derive(Clone, Debug)]
pub struct MindReport {
    pub ticks: u64,
    pub seconds: f32,
    pub patrol_s: f32,
    pub suspicious_s: f32,
    pub search_s: f32,
    pub alarmed_s: f32,
    pub peak_attention: f32,
    pub voices: Vec<VoiceLine>,
}

impl MindReport {
    pub fn line(&self, name: &str) -> String {
        format!(
            "{name}: {seconds:.1} s, patrol {patrol:.1} suspicious {sus:.1} search {search:.1} alarmed {alarm:.1}, peak attention {peak:.2}, voices {voices}",
            seconds = self.seconds,
            patrol = self.patrol_s,
            sus = self.suspicious_s,
            search = self.search_s,
            alarm = self.alarmed_s,
            peak = self.peak_attention,
            voices = self.voices.len(),
        )
    }
}

/// Walk one of the host's scripted routes (`open` / `shadow`)
/// against the living conscript. Fixed 240 Hz, rain as given.
/// Footfalls are priced by the host: the traversal sim emits
/// them on a wall-run, not on flat gait, so a ground step is
/// written here on a cadence and propagated through the engine.
pub fn simulate_route(field: &LightField, which: &str, rain: f32) -> MindReport {
    let (start, legs) = crate::drive_route(which);
    let mut ctrl = Controller::new(start, legs);
    let mut soldier = Conscript::on_route(block_route());
    let ears = engine_field(field);
    let mut report = MindReport {
        ticks: 0,
        seconds: 0.0,
        patrol_s: 0.0,
        suspicious_s: 0.0,
        search_s: 0.0,
        alarmed_s: 0.0,
        peak_attention: 0.0,
        voices: Vec::new(),
    };
    let mut step_acc = 0.0f32;
    // Settle cap matches the drive: once the operator arrives,
    // give the conscript a short tail of ticks and stop.
    let mut settle = 0u32;
    for _ in 0..240 * 200 {
        let input = ctrl.update(FIXED_DT);
        let agent = *ctrl.agent();
        let feet = Vec3::new(agent.pos.x, 0.0, agent.pos.y);
        let sight = price_sight(
            field,
            soldier.pos,
            soldier.yaw,
            feet,
            agent.speed,
            input.crouch,
        );
        let mut heard = Vec::new();
        if agent.speed > 0.3 {
            step_acc += FIXED_DT;
        }
        if step_acc >= 0.42 {
            step_acc = 0.0;
            let surface = SurfaceRecord::dry(SurfaceKind::Asphalt);
            let quiet = if input.crouch >= 0.5 { 0.45 } else { 1.0 };
            let event = StimulusEvent {
                tick: report.ticks,
                kind: StimulusKind::Footstep,
                pos: feet,
                surface,
                magnitude: surface.footstep_gain(agent.speed) * quiet,
            };
            let ear = soldier.pos + Vec3::new(0.0, SOLDIER_EYE_Y, 0.0);
            heard = propagate(std::slice::from_ref(&event), ear, &ears);
        }
        let posture = if input.crouch >= 0.5 {
            Posture::Crouch
        } else {
            Posture::Upright
        };
        let seen = sight.visibility > 0.0;
        let out = soldier.tick(
            SoldierInput {
                visibility: sight.visibility,
                target_pos: if seen {
                    Some(Vec3::new(feet.x, chest_y(input.crouch), feet.z))
                } else {
                    None
                },
                target_posture: Some(posture),
                heard,
                rain,
                ..SoldierInput::default()
            },
            FIXED_DT,
        );
        if let Some(v) = out.voice {
            report.voices.push(v);
        }
        report.peak_attention = report.peak_attention.max(soldier.brain.attention.level);
        let dt = FIXED_DT;
        match out.state {
            SoldierState::Patrol | SoldierState::Checkpoint => report.patrol_s += dt,
            SoldierState::Suspicious => report.suspicious_s += dt,
            SoldierState::Search => report.search_s += dt,
            SoldierState::Alarmed | SoldierState::Combat | SoldierState::BodyFound => {
                report.alarmed_s += dt
            }
        }
        report.ticks += 1;
        report.seconds += dt;
        if agent.done && agent.speed < 0.05 {
            settle += 1;
            if settle > 240 {
                break;
            }
        }
    }
    report
}

pub fn print_report(field: &LightField, rain: f32) {
    let eng = engine_field(field);
    println!(
        "one field        : {} lamps shared, {} occluders shared, facing dot {:.2}, step {:.4} s",
        eng.emitters.len(),
        eng.occluders.len(),
        FACING_DOT,
        FIXED_DT
    );
    for which in ["shadow", "open"] {
        let report = simulate_route(field, which, rain);
        println!("{}", report.line(which));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use c35::perception::pricing::Attention;

    fn lone_field() -> LightField {
        // A lamp over the origin, no boxes. Exposure at the
        // chest under it is a pool; a point far east is dark.
        LightField {
            lamps: vec![crate::light::Lamp {
                head: Vec3::new(0.0, 6.2, 0.0),
            }],
            occluders: vec![],
        }
    }

    #[test]
    fn shadow_crouch_is_under_notice_and_a_run_in_the_pool_is_not() {
        let field = lone_field();
        // Soldier at the origin, facing +Z. Target 4 m south,
        // in the pool, crouched and still.
        let still = price_sight(&field, Vec3::ZERO, 0.0, Vec3::new(0.0, 0.0, 4.0), 0.0, 1.0);
        assert!(still.los_clear);
        assert!(still.facing_dot > FACING_DOT);
        assert!(
            still.exposure > 0.2,
            "the pool should light the chest, got {}",
            still.exposure
        );
        assert!(still.visibility < Attention::soldier().notice);
        let running = price_sight(&field, Vec3::ZERO, 0.0, Vec3::new(0.0, 0.0, 4.0), 5.0, 0.0);
        assert!(running.visibility > Attention::soldier().notice);
        assert!(running.visibility > still.visibility);
    }

    #[test]
    fn the_back_of_the_helmet_does_not_see() {
        let field = lone_field();
        // Facing +Z, target to the north (−Z): behind him.
        let back = price_sight(&field, Vec3::ZERO, 0.0, Vec3::new(0.0, 0.0, -3.0), 5.0, 0.0);
        assert!(back.facing_dot < 0.0);
        assert_eq!(back.visibility, 0.0);
    }

    #[test]
    fn a_wall_closes_the_line() {
        let mut field = lone_field();
        field.occluders.push(crate::light::Occluder::new(
            Vec3::new(-1.0, 0.0, 1.5),
            Vec3::new(1.0, 4.0, 2.2),
        ));
        let blocked = price_sight(&field, Vec3::ZERO, 0.0, Vec3::new(0.0, 0.0, 6.0), 5.0, 0.0);
        assert!(!blocked.los_clear);
        assert_eq!(blocked.visibility, 0.0);
    }

    #[test]
    fn rain_delays_the_same_sighting() {
        fn ticks_until_suspicious(rain: f32) -> u64 {
            let mut brain = SoldierBrain::new(7, Vec3::ZERO, vec![]);
            for t in 1..8000 {
                let out = brain.tick(SoldierInput {
                    visibility: 0.55,
                    target_pos: Some(Vec3::new(0.0, 1.1, 6.0)),
                    target_posture: Some(Posture::Upright),
                    rain,
                    ..SoldierInput::default()
                });
                if out.state != SoldierState::Patrol {
                    return t;
                }
            }
            u64::MAX
        }
        let dry = ticks_until_suspicious(0.0);
        let wet = ticks_until_suspicious(1.0);
        assert!(dry < 8000, "a dry conscript never noticed");
        assert!(wet > dry, "rain should degrade him, dry {dry} wet {wet}");
    }

    #[test]
    fn the_engine_field_uses_the_blocks_lamps_and_boxes() {
        let blk = crate::block::build_block();
        let eng = engine_field(&blk.field);
        assert_eq!(eng.emitters.len(), blk.field.lamps.len());
        for (e, lamp) in eng.emitters.iter().zip(blk.field.lamps.iter()) {
            assert_eq!(e.pos, lamp.head);
            assert_eq!(e.tone, Tone::Sodium);
            assert_eq!(e.radius, FIELD_RADIUS);
        }
        assert_eq!(eng.occluders.len(), blk.field.occluders.len());
        for (a, b) in eng.occluders.iter().zip(blk.field.occluders.iter()) {
            assert_eq!(a.min, b.min);
            assert_eq!(a.max, b.max);
        }
    }

    #[test]
    fn open_route_sits_in_his_mind_longer_than_the_shadow() {
        let blk = crate::block::build_block();
        let shadow = simulate_route(&blk.field, "shadow", 0.0);
        let open = simulate_route(&blk.field, "open", 0.0);
        let shadow_heat = shadow.suspicious_s + shadow.search_s + shadow.alarmed_s;
        let open_heat = open.suspicious_s + open.search_s + open.alarmed_s;
        assert!(
            open_heat > shadow_heat,
            "open {open_heat:.2} should exceed shadow {shadow_heat:.2}\n{}\n{}",
            shadow.line("shadow"),
            open.line("open"),
        );
        // The shadow crouch is the route that stays out of the
        // sheet. A graze may suspicious him; it must not alarm.
        assert!(
            shadow.alarmed_s < 0.05,
            "shadow route alarmed for {:.2} s\n{}",
            shadow.alarmed_s,
            shadow.line("shadow"),
        );
        assert!(
            open.alarmed_s > 0.0,
            "the lit spine was never alarmed\n{}",
            open.line("open")
        );
    }

    #[test]
    fn the_same_route_reproduces() {
        let blk = crate::block::build_block();
        let a = simulate_route(&blk.field, "shadow", 0.0);
        let b = simulate_route(&blk.field, "shadow", 0.0);
        assert_eq!(a.ticks, b.ticks);
        assert_eq!(a.alarmed_s.to_bits(), b.alarmed_s.to_bits());
        assert_eq!(a.suspicious_s.to_bits(), b.suspicious_s.to_bits());
        assert_eq!(a.voices, b.voices);
    }

    #[test]
    fn posture_map_is_the_action_sheet_not_an_icon() {
        assert_eq!(aim_for(PostureSignal::AtEase), 0.0);
        assert!(aim_for(PostureSignal::Aiming) > aim_for(PostureSignal::Alert));
        assert!(uses_halt(PostureSignal::Questioning));
        assert!(!uses_halt(PostureSignal::Aiming));
        assert_eq!(
            voice_line(VoiceLine::ContactCall),
            "Contact — on the block."
        );
    }
}
