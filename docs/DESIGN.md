# DeBrief — Design Bible v0.1

*The game design document for DeBrief, the first game series built on the C35 (Comigen) engine.
Set by Chase Bryan's directives (2026-10-29); written to his vision. Status: design — the game
builds when the engine is called ready.*

**DeBrief — A City of Spies** (first entry)
Repo: `nightwarelabz/DeBrief` · Engine: C35 (Comigen), PlayStation 1.5 presentation ·
Perspective: first person native, third person fully supported (dual-mode) · Genre: immersive simulator

---

## 1. The pitch in one breath

1987. The Cold War did not end — it curdled. You are an NSA special agent working a drowned,
watchtowered port city where every agency on earth runs spies and nobody admits the war is
still on. Each operation is a story the city tells about you; each operation ends in a room,
under a light, where a man reads back everything you did — and the file is written forever.
The city never resets. The game never ends. Your record is the game.

## 2. The bloodline

Three ancestors, taken seriously and taken apart:

- **Hitman (Codename 47, the first)** — the social kill. Disguise as a verb: the right coat
  opens doors a silenced pistol never will. Targets who live on routines. Levels as open
  machines with a dozen solutions and one perfect one. Lethality as a *choice of costume*
  before it is a choice of weapon.
- **Metal Gear Solid 1** — the theatre of it. A handler in your ear (here: Control, on a
  scrambled landline frequency), gadgets as verbs, rival operatives with names, doctrines and
  monologues, a conspiracy spine that keeps unfolding, and set-piece encounters that play like
  duels of tradecraft rather than shooting galleries.
- **Ghost Recon (the original)** — the tactician's patience: open ground read like a map,
  approaches planned before they're walked, and ballistics with consequences — one good
  bullet ends anyone in this world, including you. Taken as seasoning, not as the dish:
  DeBrief is its own venture first; the ancestors are honoured by being surpassed in the
  directions they pointed.
- **Syphon Filter** — when it goes loud, it goes *all the way* loud. Third-person run-and-gun
  with momentum, precision shots that matter (the headshot is a skill, not a stat), missions
  that sprint from street to rooftop to train yard without asking permission.

What none of them had, and DeBrief does: **the debrief itself is the game loop** (§5), the
city is a persistent simulation that remembers (§6), and your senses are a signals officer's
senses — in 1987, that means analog (§4).

## 3. The world — 1987, dystopian, and the lore is load-bearing

The lore is not wallpaper. It is the mechanism: factions act on doctrine, doctrine comes from
history, and the player who reads the world plays better than the player who only aims.

- **The divergence (the spine):** the world's 1987 is a surveillance century arriving early.
  A Nazi-like movement — homegrown, flag-wrapped, and patient — has taken over the United
  States: not by invasion but by emergency, election, and decree, until the takeover was a
  fact nobody could point to the start of. What remains of the old republic governs nothing
  but its own memory. The city — an American port under occupation — lives under a permanent
  emergency that nobody remembers declaring. Watchtowers on the dockyards. Papers checked at
  bridges. A curfew that is "temporary," eleven years running.
- **The city of spies:** every intelligence service keeps a station here because the city is
  where the world's secrets change hands — dead drops in the market arcades, couriers on the
  night ferries, defectors in the safe hotels. Civilians live inside the machine: informers,
  double lives, ordinary people paying the city's prices.
- **The NSA's man:** the Agency survives as a service in exile and in shadow — a remnant
  that kept its ears when it lost its country, running agents inside the occupation it could
  not prevent. It is not the hero of this world; it is one player among many, with the best
  ears and its own dirty ledger. The protagonist — the operator of the banner — is its field
  asset in the city, and the series will interrogate his loyalty as hard as any target's.
- **The enemy:** the Regime's two instruments of fear. Its **soldiery** — terrorist soldiers
  in doctrine and practice: occupation troops who rule the districts by raid, reprisal, and
  spectacle, hunting the resistance and the Agency alike. And its **mechas** — armoured
  walking machines, the divergence's signature terror-tech: patrol walkers on the dockyards,
  siege frames at the bridges. A mecha is not a boss with a health bar; it is weather — a
  system of sensors, arcs, and armour that a professional avoids, blinds, misleads, or, on
  the best day of his life, kills from inside its blind spot with the city's own machinery.
  Fighting one head-on is how agents get files closed.
  **Canonical concept art (Chase, 2026-10-29 — in the DeBrief repo):** `enemy-soldier.jpg`
  — the Regime trooper: black greatcoat and pouches over dark armour, blank mask and
  goggles, suppressed rifle, and the faction's mark worn openly: a red shoulder and a red
  armband bearing **three white bars**. `enemy-mecha.jpg` — the walker: weathered grey
  plate over black machinery, a cannon for a right arm, one red eye-slit, and the same
  three bars on its red shoulder plate. One insignia on man and machine alike — the
  occupation brands everything it owns. (Local copies: ~/workspace/debrief/.)
- **Period truth as texture:** 1987 is rendered honestly — film, paper, tape, and wire.
  Payphones and dead-letter boxes. Mainframes behind glass. A bug is a physical object you
  plant and a physical object you can lose. No anachronistic magic: if the tech didn't exist,
  the tactic doesn't either. The constraint is the flavour.

