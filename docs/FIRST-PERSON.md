# DeBrief — FIRST PERSON

Design only. No tree edits, no repo writes. Written 2026-10-30.
DeBrief is a DUAL-MODE game: first person and third person on one
movement core, free toggle, identical stealth math — and first
person is the native, tuning mode, the graded truth the game is
balanced from (DESIGN.md, "Standing rulings — 2026-10-30"). This
note specifies the first-person rig. It was first written under a
first-person-only decision made the same day; that decision was
superseded by the dual-mode ruling, and this note now serves the
ruling, not the earlier call (§5.3 records the wording history).

Standing law this note serves, not revises: the Beauty Law, the PS1.5
identity, zero image assets, determinism, the cost ledger, the
one-field light law, the no-label law, the dark visor law, the
grapple law (EVERYTHING grapples), the both-hands law, and the
outside-in build order (engine, city, weathers, lighting, and so on).

## 1. The camera — the operator's eye

### 1.1 Eye height and base field of view

- **Eye height is the operator rig's eye plane.** No invented
  number: the build crew reports the eye height in metres from the
  operator model as built, and the camera sits exactly there. The
  eye must match the body the game simulates — a capsule whose
  height the eye plane is derived from, never a free-floating
  camera height tuned by feel alone. If the model changes height,
  the eye moves with it, and the change is recorded in the ledger.
- **Base FOV is a tuned constant with a proof, not a picked
  number.** The build crew sets it by the same fixed-camera proof
  discipline as the beauty pass: eye-view frames at the checkpoint
  (~5 m), the patrol (~20 m), and the street, judged against
  golden #9 and #11 for spatial read. The FOV must let the player
  read a doorway, a checkpoint queue, and a lamp pool's edge
  without turning — stealth lives in peripheral information.
- **FOV behavior, as rules:**
  - **Aim:** FOV narrows slightly and smoothly (eased, never a
    snap) when the weapon is raised — the visor's focus, not a
    scope. It returns on lower with the same ease.
  - **Sprint:** FOV widens slightly and smoothly with speed, and
    only while sprinting. It must never widen during stealth
    movement — crouch-walk and stillness keep the base FOV, so the
    frame does not lie about how exposed the player is.
  - **Grapple swing:** FOV follows swing speed with the same
    easing as sprint, capped at the sprint widening. No extra kick
    on latch or release — both are the motion-sickness danger
    points (§4.4).
  - **Wall-run:** FOV holds base. The wall-run already moves the
    frame violently enough; widening it further would sell speed
    the player cannot use.

### 1.2 Head-bob — little, and never when stealth matters

- Bob is **subtle, speed-scaled, and positional, not angular**:
  the eye translates a few centimetres with the gait cycle; it
  does not roll or pitch with it. Roll and pitch are reserved for
  real events (landing dip, swing lean — §4.4).
- **Bob is OFF, by law, in every state where stillness is the
  stealth:** crouch-still, aim, lean, wall-hold, and any moment
  the player has not moved in the last beat. A bobbing frame while
  hiding teaches the player that hiding moves — the game must
  never teach that.
- **Landing is a dip, not a bounce:** one eased downward
  translation on landing, scaled by fall speed, recovering over
  less than a second. No double-bounce, no camera shake on
  landing — shake is reserved for damage and the mecha's tread,
  where it is information, not decoration.
- **Idle breath:** when standing still, the eye carries the
  faintest breathing translation (the animator's idle, seen from
  inside). It must be small enough that a player holding an aim
  never notices it, and it stops entirely when aiming.

### 1.3 What the player sees of the operator

In first-person mode the operator is not seen whole in play.
What is seen is the viewmodel: hands, sleeves, weapon, and
device — and it must be right, because in the native mode it is
the most-seen operator in the game. (Third-person mode shows the
full figure in play, on the third-person rig — §5.1.)

- **Hands and sleeves, per configuration:**
  - **Street (coat):** waxed-canvas coat sleeves, cuff and seam
    lines painted at viewmodel texel density, leather gloves,
    watch face-inward on the LEFT wrist — visible when the left
    hand rises (reload, grapple aim, mantle). The watch is a
    configuration tell the player can check on himself.
  - **Assault (black set):** black plate-carrier sleeves, black
    gloves, headset boom just at the frame edge when turning
    hard — never intruding on the sight picture. No patch, no
    label, no glow, per the standing laws. The visor rim is
    visible at the frame edge as a dark frame, never as light.
