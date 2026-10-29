# DeBrief — Design Bible v0.1

*The game design document for DeBrief, the first game series built on the C35 (Comigen) engine.
Set by Chase Bryan's directives (2026-10-29); written to his vision. Status: design — the game
builds when the engine is called ready.*

**DeBrief — A City of Spies** (first entry)
Repo: `nightwarelabz/DeBrief` · Engine: C35 (Comigen), PlayStation 1.5 presentation ·
Perspective: third-person shooter · Genre: immersive simulator

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
