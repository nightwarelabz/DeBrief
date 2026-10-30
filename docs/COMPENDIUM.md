# DeBrief — The Compendium (Archive) System Spec

*Design only. No tree edits, no repo writes. Written 2026-10-30 at Chase's
directive: "there should also be a compendium that auto updates in the
menu screen or whenever the operator is back at the debrief room."*

Companion document: `STORY-BIBLE.md` (the lore this system records).
Canon sources: `~/workspace/debrief/DESIGN.md`, `~/workspace/debrief/WORLD.md`.
Everything in this spec that is not canon is marked **PROPOSED**; the
proposals list (§8) gathers them. The mystery law from the story bible
binds this system absolutely: the compendium never resolves what canon
leaves unresolved, and it never spoils what the player has not found.

---

## 1. What the compendium is, in fiction

The compendium is **the operator's Archive** — the Agency's case file
against the Administration, kept in the debrief room and carried in the
operator's head between ops.

- **In the debrief room**, it is physical: the wall map marked with red
  op-circles, the handler's briefcase paperwork, the folders on the
  desk, the service record itself. When the operator returns from an
  op, the handler reads the op back, the file is written, and the
  Archive grows — new entries appear, known entries gain paragraphs.
  This is the moment updates LAND.
- **In the menu screen**, it is the same Archive, mirrored: the same
  entries, the same state, readable at any time between ops. The menu
  does not hold a different or newer copy; it shows the Archive as it
  stands after the last debrief. (PROPOSED presentation: the menu's
  compendium reads as a file drawer — tabs by category, entries as
  folder pages, redaction bars where knowledge stops. The menu never
  shows an entry the Archive has not yet filed.)

One Archive, two doors. The debrief room is where it is written; the
menu is where it is read. Nothing the player has not earned by playing
is ever in it.

---

## 2. Categories and the entry schema

### Categories (PROPOSED set, fitted to canon)

Canon suggests these six; the seventh is the record itself:

1. **People** — the operator, the Handler, the Wanderer, the Hound and
   his handler, the Weathervane, the Neighbour, the dockhand, the
   queue-keeper, and citizens and soldiers as types (the conscript
   inspector, the checkpoint queue) rather than named individuals —
   because in this world most people are filed by role, not name.
2. **Places** — Greyport's districts (dockyards, old town,
   Administration quarter, market streets and arcade, temple ground,
   tenements, financial district), the checkpoint crossings, the
   debrief room itself, and op sites as they are filed.
3. **Factions** — the Administration, the Vigilance Party, the exile
   NSA, Control (as protocol — see the redaction rule below), the
   foreign stations. The Resistance has NO entry until canon gives it
   one (see §7): it is not filed, because it is not known.
4. **Machines** — Type 1 walker (by mark, as the Archive assembles
   the marks timeline), Type 2 (thin by law), the mecha's components
   as sub-entries (sensor, locomotion, cannon, power-coolant), and
   the operator's kit as filed objects (USP, grapple launcher, the
   sword — the sword's entry is marked UNRESOLVED by law).
5. **Events** — ops as filed: each story op and each systemic op type
   gains an entry when it is debriefed, written in the handler's
   voice — what happened, what the blood said, what grade the file
   earned.
6. **Field Notes** — doctrine the player has LEARNED, not lore the
   game tells: stimulus/response tables for the mecha, checkpoint
   rotas, the hour the lamp battery dies, scent discipline learned
   the hard way. Field Notes are the only category written in the
   second person ("you learned"), because they are the player's own
   tradecraft.
7. **The Service Record** — the career's spine: grades earned, heat,
   commendations, black marks, agents lost, and the nightjar count
   in the margin (see §4: the count, never an entry).

### Entry schema (PROPOSED)

Every entry is one record with these fields:

- `id` — stable, never reused, never renamed (e.g. `person.handler`,
  `machine.type1.sensor`, `place.dockyards`).
- `category` — one of the seven above.
- `status` — LOCKED, RUMOR, OBSERVED, RECORDED, or ARCHIVED (see §3).
- `title` — shown ONLY when status is not LOCKED. A locked entry has
  no title anywhere in the game — not in the menu, not in a count, not
  in a tooltip. (See the no-spoiler rule, §4.)