- **Weapon:** the USP in the right hand at low-ready, raised on
  aim; the slung long gun never blocks the viewmodel (it rides
  the back, out of frame, except as a shoulder-strap edge when
  looking down). Reloads, the suppressed SMG, and the other rack
  weapons each get their own viewmodel hands — a weapon swap
  must never show the wrong hands.
- **Grapple device:** the two-handed drum-fed launcher (§4.1)
  rises into frame on E press, both hands on it, and leaves frame
  on release. While latched, the hands hold the launcher and the
  line runs from it — the player sees the tool doing the work,
  not a floating rope.
- **Body awareness — looking down:**
  - Looking down shows the operator's legs, boots, and the lower
    coat skirt (street) or carrier and thigh rigs (assault). The
    legs must move with the gait — a static lower body under a
    moving eye breaks immersion instantly.
  - The torso edge is visible at the frame bottom when looking
    steeply down, and the coat skirt swings with walk (the
    animator's existing skirt behavior, now seen from inside).
  - **Body awareness must never break immersion:** no
    headless-body gaps at any pitch, no legs that do not match
    the speed the eye is travelling at, no viewmodel that
    detaches from the world during a swing or a wall-run. The
    viewmodel is parented to the camera with lag, not glued to
    it — it must feel held, not pasted.

## 2. Where the full operator figure still lives

First person hides the figure in its own mode; it does not
delete him. Third person shows him whole in play (§5.1), and the
identity work still lives on the presentation surfaces — each
surface has a governing reference:

- **The debrief room:** the operator sits across the desk from
  the handler, and the camera (placed, deliberate — the MGS1
  lesson, kept for interiors) shows him whole. Governing
  reference: the STREET coat (#31/#32) when the op was run in
  street configuration, the ASSAULT set (#33–#35) when it was run
  in assault — the room shows the man as he worked, not as a
  poster. His face is visible here, and only here in play: the
  debrief-room scene is where the bare weathered face (#31) or
  the balaclava (#40 family) is read at conversation distance.
- **The menu screen:** the operator stands in the banner pose.
  Governing reference: the MAIN IDENTITY, golden #40 (the
  detailed portrait finish — full fabric/leather/gunmetal detail,
  NO wire mesh), patch included — the single-eagle patch belongs
  to the identity portrait by ruling. The standing model-law
  corrections (visor dark, never glowing; no labels) apply to the
  IN-GAME field figure, not to the identity portrait. The menu
  portrait is the identity; the field figure is the configuration.
- **The compendium portraits:** each person entry carries its
  golden reference as its portrait — the operator's entry carries
  #40, the handler's #18/#19, the Wanderer's #27/#28, and so on.
  Portraits are the canon images, not re-renders; the compendium
  is an archive, and archives keep photographs.
- **Mirrors and reflections:** any mirror, window, or wet-street
  reflection that shows the operator shows the CURRENT
  configuration, full figure, at whatever fidelity the reflection
  system affords — and a reflection must never show the wrong
  configuration. Reflections are rare (Greyport's glass is
  mostly dark), which is what makes them land: the player sees
  himself only when the world offers a surface, never on demand.
- **Death and the file:** the operator's death is still a system,
  not a cutscene — but in first person the death is the eye's
  death (fall, black, the record closing), and the successor
  agent's first sight of the predecessor's body in the street is
  the third-person figure returning as evidence. The full figure
  is never gone from the game; it is gone from the player's own
  view while he lives.

## 3. Stealth readability in first person

Third person shows body-vs-cover directly: the player sees his
own silhouette against the lamp. First person must replace that
read, not remove it. Every mechanism below is the first-person
mode's replacement, and ties to the existing
one-field light law and the existing perception systems — no new
detection system is invented, and no HUD marker ever tells the
player "you are seen" as a game icon. The world tells him, the
way it always did.

- **Lean:** the player leans around corners and cover. Lean is a
  mechanic with a full specification: the eye translates sideways
  (never rolls the world), the lean exposes exactly the eye and
  no more, and leaning out of a lamp pool into shadow is priced
  by the light field at the eye's position — the same field the
  perceivers read. Lean while aiming keeps the weapon aligned
  with the eye. Lean bind: Q/E is OUT — E is grapple, fixed
  (Chase 2026-10-30); the lean pair is Chase's call (see open
  questions — the mechanic is not optional).
- **Cover feedback — the visor's edge language:** the visor
  frame itself carries the cover read. When the eye is in shadow
  and still, the frame sits quiet and dark; when the eye crosses
  into lamplight, the frame edge catches the light — a thin
  sodium rim on the visor edge, driven by the light field at the
  eye, not by a scripted state. The player learns to read his
  own light level from the frame, the way a third-person player
  read it from his silhouette. This is the one-field law applied
  to the viewmodel: if the frame looks lit, the perceivers' field
  says lit.
- **The visor IS the HUD (native, diegetic):** damage numbers,
  ammunition, the op clock, the compass strip, the body
  pictogram, and the SIG bars from the golden #38 HUD frame all
  live on the visor glass — analog instruments, not an overlay
  pasted on the world. Component tags on a mecha (SENSOR,
  LOCOMOTION, CANNON, POWER-COOLANT) appear through the visor's
  sense view at the ranges the sense mode already grants. The
  visor never shows an enemy's state as an icon — posture and
  sound still carry that, per the perception law.
- **Shadow awareness:** the player must be able to tell, without
  turning, whether he stands in light or shadow. Three channels,
  all from the existing fields: (a) the visor-frame rim above;
  (b) the viewmodel's own lighting — hands and weapon lit by the
  same lamp field as the world, so lit hands mean a lit man;
  (c) audio — the ambient bed changes subtly between lamp pools
  and shadow (insects, distant water, the city's hum fall away
  under a lamp's buzz). No channel is a new system; all three
  read fields that already exist.
- **Audio cues, priced as stimuli:** footsteps (surface × speed),
  the hook strike, wall-run footfalls, and the mecha's servo
  whine and tread are all stimuli in the shared fields, and the
  player hears them at the ranges the fields price — hearing a
  patrol's boots before seeing him is the first-person stealth
  sense, and it already exists in the design. The Hound's scent
  channel is unchanged (the visor never shows the player's own
  scent picture — the debrief still grades it after the fact).
- **What is LOST and how it is paid for:** the third-person
  player saw his back and his coat against cover; the
  first-person player cannot. The payment is the combination
  above — visor rim + lit hands + audio + lean — and the proof
  is the same crossing discipline as Slice 1: the open crossing
  and the shadow crossing re-proven from the eye, with the
  player's light read at the eye logged against the patrol's
  facing. If the eye's read and the patrol's read disagree, the
  stealth is broken, not the proof.

## 4. Traversal feel in first person

### 4.0 The fixed control

**E = grapple.** Fixed, in this note and as a requirement the
traversal build must carry. E press fires the hook, hold keeps
the rope attached (pull/swing), E release detaches. No other key
fires the hook, and no mode remaps E away from it in play. (The
windowed host's free-fly debug camera currently uses E as
"down" — that is a dev-camera binding, not the game, and it
remaps when play mode owns the key. Play mode is the game; the
debug camera yields.)

### 4.1 The grapple — MATCH NEXUIZ

The grappling hook must MATCH NEXUIZ. Grounded below in the
house's own records: the thisissparta2 spec
(`~/workspace/thisissparta2/DESIGN.md` §4) and the Nexuiz 2.5.2
QuakeC source in the house tree
(`~/workspace/nexuiz-research/gamesrc/qcsrc/server/g_hook.qc`,
Wazat's grappling hook, and `g_hook.qh`). The thisissparta2
config (`configs/thisissparta2.cfg`) sets exactly one hook line:
`g_grappling_hook 1` — "the offhand grappling hook", stock
`server.cfg:50`, implemented in `g_hook.qc`. Everything else
below is that code's behavior, translated into DeBrief terms.
Where the house records do not verify a number, this note says
so and lists it as an open question — it does not invent one.

**The input contract (Nexuiz, verbatim in behavior):**
- **E press (edge) fires the hook.** Nexuiz: `+hook` is bound
  to `+button6` (`BUTTON_HOOK`); `GrapplingHookFrame` fires on
  the press edge only when no hook is live and the button is not
  still held from a previous fire (`HOOK_WAITING_FOR_RELEASE`
  guard — holding the key through a detach does not instantly
  refire). DeBrief: E press fires, with the same
  waiting-for-release guard — releasing E after a detach, then
  pressing again, is required to refire.
- **E hold keeps the rope attached.** Nexuiz: while `BUTTON_HOOK`
  is held, the hook entity lives and `GrapplingHookThink` runs
  every think; releasing the button sets `HOOK_REMOVING` and the
  hook entity is removed. DeBrief: hold = attached (pull/swing);
  release = detached, momentum preserved, never zeroed.
- **E release detaches — always, instantly, without fail.**
  Release during swing, during pull, during climb: the rope is
  gone on the release tick and the capsule keeps its velocity
  into fall, jump, or wall-run. A release that lags or queues is
  a failed build.

**Latch rules (Nexuiz, from `GrapplingHookTouch`):**
- The hook is a projectile (Nexuiz: `hook.md3`, a 6-unit box,
  `MOVETYPE_FLY`, velocity along the aim) that flies until it
  touches. On touch it stops dead: impact particles, the impact
  sound (`hook_impact.wav`), velocity zeroed, movetype NONE,
  state latched. DeBrief: the claw flies from the launcher on
  the aim line and latches on touch with the same stop-dead
  character — impact sound at the strike point, a visible bite,
  no slide, no bounce.
- **Sky never latches.** Nexuiz checks the struck surface for
  `Q3SURFACEFLAG_NOIMPACT` (sky brushes) and REMOVES the hook
  instead of latching — the code comments name this "the
  sky-grapple bug" prevention. DeBrief: sky, and any surface
  the surface record marks as unbiteable, removes the hook
  without latching. The failure is visible (the claw falls) and
  priced (the strike's noise still happened).
- **Movers latch and are followed.** Nexuiz: if the latched
  entity's movetype is not NONE, the hook follows it
  (`MOVETYPE_FOLLOW`, tracking origin and angles) and detaches
  if the mover dies or changes class. DeBrief: cranes, moving
  platforms, and any mover the rope can bite follow the same
  rule — the rope's far end rides the mover, and the swing
  pivot moves with it.
- **The hook can be destroyed.** Nexuiz: the hook entity has
  health (`g_balance_grapplehook_health`) and takes damage; at
  zero health it detaches. DeBrief: enemy fire that severs the
  line detaches the rope — a rare, priced failure, not a routine
  one.
- **DeBrief's latch surface law is unchanged:** EVERYTHING
  grapples — the constraint is physical, never permission. The
  Nexuiz latch rules above (sky, unbiteable surfaces, movers)
  are physical properties read from the surface record, never a
  per-location permission table. The traversal brief's ban on
  whitelists stands: no anchor table, no blessed points.

**Pull and swing physics character (Nexuiz, from
`GrapplingHookThink`):**
- Nexuiz has two characters, selected by `g_grappling_hook_tarzan`:
  - **Tarzan (pull-and-swing):** the rope SHORTENS at the pull
    speed toward a minimum length, while a rubber-band force
    pulls the player toward the hook point in proportion to how
    far the rope is stretched past its current length, damped by
    a rope air-friction term. Overstretch (rope stretched past
    its length plus the stretch allowance) clamps the length and
    applies a stronger overstretch force. The result is the
    Nexuiz feel: a swing you can reel in, with give in the rope
    — never a rigid rod, never a free fall.
  - **Non-tarzan (fly-to-point):** the player's movetype becomes
    FLY and velocity is set directly toward the hook point at
    the pull speed, scaled down inside 200 units and cut to zero
    inside 50 — a winch, not a swing.
- **DeBrief matches the TARZAN character.** The reference values
  in the house source's comments are: pull speed 2000, rubber
  force 2000, rubber overstretch force 1000, minimum length 100,
  stretch allowance 400, rope air friction 0.2 (units are
  Nexuiz's; the traversal build re-derives them in C35 units
  from the course, and reports the conversion — it does not
  copy the numbers blind). The thisissparta2 config touches
  NONE of these — it runs stock — so the shipped
  `balance25.cfg` exact values are NOT verified in the house
  tree and are an open question (§6), not a number this note
  asserts.
- **Crouch while hooked stops the reel, not the swing.**
  Nexuiz: holding crouch clears `HOOK_PULLING` — the rope stops
  shortening, but the rubber-band swing continues. DeBrief keeps
  this exactly: crouch on the rope = stop reeling, keep swinging.
  It costs nothing in stealth vocabulary (crouch in the air was
  never a posture) and buys rope-length control without a second
  key.
- **The swing is the traversal verb, not a taxi.** The player
  steers the swing with movement input (the pendulum's plane
  follows the aim and the strafe), and the swing's energy is
  conserved except for the priced decay — no swing may gain
  energy, the same law the traversal brief already states for
  wall-runs. Exit speed never exceeds entry energy plus the
  reel-in the player paid for with hold time.

**Rope/beam rendering (Nexuiz, from `GrapplingHookThink`):**
- Nexuiz draws the rope every think as a beam (`te_beam`) from
  the player's shot origin to the hook point, over the flying
  hook model. DeBrief renders the same read in PS1.5 terms: a
  braided line (golden #16's braided line, not a laser) from the
  launcher to the claw, taut and straight while the rope is
  taut, with the traversal brief's drape behavior at edges — a
  line over a parapet shortens its free length at the contact
  point and the pendulum pivot MOVES to the edge. The beam's
  straight-taut read and the brief's drape are one rule: taut =
  straight, caught = draped at the contact. The line is visible
  in lamplight and readable against the sky — a rope the player
  cannot see is a rope he cannot steer.
- The claw itself is visible in flight and latched: the
  three-prong claw (golden #16) at the line's far end, not an
  abstract point.

**Interaction with wall-run and mantle (DeBrief's verbs —
Nexuiz has neither, so this is DeBrief law built on the Nexuiz
rope):**
- **E mid-wall-run fires the hook and ends the run.** Both
  hands commit to the launcher (the both-hands law — see below);
  the wall-run's momentum carries into the swing's first arc,
  never zeroed. The run's decay already spent is not refunded.
- **Swing → wall-run (#38):** releasing E at the swing's arc
  toward a wall, with wall-run entry speed, enters the wall-run
  on that wall — the founding transition frame, now seen from
  the eye: the wall fills the frame, the wall-side hand trails
  for balance (visible in the viewmodel's lower frame edge),
  and the coat's motion is felt as frame-edge cloth, not seen
  as a figure.
- **Swing → mantle:** releasing E near a ledge within mantle
  reach, with forward intent, ends in the mantle (brief §1.3:
  ledge within reach at capsule top, climb to stand). The
  mantle from a swing keeps the swing's forward momentum into
  the climb — a fast, committed exit, priced by its exposure
  (a mantling man is a silhouette on a roofline, and the
  Weathervane's ground is exactly that roofline).
- **Wall-run → wall-run (corner transfer):** where two surfaces
  meet at a corner the capsule can round, the run transfers
  without a hook — the brief's rule, unchanged in FP, with the
  camera yawing through the corner smoothly (no snap).
- **The both-hands law, reconciled with Nexuiz:** Nexuiz's hook
  is OFFHAND — the player can shoot while hooked. DeBrief's
  launcher is a TWO-HANDED drum-fed tool (golden #16), and the
  both-hands law stands: while the launcher is committed (from
  the strike until release), the weapon layer is closed — no
  firing, no aim pose — until the swing exit of #38 (line
  released, one hand free, THEN the pistol may speak). This is
  the one deliberate deviation from Nexuiz's offhand freedom,
  and it is DeBrief doctrine, not an oversight: the grapple is
  a commitment, and the pistol speaks after it, not during it.
  The traversal build must enforce it as graph state, per the
  brief — never as an input suggestion.

### 4.2 Wall-running in first person

- **The eye's wall-run:** the camera tracks the wall at a fixed
  offset, yawed to face travel direction with a slight lean INTO
  the wall (the lean is a camera roll toward the wall, small and
  constant — the wall-side arm trailing for balance is visible
  at the frame's lower edge). The wall fills one side of the
  frame; the street falls away on the other. Speed reads through
  the FOV rule (§1.1) and the passing wall texture, never
  through shake.
- **Entry and budget are unchanged from the brief:** entry from
  the run/sprint band, moving roughly parallel to a wall within
  reach; the run decays against gravity; wall angle and surface
  wetness price it (dry brick holds, wet concrete shortens the
  run and slickens the exit). The eye's job is to sell the
  budget: as the run decays, the camera's forward pitch drops
  with the arc — the player feels the fall coming before it
  arrives, and the exit (jump, grapple, ground) is chosen, not
  suffered.
- **Footfalls are the wall-run's sound:** each wall-run footfall
  emits a footstep stimulus on that surface (surface × speed),
  priced into the shared fields — the brief's rule, unchanged.
  In first person the footfalls are also the player's own rhythm
  cue: the run's remaining budget is audible in the step rate.

### 4.3 Mantle in first person

- The mantle is a short, committed camera move: the eye rises
  over the ledge on a fixed arc, hands visible reaching the
  ledge (viewmodel mantle hands — the one moment the player
  sees both hands on the world), then the eye settles to stand.
  The arc is eased at both ends and never snaps. During the
  mantle the player cannot fire — both hands are on the ledge —
  and the mantle's exposure is priced like any other: a man
  cresting a roofline is the Weathervane's easiest read.

### 4.4 Motion sickness — kept out by construction

Fast stealth in first person must not punish the player's
stomach. Rules, all enforced, none optional:

- **No camera snap, anywhere:** latch, release, wall-run entry,
  wall transfer, mantle, landing — every transition is eased
  over frames, never a cut. A snap is a defect, not a feel.
- **Roll is capped and rare:** the only rolls are the wall-run
  lean (small, constant) and the swing's bank (proportional to
  lateral swing velocity, capped). No roll on footfalls, no roll
  on landing, no roll on damage.
- **The horizon is sacred:** during swings the camera keeps the
  horizon level except for the capped swing bank — the world
  must not tilt with the pendulum, or the player will be ill
  before the second swing.
- **No head-bob during traversal:** bob is a foot-gait effect
  (§1.2) and is OFF during swings, wall-runs, and mantles. The
  traversal verbs have their own camera language; bob on top of
  it is noise that nauseates.
- **FOV changes are eased and capped** (§1.1) — a fast FOV punch
  on latch or release is the single most nauseating thing a
  grapple game can do, and it is banned here.
- **A comfort fallback is owed, not designed here:** if
  playtesting shows the swing still sickens, the build crew
  reports it with the exact transition that does it — the fix
  is a named transition's easing, never a global "reduce
  motion" slider invented in this note. (See open questions.)

## 5. What re-scopes in the build

### 5.1 The animator pose-graph — a second rig, not a replacement

- **The third-person pose-graph STAYS.** It is not deleted. It
  poses the operator in third-person play, NPCs, enemies
  (soldier, Hound, Weathervane, Neighbour, citizens), the
  mechas, and every presentation surface where the full figure
  appears (debrief room, menu, mirrors, compendium-adjacent
  scenes, death evidence). The M7 gaits, the wall-run family,
  and the grapple stance all remain as third-person poses — the
  world still needs them, because the world still contains the
  operator's body, seen by others, by the camera in
  presentation, and by the player himself in third-person mode.
- **First person needs a separate viewmodel rig.** Hands,
  sleeves, weapon, launcher, mantle hands — a rig parented to
  the camera with lag, driven by the same PoseFrame seam (joint
  transforms in, posed geometry out) but with its own pose
  set: low-ready, aim, reload (per weapon), launcher raise,
  strike, hang, swing tuck, wall-run trailing arm (frame-edge),
  mantle reach, lean. The viewmodel rig is NOT a shrunken
  third-person model — it is built for the eye's distance and
  the frame's edges, at viewmodel texel density.
- **One sim, two presentations:** the capsule, the rope, the
  wall-run budget, and the stimulus events are single — the
  viewmodel and the third-person figure are two renderings of
  the same state. If they ever disagree (the hands say latched,
  the body says falling), the sim is wrong, not the rig, and
  the proof that catches it is the same course replayed with
  both presentations recorded.

### 5.2 The operator-fidelity build — re-scoped, not restarted

BEAUTY-PASS.md §5 assumed third-person visibility and must be
re-scoped by this note (BEAUTY-PASS.md itself is not edited —
this section states what changes in it):

- **What survives unchanged:** the character material system
  (own atlas budget, cloth/webbing/leather/gunmetal treatments,
  skin never a flat fill), the proportion and silhouette rules
  (pinned to the references), and idle life through the animator.
  The materials still matter — the hands, sleeves, and
  presentation figure wear them.
- **What moves:** fidelity's centre of gravity moves from the
  full body at 20 m to (a) the VIEWMODEL at 0.5 m — hands,
  sleeves, weapon, launcher, all at viewmodel texel density, with
  the same per-distance read rules applied at arm's length
  (seams, stitching, wear, and the face-inward watch must read
  in the frame); and (b) the full figure at the distances it is
  actually seen — the third-person boom in play, and the
  PRESENTATION figure at conversation distance (debrief room,
  menu) — judged at those distances, not at patrol distance.
- **What is retired (first-person mode only):** the 20 m / 10 m /
  5 m silhouette-read requirements for the PLAYER's figure — in
  first person the player does not see himself at those
  distances, so proving his silhouette there proves nothing in
  that mode. In third-person mode his figure is on screen and
  the reads apply to him as to any figure. Either way the
  requirements TRANSFER to the NPCs and enemies (a soldier's
  silhouette at 20 m must still read his kit — that was always
  the perception law's demand).
- **The face question is settled by the perspective, not by a
  ruling:** the painted-quad face problem (§5 of the beauty
  pass) stops being a gameplay problem — the player never sees
  his own face in play — and becomes a presentation problem,
  judged in the debrief room at 2 m, where modeled head planes
  and a dedicated face texel budget still apply. The beauty
  pass's OPEN QUESTION (coat #31/#32 vs identity #40 for the
  in-game street figure) is answered in §2 of this note: the
  field figure reads as its CONFIGURATION, and the identity
  portrait governs the menu. No further ruling is needed on
  that question for the FP build — unless Chase reopens it.
- **The per-distance rules, restated:** at arm's length
  (viewmodel, first person) the kit must read; at boom distance
  (third-person play) the figure and its configuration must
  read; at conversation distance (debrief room) the face and the
  configuration must read; at patrol distance (NPCs) the
  silhouette must read. A figure that only reads at one distance
  has failed — the rule survives, the distances move.

### 5.3 The DESIGN.md wording — superseded, and the current state

This note originally prescribed first-person-only wording for two
places in DESIGN.md. That prescription is SUPERSEDED by the
standing ruling (DESIGN.md, "Standing rulings — 2026-10-30"):
first person native, third person fully supported, one movement
core, identical stealth math. The current state of the two places:

- **Header block:** reads "Perspective: first person native,
  third person fully supported (dual-mode)" — the ruling's
  wording. The first-person-only line this note first drafted
  for it ("Perspective: first-person · Genre: FPS stealth sim
  action game") was never the bible's settled text.
- **§8, Presentation:** carried the pre-ruling sentence —
  "Third-person camera with a law: free and low over the shoulder
  in open ground; in interiors, the camera takes composed,
  deliberate angles — the MGS1 lesson that a *placed* camera is
  atmosphere." Against the ruling it was incomplete: it stated
  the third-person rig's law and the interiors law, but not the
  native first-person mode. The parent fixed §8 in the same
  landing as this alignment — §8 now states both rigs' laws
  under one movement core, with identical stealth math.

**Companion note (not in DESIGN.md — for the parent to carry):**
PS15-DIRECTION.md golden (6) annotates the operator rear view with
"the back is the most-seen angle in third person." Under the
dual-mode ruling that annotation stands as written for
third-person mode — the rear view is what the third-person player
sees — and the rear view remains a golden standard and a
presentation angle (debrief room, menu, mirrors). In first-person
mode the most-seen angle is the viewmodel's hands and the visor
frame. The (6) entry itself (the rear-view image and its kit map)
is unchanged.

**Not changed (deliberately):** the §2 Syphon Filter line
("Third-person run-and-gun with momentum") describes an ANCESTOR,
not DeBrief, and stays as written. The §4 fast-stealth doctrine,
the grapple law, and the traversal section need no wording change
— they are perspective-neutral, except for the traversal brief's
pose-family presentation, re-scoped in §5.1 above.

## 6. What does NOT change

- **The city.** Greyport, the block as TILE, the districts, the
  vertical city-dungeon, the checkpoint furniture, the
  watchtowers, the wet streets — all untouched by the camera.
- **The systems.** The one-field light law, the perception fields
  (soldier sight/sound/memory, mecha governor tables, Hound
  scent), the witness economy, the checkpoint-and-papers system,
  the grades (Ghost, Surgeon, Butcher, Ghost-That-Never-Was),
  the kill ladder (avoid, blind/deafen, hobble, kill through the
  coolant line from above), death-and-inheritance (the career
  ends, the city keeps the body, the successor inherits the
  file), and the RPG layer (visor damage numbers, HUD, the
  two-layer skill tree, the sword as final unlock).
- **The story bible in progress.** The campaign arc, the lore,
  the dossiers, the mysteries that stay mysteries (what is in
  the Type 2, whether the handler is Control, the Wanderer's
  motive, Mister Ed's pod, the sword's provenance, the
  nightjars) — first person changes how the story is SEEN, not
  what it IS. The debrief room gains power from FP: the player
  has spent the op inside the eye, and now sits across the desk
  and sees the man he was.
- **The mod engine spec.** Props, registry entries, JSON recipes,
  staged v1/v2/v3 — perspective-neutral. A modder's prop is
  seen from the eye now; the registry does not care.
- **The beauty spec's world sections.** Materials, the city
  architecture kit, smoothness, weathers, lighting — all
  untouched, except §5's operator-fidelity section, re-scoped
  above. The world gets MORE scrutiny from the eye, not less:
  the beauty pass's proofs now include eye-view frames.
- **The outside-in order.** Engine, city, weathers, lighting,
  and so on. First person is engine-phase work (camera rig,
  viewmodel rig, eye-light read) and rides the engine phase —
  it does not reorder the phases, and it does not pull DeBrief
  content forward.

## 7. Open questions — for Chase's ruling, not decided here

1. **Shipped Nexuiz balance numbers.** The house tree holds
   `g_hook.qc` with reference values in its comments (pull 2000,
   rubber 2000, rubber overstretch 1000, min length 100, stretch
   400, air friction 0.2, hook health 120), but the shipped
   `balance25.cfg`/`physics25.cfg` values the thisissparta2
   config defers to were NOT verified in the house tree. The
   traversal build must confirm them from the release data (or
   re-derive them from the course) before converting them to
   C35 units. Nothing in §4.1 asserts them as shipped fact.
2. **`g_grappling_hook_tarzan` stock default.** The
   thisissparta2 config sets only `g_grappling_hook 1` and
   touches no tarzan cvar, so DeBrief's tarzan character is the
   stock default — but the stock default value itself was not
   verified in the house tree. If the stock default is
   non-tarzan (fly-to-point), DeBrief still matches tarzan by
   explicit choice, and this note records that as a choice, not
   as stock fidelity.
3. **Lean bind.** Q/E is OUT — E is grapple, fixed (Chase
   2026-10-30). The mechanic is specified; the pair is Chase's
   call, and it cannot include E.
4. **Base FOV and eye height numbers.** Specified here as rules
   with proofs (§1.1); the numbers have since SHIPPED in the
   dual-mode controller (C35 PR #38): first-person rig 55 base /
   50 aim / 60 max, third-person rig 50 / 45 / 55, eye at 1.62 m
   (DESIGN.md's standing ruling). Chase judges the frames, not
   the constants — the registers are named constants, retuned on
   his play feel if the frames say so.
5. **Comfort fallback.** If playtesting shows the swing still
   sickens, the fix is a named transition's easing (§4.4) — but
   whether a comfort option (reduced swing bank, reduced FOV
   swing) ships as a setting is Chase's call, not this note's.
6. **The #38 HUD frame's camera.** Golden #38 is a
   third-person swing frame (operator mid-swing, firing
   one-handed). Under FP, the #38 transition is re-framed from
   the eye (wall filling the frame, hand trailing at the edge).
   Whether #38's original framing is kept as a compendium
   presentation image or retired as a gameplay reference is a
   canon question for Chase — the frame's CHAIN (wall-run →
   swing → wall-run) survives either way.