The full world bible (factions, the divergence timeline, the city's districts and doctrines)
is its own living document and grows with the series; this bible fixes its laws.

## 4. The immersive simulator — systems, not scripts

An op is a simulation handed to the player with objectives, never a corridor of triggers.

- **Disguise and identity:** uniforms, papers, faces in crowds. A disguise is a hypothesis
  the world keeps testing — posture, route, and behaviour either hold it up or give it away.
- **Analog SIGINT — the DeBrief sense:** your visor and kit read the electromagnetic city:
  active bugs, wired rooms, camera circuits, radio traffic, tapped lines. In 1987 this is
  earned information — sweep a room and learn its ears; follow a wire and find its listener.
  It replaces the genre's magic radar with tradecraft the player performs.
- **The operator's weapon — the HK USP:** the Agency's standard sidearm in this timeline
  and the operator's signature: the pistol of the banner, carried in every op, the one
  constant in the kit. (In the world's own history the USP program arrived early — one of
  the divergence's small, documented facts, the kind the world bible keeps receipts for.)
  Other weapons are taken from the world and left in it; the USP comes home.
- **Gadgets as verbs:** lock picks, film cameras, tape recorders, planted microphones, a
  garrote, a silenced pistol, a flare, a coin. Every tool opens a class of solutions; the kit
  you carry in is the first decision of every op.
- **Fast stealth — the movement doctrine:** stealth here is not slowness. The agent is a
  professional and moves like one: sprint, slide, mantle, drop, vanish — at full tempo.
  Speed is itself a stealth system: patrols act on what they saw a moment ago, sound is a
  radius and a duration, and a ghost who crosses a courtyard in two seconds was never really
  there. The file grades time alongside traces — the perfect op is fast *and* clean, and the
  action mechanics (aiming on the move, takedowns at speed, gunplay without stopping) are
  built for a player who refuses to crawl.
- **The grappling hook — everything grapples:** the signature traversal verb, and the
  city's geometry is built for it. EVERY surface takes the hook — parapets, ledges, cranes,
  wire masts, tower rails, the sheer faces of the towers themselves. There are no invisible
  walls against it and no blessed anchor points: if it stands in the world, it can be hooked,
  climbed, swung from, or dropped from. The constraints are physical, not artificial — line
  length, the strike's noise, exposure while you climb, momentum on the swing — and the
  levels are designed as a **city-dungeon**: tall buildings and walls, courtyards as rooms,
  rooftops as the dungeon's upper floor, every district a vertical labyrinth whose fastest
  route is rarely the street. The grapple is how a professional crosses a watched square
  without ever being in it.
- **The world answers:** bodies are found and acted on. Alarms change patrols permanently
  for that op. A witness is a thread that either gets cut, bought, or pulled — and pulling it
  unravels into later ops. Light, noise, weather, and the hour are systems, not set dressing.
- **No failure state reload:** an op that goes wrong keeps going wrong, forward. You extract
  with what you have, or you don't extract. The debrief records what actually happened (§5).

## 5. The debrief — the loop that makes it ours

Every operation ends the same way: a small room, a recorder, your handler's voice.

- The op is **read back**: what you did, what you left, who saw you, what the blood says
  (§7). The game grades the file, not a score screen — Ghost, Surgeon, Butcher, Ghost-That-
  Never-Was; the words are the Agency's, and they follow you.
- **The service record is the save file:** your career — clearances, reputation with the
  city's factions, heat, commendations, black marks — persists across ops and shapes what
  the city offers and what Control trusts you with.
- **Burned agents, persistent city:** if an agent is burned — exposed, captured, killed —
  the career ends and the *city does not care*: a new agent arrives, inherits the same city
  with its same grudges and unfinished business, and reads the predecessor's file. The series'
  memory outlives its protagonists.

## 6. Never-ending, replayable by construction

- **The city generates the war:** faction dynamics — couriers moving, assets flipping,
  stations rising and burning — compose new operations from the simulation itself: tails,
  dead drops, extractions, eliminations, burglaries of secrets, counter-surveillance sweeps.
  Hand-built **story ops** (the spine) interleave with systemic ops, and the systemic city
  keeps score between them.
- **Replay is a doctrine, not a mode:** every level replays under new conditions — the hour
  (the world runs on the player's real clock through the engine's reality layer), the season,
  the alertness the city has *learned* about you, different kit, different intel, different
  orders. A level mastered as a ghost returns as a different level when the city is hunting.
- **Mastery is the content:** grades, ghost runs, no-trace runs, full-archive runs (every
  secret in the level filed) — the archive of a level is the completion that matters.

## 7. Blood — gore as evidence, not horror

There will be blood and giblets, at Resident Evil weight — and this is not a horror game.

- **Violence has a physics and a price:** wounds are specific, bodies stay, blood pools,
  spreads, and testifies. The loud path writes its story on the walls, and the city's
  investigators — and your debrief — can read it.
- **Gore is the receipt of a decision:** stealth, done right, leaves a clean file. A massacre
  leaves a massacre, and the record says so, and the city prices the Agency's man accordingly.
  The gore system exists so that force is *legible* — to the world, to Control, to the player.
- **Not horror:** nothing here jumps out to frighten. The dread is bureaucratic and moral —
  a quiet room, a red light on a recorder, a man reading your own work back to you.

## 8. Presentation — PlayStation 1.5, enforced by the engine

Per the C35 direction (`~/workspace/c35-stage/PS15-DIRECTION.md`): chunky low-poly models with
big readable silhouettes, flat facets, low-resolution generated surfaces, fog and night grades
like the banner — cleaner than PS1 (no affine warp, no vertex wobble), short of PS2. 1987's
city in that style: sodium light, wet concrete, watchtowers, neon through rain. Third-person
camera with a law: free and low over the shoulder in open ground; in interiors, the camera
takes composed, deliberate angles — the MGS1 lesson that a *placed* camera is atmosphere.

## 9. Standing rulings carried into this design

- The protagonist is the operator of the banner and the reference image — an NSA special
  agent; the player *is* that agent, career after career (§5's burned-agent rule).
- The series is called **DeBrief**; the first entry is **A City of Spies**.
- Lore is extremely important: the world bible is content, not flavour text, and faction
  doctrine drives the simulation that generates ops (§3, §6).
- Stealth is the base genre, action is the tempo: extreme action mechanics, a proficient
  fast-moving agent, and the grappling hook as the signature traversal tool (§4).
- Sequencing: the game builds on C35 when the engine is called ready; the engine's M6
  (the PS1.5 pivot + the operator pipeline) is aimed at this document.

## Standing rulings — 2026-10-30 (Chase)

- **One operator.** The game has a single main character: the operator. There is no
  playable cast and no co-op spine — "focus on one character as the main character that
  just seems a lot simpler than getting mixed up with multiple agents and things like
  that." Complexity is welcome: "It doesn't matter if it's advanced and complicated.
  It just needs to be really well done." The story is lived through his eyes.
- **A city of spies is inheritance, not co-op.** Other agents are never playable.
  When the operator dies, a successor inherits the world he left: the traces persist
  between careers — a window he broke stays broken, his tagged echoes and his
  half-finished routes remain in the city for the next agent to find. The city of
  spies is what the world remembers, not a roster.
- **The visor plays in signals and sound.** The visor can tag a sound and leave a
  ghost echo of it in the world; signal and sound are a first-class play layer, on
  the same law as light. The visor's sound identity is its power-on motif (approved
  2026-10-30), the anchor every other visor sound derives from.
- **Camera: first person native, third person supported.** First person is the
  graded truth — eye at 1.62 m, the lean, the aim, the visor frame — and the game
  is tuned and balanced from it. Third person is fully supported on the same
  movement core with its own rig; it carries no separate balance, and the stealth
  math is identical in both modes.
- **The hanging banner's bars are vertical.** The three bars on the in-engine
  hanging street banners stand vertical, as ruled: "Uh no they are supposed to be
  vertical." (The horizontal stripes belong to the enemy flag artwork — a different
  object.)
- **The graphics gap is on record.** The current engine graphics stand rated by
  Chase at "about a one out of ten maybe a twelve out of one hundred" (2026-10-30),
  with the ruling "It needs to be significant." That is the standing gap the
  graphics work must close against the golden standards — the banner's temperature
  above all. Performance is not the gap; the picture is.

## Mecha damage & destruction (Skelli's proposal, 2026-10-29 — pilot fork open)

LAW: you do not out-shoot a mecha; you out-think it, and the city is the weapon.
No health bar, no DPS. Damage is STATE — specific physical insults, each with a
visible system effect. Small arms cannot hurt plate; honesty is the design.

THE MACHINE AS SYSTEMS: sensors (the eye — blind it and it falls back to audio,
vibration, last-known-position; partial blindness cuts true blind wedges into
its arcs) · locomotion (the ground is taken from it: crane loads, dropped
containers, slicks, rubble, cut street cables — limp states, wider turning,
streets it can no longer enter) · the cannon (cannot be destroyed by the
agent's weapons; its fire can be BAITED — the Regime's own gun turned on its
own checkpoint, fuel store, or support) · power/coolant (runs EXPOSED on the
bare back — golden #8 — because doctrine never expected one man behind it:
the back is the intended kill path, reached from above; the grapple buys the
angle).

THE KILL LADDER: (0) avoid it — the grade rewards never meeting it · (1) blind
and deafen it (sensors, steam, smoke, bloomed night optics) · (2) hobble it
with the terrain · (3) kill it — always preparation + position, never a
firefight: a placed charge on the back runs from a rooftop · industrial
machinery (crane drop, dry dock, freight shaft, the fuel depot it is lured
beside) · its own cannon, baited · and, if the link ruling passes, the soft
kill: jam or spoof the control link and it shuts down or wanders home.

PERSISTENCE: damage persists across ops in the persistent city. A blinded eye
stays blind until Regime logistics repair it; a soft-killed machine is
RECOVERED and returns; a destroyed one is gone but raises the city's alert
budget. The same unit returns with weld plates over old wounds — the city's
memory made visible. Every hard kill is loud and witnessed, and the record
prices it: a Ghost never kills one.

FAIRNESS: it telegraphs (servo noise, the eye's sweep, ground tremor); its
perception runs on the same light/sound fields as everything else; the grab
(golden #12) is survivable at a cost — a struggle window, gear left in its
claw, injury carried into the record. Never instant death.

RULED (Chase, 2026-10-29): AUTONOMOUS. No pilot, no control link, no operator
in a van. An onboard analog governor — cams, relays, gyroscopes, punched
doctrine — runs the machine. Consequences, worked through:
- There is NO signals kill. Nothing to jam, spoof, or negotiate with. The
  SIGINT trade reads the machine instead: it radiates and whines and ranges,
  and the visor/sense mode reads its emissions to tell its state — searching,
  suspicious, locked. You read its mind through its sounds.
- Its doctrine is FIXED, therefore learnable to the letter. The Archive
  accumulates its stimulus/response tables op over op: what it does when it
  hears glass, loses the light, loses you. Player knowledge is the weapon;
  mastery is predicting it perfectly. The debrief grades what you knew.
- Misdirection is physical, never electronic: thrown weight, cut power to a
  block (darkness rebalances its sensors), rain and the foghorn as noise
  cover, decoys. The Dark Mod stimulus coupling IS its mind.
- The soft kill becomes the doctrine trap: because its logic is fixed it can
  be LED — into the dry dock, the freight shaft, the street it cannot leave —
  and held there until its power dies or the Regime cranes it out.
- Damage compounds through the governor: a blinded machine compensates in
  fixed, readable ways (leans on audio), so last op's wound is this op's
  exploit. Persistence plus predictability is the whole late-game skill.
- The horror is bureaucratic: nobody hesitates inside it, nobody surrenders
  it, nobody recalls it. The Regime built a machine because men asked
  questions. It does not.

## Perception: the light read and the senses (Skelli's design, 2026-10-29)

ONE FIELD LAW: a single practical-light field serves the renderer AND every
perceiver. There are no separate "AI vision" numbers anywhere in the game.
What the frame shows is what a guard's eyes receive; the lamp pools on the
wet street are geography, not decoration. If a screenshot looks safe, it is
safe — the game never lies with light.

THE SOLDIER (human perception): stimuli are visual (light level x motion x
posture — crouched and still in shadow is nearly invisible; upright and
running in a lamp pool is seen street-length), audio (footsteps priced by
surface and speed; thrown weight; gunfire means investigate), and memory
(last known position, a search that decays). His states — unaware,
suspicious, searching, alarmed — are carried in posture and sound (the
weapon coming up, the whistle, the radio crackle), never in icons. He is a
conscript, not a machine: he hesitates, he can be wrong, rain and cold
degrade him, and he fears the mecha — a patrol yields out of its arcs, a
tell the player learns to read like weather.

THE MECHA (governor perception): the same fields, rigid tables (see the
autonomy ruling). Its suspicion is threshold machinery with fixed timers;
it does not doubt the way men doubt. Its emissions — servo whine, ranging,
vents — are data: the sense mode reads its state from its sounds.

THE SENSE VIEW (the visor): the world rendered as its signals. Emitters
bloom — radios, idling engines, the mecha thinking aloud. Other men's
attention is carried by tone and by sound, not HUD markers: the view stays
sparse, analog, photographic in its own way. It is the agent's trade made
visible, and the debrief grades its use: did you look before you moved?

SLICE 1 ships only the light read itself: lit / unlit at the operator's
position, logged against the patrol's facing on the proof drive. Everything
above is the spec that read grows into.

## Death and bodies

No fades, no sinks, no despawn. What dies in Greyport stays in Greyport until
the op closes — a body is evidence, and evidence is a system.

- **Persistence.** Corpses persist for the whole op, where they fell, in the
  state they were left. Blood pools spread on wet ground and stay; heavy
  impacts and the mecha's violence leave giblets. Blood trails mark a wounded
  man's path. The debrief reads all of it: an unfound body and a found one
  are different records.
- **Discovery.** A patrol that finds a body changes state — alarm, search,
  a report the Archive hears. Moving and hiding bodies is therefore core
  stealth craft, priced in time and exposure, exactly like the grapple.
- **Wounds before death.** A shot soldier is not instantly a corpse: wound
  states — down, crawling, bleeding out, calling out — and his sounds are
  stimuli in the same fields as everything else. Finishing him is a choice
  the record sees. So is aid. So is walking past.
- **Gibbing has a threshold.** Small arms wound and kill honestly; gibs
  belong to heavy force — the mecha, explosives, machinery, falls. Gore at
  Resident Evil weight, rendered in the era's own language: blood as palette
  and pooling decal, giblets as low-poly chunks with full surface craft.
  PS1.5 gore is geometry, not a filter.
- **The operator's death** ends the career, not the game: the file closes,
  the city keeps the body where it fell, and the next agent — who inherits
  the same persistent Greyport — can be sent to the same street and find
  what the mecha left. The grab, the lift, and the death in the golden
  canon are not cutscenes. They are a system the player survives or joins.

## Checkpoints and papers

The Regime does not only patrol Greyport; it processes it. The checkpoint is
where the occupation touches ordinary life, and it is the game's social
stealth heart — the Hitman blood in the design.

- **The furniture.** Barriers, a boom gate, a kiosk, a queue of citizens.
  Golden #9's street left it out; Slice 1 dresses a corner of it; the full
  composition is its own concept piece, still owed.
- **The verb is HALT.** The soldier's raised left palm (golden #15, rear
  pose) is the checkpoint's grammar: stop, be seen, be read. Papers are
  presented, not clicked — the inspection takes real time, in the open,
  under the lamp, while the queue watches and the mecha's tread shakes
  the kiosk glass two streets away.
- **Papers are a system.** Identity is layered: face, dress, papers,
  behaviour. A forgery is a physical object the Archive makes — it can be
  good, stale, or for a man who died last week. The inspector is a
  conscript with a quota and a mood; rain, cold, and fear of the mecha
  all degrade him (the perception laws apply to him first).
- **Failure is graduated.** A bad read is a question, then a detention,
  then the palm turning into the other poses on the action sheet. The
  checkpoint can be avoided (the rooftops are the upper floor), bribed
  only with what a conscript risks his life to hold, or passed clean —
  and a clean pass is information: you learn the rota, the inspector,
  the hour the lamp battery dies.
- **The record prices it.** Every crossing, clean or not, is a line in
  the debrief. The Archive is built out of crossings.

## The handler

The debrief has a face (golden standards #18/#19). The handler is the man
across the desk: gaunt, weathered, grey at the temples, deep-set eyes that
have read too many files. Charcoal checker-wool greatcoat over a three-piece
suit, leather gloves, and a brass-clasped briefcase in his RIGHT hand in
both views — the Archive's paperwork made portable; the case is part of the
silhouette, not an accessory. The player knows him from the front (across
the desk, grading you) and from behind (walking you in, walking you out);
the rear view — centre-vented coat, buttoned sleeve straps — is canon with
the same weight as the front. His voice is the narrator's voice: gruff,
harsh, slow, methodical. Whether he is Control or only Control's face is
deliberately unstated — the voice on the record and the man in the room may
or may not be one person, and the game does not answer it. He belongs to
the room, the desk, and the record; he is not a field character.

## Planned enemies: the Hound, the Weathervane, the Neighbour

Adopted as a set (Chase, 2026-10-29). The soldier owns the street by
sight, the mecha owns the district by presence; these three own the senses
and layers the first two cannot.

### The Hound (first — concept landed 2026-10-29, golden #20–#22)

A scent-tracking dog worked by a handler (a soldier variant: same
greatcoat family, long tracking lead, coiled second line, canvas scent bag
at the belt). The dog: a large dark-sable shepherd, erect ears, unmuzzled
when working, in a leather-and-webbing Regime harness with a small
three-bars plate on the LEFT shoulder strap — heraldry discipline as with
the soldier's armband. Regime paperwork: a scent asset with a serial
number, Tracking Section. The Archive's name: the Hound.

The system it adds is SCENT — the third perception channel, and the only
one the operator cannot watch himself spend. His trail persists on the
ground he crosses and decays over minutes; crossing his own trail freshens
it (the dog works faster, not slower). Rain washes it (the weather
engine's stealth price), wind carries it (approach from downwind or do
not approach), water breaks it. The dog's state reads from posture alone,
per the house law: nose down and pulling = tracking; head up, still =
scent held; hackles, line straining = contact. The visor cannot show the
player his own scent picture — the debrief grades it after the fact ("the
dog had you from the corner of —"). Killing the dog is priced in the
record like everything; the handler's grief is the most human thing the
Regime owns, and the game lets it be witnessed.

### The Weathervane (concept landed 2026-10-29, golden #23–#24)

A rooftop observer: scope, field telephone, patience. He almost never
shoots — he REPORTS, and the street below answers (a telephone call is a
perception event the whole district hears the consequences of). He owns
the high sightlines the grapple crosses, contesting the vertical layer;
his hearing is poor, his rotation is scheduled, his silhouette against the
sky is his tell. Regime: Observation Post (Elevated). The Archive: the
Weathervane — he turns to face whatever the street is doing.

### The Neighbour (concept landed 2026-10-29, golden #25–#26)

A plain-clothes informer: no uniform, no weapon but a notebook and the
telephone kiosk on the corner. Unreadable by dress — readable only by
behaviour (he watches, he follows at a distance, he writes). His success
state is not a gunfight: it is your description, entered into the other
archive. The social layer's enemy, and the checkpoint system's shadow.
Regime: Block Warden (Volunteer). The Archive: the Neighbour — anyone's.

## The wanderer (adopted 2026-10-29, Chase; concept landed golden #27–#28)

The operator's one ally, and the city's one unexplained person: a Japanese
Zen monk, a wanderer, who shows herself rarely and belongs to no side's
machinery. The NSA does not run her; nobody runs her. The Archive holds a
folder on her with a single page — her description, marked UNRESOLVED. The
Regime's other archive holds the same page. She is the one person in
Greyport with no file, in a city made of files.

APPEARANCE IS DOCTRINE, LEARNABLE LIKE THE MECHA'S: (1) she appears after
mercy — an op where the operator aided the wounded or spared a man raises
the chance she is somewhere quiet in the next; (2) she tends the badly
wounded operator when he is alone — the game's only outside aid, priced in
TIME against the op clock; (3) never during alarm, never twice in one op,
never when the Neighbour's description of the operator is active — she
knows when she is watched, or the city knows for her.

GRACE IS A FIELD, like light is a field: near her, soldiers' suspicion
rises slower (she tends their wounded too, and a conscript's hesitation
around her is priced into the math); the Hound will not track across her
raked gravel — the scent breaks at her ground; the Neighbour lowers his
notebook, because to his eyes she is nothing at all. In the visor's sense
view she barely registers: no emitter, no signal. Silence, read as data.

HER GROUND: the Regime tolerates one temple as a pressure valve and a
propaganda of tolerance; patrols do not enter. That tolerance has a
BUDGET — world persistence prices it: use her cover too often and the
temple gets watched, then closed. Grace can be spent by the city, not
only by the player. She aids ANY wounded, enemy included — aiding a downed
soldier beside her is a record-read choice, and the debrief may note it.
Across a career she can die; the successor inherits the garden, the raked
gravel, and the systems, as her ghost.

HER PRICE AND HER MYSTERY: her aid costs time, never goods. What she
wants is never answered — the game does not say whether she is a defector
of the Emergency years, the Archive founder's last agent, or simply a
monk, and the mystery is that there is none. The handler knows something:
a debrief may contain one line — "You were seen sitting with someone at
——. The record will omit it. This once." Whether that protects her, the
operator, or the handler's own file is unstated, like his nature.

## The operator — two configurations (adopted 2026-10-29, Chase)

One man, two kit states, built on one body and one skeleton. The STREET
configuration is the coat (golden #31 front, #32 rear — the rebooted
founding concept): how he moves through the city, queues, and passes
checkpoints as nobody. The ASSAULT configuration is the black set
(golden #33 front, #34–#35 rear): plate carrier, balaclava, headset —
what he wears when the work goes direct and being recognised stops
mattering more than being protected.

Two corrections bind the model pass, ruled with the adoption: the
assault visor is DARK and never glows — red light is the mecha's
language, and nothing on the man emits; and the NSA sleeve patch is
DELETED — no labels, letters, or insignia anywhere on him in either
configuration. An exile service does not sign its work.

The configurations are priced by the systems already written. Assault
cannot pass a checkpoint — the papers doctrine reads kit as loudly as
faces. Being witnessed in assault is the most expensive description
the Neighbour and the witness economy can carry. Changing
configuration takes time and a private place; it is a verb for a
later slice, noted here, not built under the operator-first directive.
The perfection work covers both configurations: same silhouette
discipline, same kit map, same man underneath.

### His main identity (ruled 2026-10-30, Chase)

Chase: the latest operator image made in-house — golden #40, the mesh
study: the banner pose, the black-camouflage balaclava, and the
single-eagle NSA patch on the arm that Chase had corrected to one
eagle — "be his new identity. His main identity."

So #40 is now the defining portrait of the operator: the balaclava'd
mesh-study figure is who he IS, ahead of either configuration's
reference sheet. This revises the standing above in two named ways:
the street coat is a configuration he wears, no longer the identity
default; and the single-eagle patch belongs to the identity portrait
itself. What the ruling does not settle, and is not stretched to
cover: whether the in-game model wears the patch and the balaclava at
all times, or the identity governs presentation (banner, profile,
cover) while configurations govern the field. That question rides to
the model pass with the ruling on record. The new banner Chase
commissioned the same night is drawn to this identity — it is the
aim the graphics run at, on his ruling that this grade of image is
what C35 on the EliteDesk runs.

FINISH (2026-10-30, later the same night): Chase sent the
finished-detail portrait of the same pose and kit — filed
`debrief-operator-identity-reference.webp` — and designated it the
reference: the identity's finish is FULL detail (fabric, leather,
gunmetal; the translucent red visor; the gold-eagle NSA patch with
its serif letters), with NO wire mesh anywhere on him. The mesh
study was the pose's draft; this portrait is the finish. He
confirmed explicitly that this detailed grade is the one C35 on the
EliteDesk runs at full quality — the banner and the model pass aim
at this portrait.

## The RPG layer: numbers, the HUD, and the skill tree (adopted 2026-10-29, Chase)

Chase: "there should be an RPG element to the game. Shooting enemies
projects damage numbers etc. the player has a hud and systems they use
to engage the world. A full skill tree etc" — and, on the mecha
problem: "That will solve some of the problems with how the mechas and
etc are killed."

The layer is real and it is diegetic: the numbers come through the
VISOR. Analog SIGINT is the sense mode; the visor that reads the
machine also grades the operator's own hits — impact, penetration,
component struck. Damage numbers report what YOUR action did, never
what an enemy thinks: the perception law stands untouched, and enemy
state is still read from posture alone. The HUD is the visor and the
analog instruments — wound state, ammunition, the op clock the
doctrine already grades by — not a video-game overlay pasted on the
world.

The mecha keeps its law: NO health bar. What the numbers change is
legibility and reach. A hit on the machine reports its component —
SENSOR, LOCOMOTION, CANNON, POWER-COOLANT — and a skilled visor reads
component integrity, so the kill ladder (avoid, blind/deafen, hobble,
kill through the coolant line from above) becomes something a player
can measure, plan, and execute instead of guess at. The ladder itself
does not move: small arms still cannot simply shoot a mecha to death.
What grows is the operator.

The skill tree has two layers, because careers end. The OPERATOR
layer is the man's own craft — marksmanship and force, grappling and
movement, field endurance — and it dies with him. The SERVICE layer
is the Archive's institutional memory — visor depth, forgery quality,
district knowledge, the handlers' accumulated tradecraft — and it
persists across agents, inherited with the city and the file, like
the model kit on the desk. Progression is paid by the DEBRIEF:
grades and completed objectives are the currency, spent at the desk
between ops, never in a field pause menu. The three branches map to
the grades the game already gives: FORCE (the Butcher's road),
TRADECRAFT (the Ghost's — papers, configurations, checkpoint talk,
reading the Neighbour), and FIELD SYSTEMS (the Surgeon's precision —
visor depth, grapple mastery, scent discipline, component surgery on
machines).

Design adopted now; build order unchanged — the operator is
perfected first, and the RPG layer takes its own milestone in turn.

## Weapons and the final unlock (adopted 2026-10-29, Chase)

The operator equips different weapons across the progression — the
USP is the standard sidearm, not the whole armoury. The issued rack
is golden #36, FIELD ARMAMENTS / 1987, eight weapons in progression
order: 01 Service Pistol, 02 Suppressed SMG (the Ghost's gun — the
only quiet firearm), 03 Field Carbine, 04 Battle Rifle, 05 Pump
Shotgun, 06 Marksman Rifle, 07 Squad Automatic, 08 Single-Shot
Launcher (concussion works on machine components — it can blind and
hobble a mecha; the kill still belongs to the coolant line). Noise
is each weapon's true price: the sheet reads top-to-bottom as a
ladder of escalating stimulus as much as escalating force. Weapons are
priced like everything else in this design: a gunshot is a stimulus
the whole district hears, the grapple launcher commits both hands,
and a slung weapon is a silhouette a checkpoint reads.

The skill tree's FINAL unlock — the last thing the progression gives,
earned at the tree's end and never before — is a samurai sword.
Its place in the systems is exact: a blade makes no report. No
gunshot stimulus, nothing for the Hound's ears or a patrol's alarm
chain — the quietest kill in the game, priced in range: arm's length
is the whole of its reach, and closing that distance is the risk the
unlock asks the player to carry. And against the mecha it completes
the doctrine the RPG layer made legible: a pistol cannot cut a
coolant line. From above, by grapple, onto the bare machinery of the
back — a blade can. The final unlock is the kill ladder's top rung
given an edge.

Its receipt, in a world where every anachronism carries one, points
at the temple and stops there: the game's one ally is a Japanese Zen
monk, and the Archive's page on the sword's provenance, like its
page on her, is marked UNRESOLVED. The handler does not explain it.
Neither does the game.

## Mecha Type 2 — the white one (adopted 2026-10-29, Chase)

Chase's concept, verbatim in substance: a white mecha Type 2 that is a
female figure but incredibly obese, moves extremely fast, and screams
at the player like a soul trapped inside a mecha suit while she runs
at the player and attacks them.

She is the Type 1's inversion. The walker is a doctrine without a
soul — autonomous, signal-dead, patient. She is a machine with
something in it, and everything about her is immediacy. Her plate is
institutional WHITE, and under the one-field law that is a systems
fact: white under sodium light is always lit. She cannot use shadow,
cannot approach unseen, and does not care — the stealth economy the
whole game runs on simply does not price her. You will see her
coming. It will not save you by itself.

DOCTRINE IS MOMENTUM. Mass at her speed cannot turn: she charges in
lines, overshoots, and pays in street furniture and recovery seconds
while her overstressed locomotion hauls her around. Fixed, learnable,
fair — the sidestep is a skill and the charge line is a readable
tell. She cannot climb and cannot follow a grapple; rooftops are
refuge, which makes the Weathervane's ground the most contested in
the district. Contact is not the Type 1's held, survivable grab —
mass at speed is the heavy force the death design names as
gib-capable. Distance is life.

THE SCREAM IS A STIMULUS. It carries around corners and through
rain; the district hears her — soldiers orient on it, citizens
scatter, the Hound's ears go up. She is an alarm that hunts. Her
voice apparatus is therefore a component the visor can grade: a
marksman can silence her, and the charge keeps coming while the
district stops hearing it — a tactical mercy with a record price,
and the debrief will have a line about it. Between charges she is
nearly quiet: breathing, through a speaker. Sometimes the screaming
resolves almost into words. Almost.

ESCALATION, NOT POPULATION. She is what a risen alert budget buys —
kill a Type 1 and the Administration's answer may be her. She is
never a standing patrol, never common, and her rarity is a tone law:
she is an event the city commits, not a resident of it. Her full
damage model is designed when her slice comes; fixed now are
momentum, the scream, the escalation price, and the rarity.

WHAT IS IN HER IS NOT ANSWERED. Volunteer, prisoner, punishment, or
a recording of a woman who is no longer in there — the Archive's
page on the Type 2 programme is thin and partly unfiled, and the
game does not resolve it. In the Archive she is TYPE 2. The street's
names for her are standing proposals, undecided: THE BRIDE, for the
white; MOTHER, for the shape and the sound of her coming.

Her accepted identity render (golden #37) fixes the details: the
three bars ride a RED field on her LEFT pauldron — the Type 1's
insignia law — the speaker grille sits in her throat, her face is a
smooth dark dome with no eyes and no glow, and her white is not
parade wear but a ruin of white: chipped to dark metal, rusted at
the seams, still the brightest thing any street will ever hold.

## The citizens (adopted 2026-10-29, Chase)

Greyport is populated, and its population is a system, not set
dressing. The citizens are built as a KIT: a handful of base
silhouettes varied by coat, head, and palette regions on one
generated atlas — a crowd of individuals, never one cloned face
repeated. The founding pair are golden #29, NPC Male 1, the
dockhand — broad, patched, a lunch tin in his hand, a face that at
twenty metres in rain could be mistaken for the Neighbour's, a
confusion the design prices in on purpose — and #30, NPC Female 1,
the queue-keeper: belted raincoat, headscarf, string bag, purse in
both hands, and the assessing sideways look of the best witness in
the game.

They work four jobs in the simulation. THE WITNESS ECONOMY: a
citizen who sees a body, a drawn weapon, or a man climbing a wall
runs for a telephone kiosk — a second stealth clock the visor never
shows you: who saw, and how far they are from the nearest phone.
THE CHECKPOINT QUEUE: the other half of social stealth, a crowd
that shuffles, watches, and remembers faces. MORAL WEIGHT: bystanders
price the Butcher's road in the grading — what a street saw cannot
be un-seen by the record. And ordinary life on the same perception
fields as everyone else: they shelter from rain, freeze and look
down for the mecha, scatter at gunfire — and at a scream.

Two cast members are citizens by construction. The Neighbour is
built from this kit on purpose: the informer is terrifying because
he is the crowd, with a notebook. And the Wanderer is the citizens'
saint before she is the operator's ally — the offerings at her
temple gate are theirs. The citizens arrive with the checkpoint
slice; under the operator-first directive their build waits its
turn like everyone else's.

## Traversal: the grapple, and wall-running (wall-running adopted
2026-10-29, Chase)

The grapple law stands from the founding design: EVERYTHING
grapples — the constraint is physical, never permission; the
launcher is a two-handed, drum-fed tool (golden #16), so hooking
commits both hands. WALL-RUNNING joins it as the second traversal
verb, with its own physics and its own animation family, because
Greyport is vertical: the rooftops are a second city and the walls
are its streets.

The physics are honest, priced like everything else. A wall-run is
bought with approach speed and spent against gravity — the run
decays, the arc falls, and the exit is a jump, a grapple, or the
ground. Surfaces rule: dry brick holds; wet concrete in the rain
shortens the run and slickens the exit — the weather engine prices
traversal the way it prices hearing. The animations are a pose
family of their own (the lean, the wall-side arm trailing for
balance, the coat streaming) chained by the pose graph: sprint into
wall-run, wall-run into grapple swing, swing into wall-run on the
far wall — golden #38 is the founding frame of exactly that
transition, the operator leaving a dockyard wall on the line,
firing one-handed as he goes.

And it is LOUD in every channel the game reads. Boots on a wall are
footstep stimuli on a vertical surface; a man running along a wall
is a silhouette against the sky and the lamplight — the
Weathervane's dream, and the witness economy's easiest sentence:
citizens look UP at that. Wall-running is the fastest way across a
block and the fastest way into the other archive's description of
you. Speed is a stealth system; altitude is its price list. It
lands with the traversal milestone, after the operator — physics
slice (capsule, the rope, wall adhesion) and the animator's
wall-run family together, never one without the other.

## The nightjars (adopted 2026-10-29, Chase)

Nightjars are in Greyport, and nobody explains them. A wild bird in an
occupied dockyard city should not be there; the game never says why
they are. Laws:

- **Rare.** A handful in the whole city, perched throughout it — on
  wires, crane arms, rooftop edges. Most players will finish an op
  having seen none.
- **Heard before seen.** Their eerie churring call carries at night
  between the cranes — a sound-engine event with no source marker;
  the visor does not tag it. If you follow the sound and move
  quietly, you may find the bird.
- **A collectible.** Each nightjar found is a rare collectible in the
  service record's margin — counted, never explained, never cashed
  in for power. Finding one asks nothing of the player but
  attention.
- **Mystery is the mechanic.** No lore entry resolves them; the
  handler does not remark on them; the Archive holds no page. They
  are the one thing in Greyport that is not filed. (Out-of-world:
  the nightjar is the lab's own mark — NIGHTWARE LABZ' bird, sitting
  inside its game unexplained, exactly as a signature should be.)

## Mister Ed — the fable of the keeper (adopted 2026-10-29, Chase)

There is a fable Greyport tells about itself, and the Archive's
page on it is the thinnest in the building. Somewhere there is a
room that is not on any district plan: a desk, a lamp, a pipe gone
to ember — and Mister Ed, grey and long-limbed in work clothes,
boots up on the desk, keeping the machine that keeps the world.

The machine is an EliteDesk 800 G1. In 1987 that box is wrong by
four decades — no receipt exists, and the fable offers none; it
simply insists. Inside its case, where the drives should be, an
amber pod grows around something fetal and patient, and on the pod
a placard reads SYSTEMZ: the world's heart, or its replacement
being grown for when the first one fails. What is inside the pod is
unresolved by design, in the standing company of Control's face and
the white mecha's passenger.

His doctrine is maintenance, never intervention. He does not save
the operator, warn him, or grade him — he keeps the lamps lit and
the rain falling and the districts stitched, and the proof of his
shift is that the world is still there in the morning. His traces
are repair-shaped: a quarter that stuttered and recovered, damage
the city quietly healed overnight, weather that restarts like
someone re-seated a card. He is never met and never fought. If a
player ever finds the room, the chair is warm, the pipe is lit,
and he is not in it.

The fable is also, out of fiction, simply true: the world the
player plays in runs on an EliteDesk 800 G1 — the same machine
whose shift reports are the hardware ledgers (139 fps, Greyport
holding its light read digit for digit). Mister Ed was on duty
for every one of them.