- `paragraphs[]` — the body, in filing order. Each paragraph carries
  its own `provenance` (which op filed it, and which grade that op
  earned) and its own `certainty`: KNOWN, REPORTED (a single source,
  unconfirmed), or REDACTED (the Archive knows something is here and
  will not say what — rendered as a black bar, never as a hint).
- `unknowns[]` — the questions the entry itself records ("What is
  inside the Type 2?" is never written as a question in the Type 2
  entry — see the mystery rule below). Unknowns are only the
  ordinary kind: "Inspector's name not yet filed." "Mark timeline
  incomplete."
- `crosslinks[]` — other entry ids this entry references, shown only
  when the linked entry is itself unlocked.

**The growth rule (LAW):** an entry gains paragraphs as the operator
learns more; it NEVER rewrites what was already recorded. An early,
wrong paragraph stays, marked by the later paragraph that corrects
it ("Later filing corrects: the frame's eye was not blinded; its
audio compensation was misread."). The Archive is a record, not a
wiki — its errors are part of the story, and the debrief grades what
you knew when you knew it.

**The mystery rule (LAW, binding):** no entry — People, Machines,
Events, or Field Notes — ever resolves the six mysteries of
STORY-BIBLE.md §11 (what is inside the Type 2; whether the Handler
is Control; the Wanderer's motive; what grows in Mister Ed's pod;
the sword's provenance; why the nightjars are there). For these, the
compendium's behavior is fixed:
- The Type 2 entry records only what is observed (white plate, charge
  lines, scream, silenceable voice apparatus) and marks its programme
  page "partly unfiled" — it never speculates about a passenger.
- The Handler entry records the man across the desk and notes the
  Control question exactly once, as REDACTED, never answered.
- The Wanderer entry is one page — her description, marked
  UNRESOLVED — and it never grows beyond description and doctrine.
- The sword entry records its systems (silent, arm's length, coolant
  line from above) and marks provenance UNRESOLVED.
- Mister Ed has no confirmed entry (see §3, Fable tier).
- The nightjars have NO entry at all (see §4). The Archive holds no
  page, and the compendium honors that by having no page.

---

## 3. Entry lifecycle — how an entry is born and grows

Five statuses, in order. Movement is one-way except where noted:

1. **LOCKED** — nothing is known. The menu shows only a locked
   marker (a closed folder, no title, no category count that reveals
   it — see §4). The player cannot tell how many locked entries
   exist, or in which category. A locked entry is not a tease; it is
   an absence.
2. **RUMOR** — the operator has heard the thing exists (a citizen
   mentions the temple, a soldier mentions the Hound's section, the
   street talks a name for the white machine). Title appears; body
   is one paragraph, certainty REPORTED, marked "street talk" or
   "single source." Rumor entries never contain mechanics.
3. **OBSERVED** — the operator has met, visited, or been affected by
   the thing in an op (a person met, a place visited, a machine
   component damaged, a checkpoint crossed). Title and first KNOWN
   paragraphs land. Mechanics the player has personally witnessed
   may appear (the Hound's posture states, once seen).
4. **RECORDED** — the debrief has filed it with grade weight behind
   it (a second op's confirmation, a component graded by the visor,
   a grade earned on an op about it). Paragraphs accumulate; Field
   Notes crosslink in.
5. **ARCHIVED** — the entry is complete as far as the Archive will
   ever take it (the marks timeline will never be ARCHIVED — it is
   complete only when the campaign's spine closes; district entries
   archive when the career has walked the whole map). ARCHIVED is a
   state of the file, not a reward, and it never unlocks power.

**Fable tier (PROPOSED, for Mister Ed only).** The Keeper's Room
fable is the one entry that can never pass RUMOR: the Archive's page
on it is the thinnest in the building, and the room is never found in
play (canon: never met, never fought; if found, the chair is warm
and he is not in it). PROPOSED unlock event: a citizen mentions the
keeper in systemic city chatter (a rare, unmarked line). The entry
then reads, entire: "A story the city tells about itself. A room on
no district plan, a man keeping a machine. UNCONFIRMED." It never
gains another paragraph. If Chase rules the fable should have no
entry at all, it is deleted, not expanded.

---

## 4. Auto-update rules — exactly what unlocks what, and when

**When updates land (LAW).** Updates are computed during the op but
LAND only at the debrief: on return to the debrief room, the handler
reads the op back, and the compendium grows in front of the player —
new folders appearing, known entries gaining paragraphs, the grade
stamped on the op's Event entry. The menu screen shows the Archive's
current state at any time between ops; it never previews an op's
pending updates. There is no mid-op compendium growth and no
mid-op spoiler: what you have not yet been debriefed on is not yet
filed.

**Event → entry mapping (PROPOSED, exhaustive for v1).**

| Gameplay event | Compendium effect (lands at debrief) |
|---|---|
| Person met in an op (Handler at the desk is automatic; Wanderer, Hound's handler, a named inspector) | That person's entry: LOCKED → OBSERVED (or RUMOR → OBSERVED), first KNOWN paragraph from the encounter |
| Person merely heard about (citizen mentions the temple, soldier mentions Tracking Section) | That entry: LOCKED → RUMOR, one REPORTED paragraph |
| Place visited (district entered, checkpoint crossed, temple ground stood on, debrief room returned to) | That place's entry: → OBSERVED; revisits add paragraphs (rota learned, lamp hour learned) |
| Checkpoint crossed clean / detained / forced | Place entry gains a paragraph; Event entry for the op records the crossing and its price; a clean pass adds a Field Note (the rota, the inspector, the lamp hour) |
| Machine component damaged (visor grades SENSOR, LOCOMOTION, CANNON, POWER-COOLANT on a Type 1) | That component's sub-entry: → OBSERVED on first grade, → RECORDED on confirmation; the Type 1 entry gains the stimulus/response paragraph the damage taught |
| Mecha blinded / hobbled / soft-killed / destroyed | Type 1 entry gains paragraphs; a destroyed unit's entry gains its final paragraph and its weld-plate history closes; alert-budget consequence noted in the Event entry |
| Type 2 encountered (charge survived, voice apparatus silenced, rooftop refuge used) | Type 2 entry: → OBSERVED with observed-only paragraphs (charge lines, scream as stimulus, silenceable voice). NEVER a paragraph about what is inside her. Street names THE BRIDE / MOTHER appear only as RUMOR paragraphs ("the street calls her —"), never as the entry's title |
| Grade earned (Ghost, Surgeon, Butcher, Ghost-That-Never-Was) | The op's Event entry is stamped with the grade; the Service Record gains the grade line; grades that complete a skill-tree branch add a Field Note (the branch's doctrine, as learned) |
| Skill purchased at the desk (grades as currency) | Service Record entry gains the purchase; the relevant Field Note gains a paragraph (what the skill is, in the visor's language) |
| Nightjar heard (churring at night, no source marker) | **NO compendium entry.** The nightjar count in the Service Record's margin increments by one. Nothing else happens. No entry unlocks, no paragraph lands, no title appears |
| Nightjar found (bird located by following the sound quietly) | Margin count increments; still NO entry. The count is the whole of it — counted, never explained, never cashed in for power |
| Witness event (citizen sees a body / drawn weapon / wall climb and reaches a phone) | The op's Event entry records it; the Witness Economy gains a Field Note paragraph (who saw, how far the phone was) — the second stealth clock, filed after the fact |
| Neighbour description entered (player described) | Neighbour entry → RECORDED (if not already); the op's Event entry records the description's price (checkpoints read harder; Wanderer cannot appear) |
| Wanderer aid received (tended while alone, time paid) | Wanderer entry gains one paragraph — description and doctrine only, never motive. The handler's one line ("The record will omit it. This once.") is filed verbatim, marked REPORTED |
| Temple budget spent (cover used; temple watched, then closed) | Temple Ground entry gains the paragraph; the Wanderer's entry notes the ground's state change, never why the Regime tolerates it beyond the canon pressure-valve line |
| Agent burned / killed; successor inherits | Service Record gains the closing paragraph (grade, manner, street where the body fell); the successor's first debrief files the inheritance paragraph ("You inherit the city, the Archive, and the file of —"). The compendium PERSISTS across agents (see §6) |
| Kit item first used (USP in every op is automatic; a rack weapon first fired; the sword first drawn) | That kit entry: → OBSERVED with its systems paragraph (noise price, stimulus ladder, hands committed). The sword's entry marks provenance UNRESOLVED and never revisits it |

**The no-spoiler rule (LAW).** Locked entries show NOTHING but a
locked marker — never a title, never a category count, never a
silhouette that gives the mystery away. The menu's category tabs show
only unlocked entries; there is no "12 locked entries" counter, no
completion percentage, no "you have not found the —" hint. The
compendium is a record of what happened, not a checklist of what
hasn't. A player who has never met the Type 2 must not learn from
the menu that a white machine exists.

---

## 5. Grading tie-in — the career's story as told by its own record

- **Grades are currency and text.** Grades (Ghost, Surgeon, Butcher,
  Ghost-That-Never-Was) pay the RPG layer's progression at the desk
  between ops, never in a field pause menu — and each grade is also a
  paragraph in the op's Event entry, in the handler's voice. The
  grade is not a score screen; it is a sentence in the file.
- **The service record is the save file, and the compendium's
  spine.** Clearances, reputation with the city's factions, heat,
  commendations, black marks — every line is both a system value and
  a compendium paragraph. What Control trusts you with next is
  decided by what the record says, and the record is readable in the
  menu at any time.
- **Death and inheritance (LAW, from DESIGN.md §5).** The operator's
  death ends the career, not the game: the file closes, the city
  keeps the body where it fell, and the next agent — who inherits the
  same persistent Greyport — can be sent to the same street and find
  what the mecha left. The compendium persists across agents,
  because the Archive does: the successor inherits every entry, every
  paragraph, every Field Note. What does NOT carry is the dead man's
  OPERATOR-layer skill (marksmanship, grappling, endurance — the
  man's own craft dies with him); the SERVICE layer (visor depth,
  forgery quality, district knowledge, handlers' tradecraft) persists
  with the Archive. The compendium records this honestly: the
  successor's Service Record opens with the inheritance paragraph,
  and the dead agent's Event entries stay filed under his name. The
  Archive outlives its authors — that is the point of an archive.
- **Full-archive runs.** DESIGN.md §6 names the full-archive run
  (every secret in the level filed) as the completion that matters.
  The compendium is how the player sees it: a level's entries, filed
  completely, with no locked markers left in that level's set — but
  the game never tells the player a level HAS unfiled entries (see
  the no-spoiler rule). The full archive is discovered by having
  filed it, not by being shown a gap.

---

## 6. What the compendium needs from the engine

The compendium is UI/text + persistence work. It rides the engine's
UI/text subsystem and world-persistence work (the Archive as a file),
sequenced AFTER the engine graphics phase in the outside-in order
(engine → city → weathers → lighting → …). It needs no new renderer
work: it is text, redaction bars, and folder pages over data the game
already records.

**Per-op record (PROPOSED schema — the data the game must record for
the compendium to work).** Written at op close, read at debrief:

- `op_id`, `district`, `hour`, `weather`, `kit_carried[]` (with
  configuration: street/assault), `objectives[]` (each: done/failed/
  unattempted), `crossings[]` (checkpoint id, result, inspector state),
  `witness_events[]` (who saw, what, phone distance, outcome),
  `descriptions[]` (Neighbour descriptions entered), `bodies[]` (left
  where, found/unfound, moved/hidden), `blood[]` (pools, trails —
  the debrief reads them), `mecha_contacts[]` (unit id, components
  graded, damage states, outcome: avoided/blinded/hobbled/soft-killed/
  destroyed), `type2_contacts[]` (observed facts only — charge lines,
  voice silenced, refuge used; never speculation), `scent_events[]`
  (trail laid, washed, broken — graded after the fact, never shown
  live), `wanderer_events[]` (aid received, time paid, temple state),
  `nightjars_heard`, `nightjars_found` (counts only), `grade`,
  `time_seconds`, `traces_left[]` (gear left in a claw, injury carried).

**Persistence (PROPOSED).** The Archive is literally a file (canon:
world persistence is "the Archive is literally a file"). One
append-only journal per career plus one Archive file per city:
- The journal appends one op record at op close; it is never edited.
- The Archive file holds the compendium state (entries, paragraphs,
  statuses, Service Record) and is rewritten only at debrief, from
  the journal — so a crash mid-op loses nothing filed, and a crash
  mid-debrief replays from the journal.
- Across agents, the Archive file persists; the journal starts a new
  career section under the successor's name, with the inheritance
  paragraph as its first line.
- Determinism law holds: the same op record always files the same
  paragraphs. The compendium is a pure function of the journal.

**UI/text needs (PROPOSED).** A text surface in the debrief room
(the desk, the folders) and in the menu (the file drawer); type set
in the room's own language — paper, stamps, redaction bars, the
handler's voice in Event entries. No HUD markers, no icons for
states: the compendium is read, not glanced at. It needs the UI/text
subsystem's text layout and the persistence file format, and nothing
else from the renderer.

**Sequencing (PROPOSED).** (1) Engine graphics phase lands first
(the compendium is text; it must not compete with the renderer
build). (2) UI/text subsystem + Archive file format land together.
(3) The compendium's v1 ships with People, Places, Factions,
Machines, Events, and the Service Record; Field Notes arrive with
the skill-tree milestone (their paragraphs are the tree's doctrine,
learned). (4) The Fable tier (Mister Ed) ships only if Chase rules
it an entry at all (§8, item 4).

---

## 7. Worked examples — what growth looks like

**Example 1: the soldier entry, across three ops.** After op 1 (a
checkpoint crossed, papers good), `person.soldier.conscript` is
OBSERVED: "A conscript with a quota and a mood. Raised left palm
means HALT." After op 2 (a detention in rain), it gains: "Rain
degrades him. Cold degrades him. Fear of the mecha degrades him
most — he yielded out of the patrol frame's arcs without being
told." After op 3 (a body found by his patrol), it gains: "A patrol
that finds a body changes state — alarm, search, a report the
Archive hears." Nothing was rewritten; the early paragraph's
simplicity stands next to the later paragraphs' precision, and the
difference between them is the player's education.

**Example 2: the Type 1 sensor sub-entry.** First visor grade on a
sensor: OBSERVED — "SENSOR graded. The eye sweeps on a fixed arc;
servo noise precedes the sweep." Second op, blinded: RECORDED —
"Blinded, it leans on audio in fixed, readable ways. Last op's
wound is this op's exploit." The entry never contains a health
number, because the machine has no health bar — only states, and
states the player has caused.

**Example 3: the nightjar margin.** Op 4, night, dockyards: the
player hears churring between the cranes, follows it quietly, finds
the bird on a crane arm. At debrief, nothing unlocks. The Service
Record's margin reads: "Nightjars: 1." No entry, no paragraph, no
title, no explanation. The count is the whole of it, and the handler
does not remark on it. That silence is the design.

---

## 8. Proposals needing Chase's ruling

1. The category set (§2): six categories plus the Service Record —
   fitted to canon, but the set itself is proposed.
2. The entry schema (§2): field names, the five statuses, and the
   append-only growth rule — proposed as the system's law.
3. The event → entry mapping table (§4): exhaustive for v1, but every
   row is a proposed rule, not canon.
4. The Fable tier for Mister Ed (§3): whether the keeper's fable
   gets a RUMOR entry at all, or stays entirely unfiled. If ruled
   out, it is deleted, not expanded.
5. The per-op record schema and the Archive file format (§6):
   proposed data shapes, needing the persistence milestone's ruling.
6. The sequencing (§6): compendium v1 after the engine graphics
   phase, Field Notes with the skill-tree milestone — proposed order.
7. The Resistance having NO compendium entry (§2): follows from
   canon's silence, but the decision to keep it unfiled (rather than
   a RUMOR entry) needs Chase's word.
8. Menu presentation as a file drawer with tabs and redaction bars
   (§1): proposed dressing; the law (same entries, same state, both
   places) is the requirement, the dressing is not.
