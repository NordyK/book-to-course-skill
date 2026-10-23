# Design log: impeccable commands, in order

The course player was designed with the impeccable skill, guided by `design-reference/DESIGN-1.md`. Each command
below was run in sequence against the player (`index.html`, `assets/app.js`, `assets/style.css`). Each entry
records what the command found, what changed, and any decision that departs from the skill's defaults.
Screenshots from each review are kept out of git, in `.impeccable/review/`.

## Deliberate departures (kept on purpose)

- **Grid backdrop with plus marks on the navy home band.** The detector flags `codex-grid-background` (advisory).
  DESIGN-1 requires this backdrop on navy fields, so it stays, but only on the home band and not behind the
  reading column.
- **Mono uppercase labels.** The craft floor bans eyebrow labels above headings. DESIGN-1 uses mono labels for
  operational status. Labels are kept for status and controls only (tags, console rows, tiles). No heading sits
  under a kicker.
- **Cormorant Garamond italic emphasis in display statements** (`italic-serif-display`). The surface brief pins
  Cormorant as the stand-in for DESIGN-1's Season serif, and the brief takes precedence. The italic is used for
  one emphasised phrase per statement.
- **`.cell.started` half fill** (flagged as `ai-color-palette`, "cyan gradient"). This is a false positive: it is
  a hard-stop diagonal state marker in the Sky Paper token, not a gradient.

## 1. init (done before the build)

Wrote `PRODUCT.md` (product-schema 1, platform web). Lukasz confirmed the audience as complete beginner through
ultra-advanced.

## 2. shape / concept-seed

`concept-seed --scope surface --mode operate` ran **degraded**, because the impeccable.style roll API is
unreachable from the build sandbox. Lukasz picked "Reading + live console". The brief is
`.impeccable/surfaces/index-html.md` (seed reference `seed:d5cecc516bc82015`). Operate mode, so the build is
code-led.

## 3. craft (new-work build)

- Built the player: top bar, chapter rail, a 68ch reading column, and a sticky 520px Verification Console.
- The home page has a navy band with a serif statement and a Resume card, a chapter × lesson ledger, a study
  log, and progress and glossary pages.
- On phones the console becomes a bottom sheet with a peek bar.
- Fonts are self-hosted OFL stand-ins.

## 4. critique: 26/40 (Acceptable)

Run as two isolated sub-agents: a design review and detector + browser evidence. The snapshot is in
`.impeccable/critique/`.

| Priority | Finding |
|---|---|
| P0 | Inline code and bold break the Key points grid and the quiz feedback |
| P1 | Resume points at the chapter test while lessons are unfinished |
| P1 | Compiler and panic output is shown raw |
| P1 | The console is a dead blue column on lessons with nothing to run |
| P2 | The chapter-test end state is unfinished |

Questions were not asked. Lukasz's standing instruction is to run every command in sequence and fix what they
find, so all findings are in scope.

## 5. audit: 14/20 (Good)

| # | Dimension | Score | Key finding |
|---|---|---|---|
| 1 | Accessibility | 3 | Editor gutter 2.8:1 and the console shortcut hint 3.9:1. Lessons jump from h1 to h3. The meter has no value. The test badge "BEST" is unreadable on mint. |
| 2 | Performance | 3 | No layout thrash and fonts are preloaded. The progress POST response is never read, so the request stays open in Chromium. The editor repaints the full highlight on every keystroke, which is fine at lesson sizes. |
| 3 | Theming | 3 | Tokens are used throughout, with a few hard-coded console colours (`#5d5a57`, `#071a3a`). In dark mode the navy band merges with the navy page. |
| 4 | Responsive | 2 | Phone targets are under 40px: ledger cells 22px, Copy and Try it 26px, radios 16px, stepper ticks 22×6. Tables scroll inside their frame but cannot be seen to scroll. The mobile bar hides the nav. |
| 5 | Implementation integrity | 3 | One real `side-tab` stripe (`.runnable.loaded`). The rest of the detector findings are false positives or departures that DESIGN-1 requires. |

There are no WCAG A blockers. The AA misses are both in the console (gutter and shortcut hint) and in the test
badge. All findings are mapped to the commands that follow.

## 6. layout

- The reading measure is `42rem`, not `68ch`. A `ch` measure scales with each element's own font size,
  so headings, code and body text ended up at different widths.
- Lessons with nothing to run use a single column (`.lesson-grid.solo`). They no longer show an empty console
  or a mobile peek bar.
- The Key points list wraps each item in a span, so inline code and bold stay in the text flow (critique P0).
- Phones get a Course / Progress / Glossary row at the top of the rail, and ledger cells grow to 30px.

## 7. typeset

- Headings are balanced and paragraphs use `text-wrap: pretty`. Block titles go through the inline renderer,
  so `code` in a title is set in Plex Mono.
- The lesson outline is h1 > h2 everywhere. It used to jump to h3 (audit, accessibility).

## 8. colorize

- The editor gutter is `#8f8a84` (4.6:1 on Carbon) and the shortcut hint is at .88 opacity. Both pass AA now.
- The test header tag is mint when passed and coral when failed, so "BEST" is readable.
- The `.runnable.loaded` side stripe is replaced by an outline and an "In console" tag. This was the one real
  detector finding.

## 9. clarify

- The console explains each result in one plain-language line before the raw output:
  - compile errors give the line, the message and a link to the rustc error-code page;
  - panics are marked as expected or unexpected;
  - tests show "N of M pass" and the left/right values that differ;
  - output mismatches are named.
- When a playground is meant to panic, the row says "Stopped by Rust, as intended" instead of showing a red failure.
- Resume skips a chapter test while lessons in that chapter are unfinished, and it names what is still to do
  (critique P1).
- The empty console says how to fill it. Check with nothing selected gives a nudge instead of a wrong mark.

## 10. animate

Motion thesis: only verification moves. Console rows tick in one by one as the compiler and tests report.
A stamp is set inside the console when a check passes. The generic scroll reveal was removed: it only delayed
reading. Everything is gated by a targeted `prefers-reduced-motion` block.

## 11. delight

Passing a chapter test opens a "Chapter complete" panel. It lists the chapter goals as "You can now:" and
links to the next chapter, or names it if it is not written yet (critique P2).

## 12. onboard

Until the first lesson is complete, the course page shows "How each lesson works": read, try it in the
console, then check yourself. It also has a runner status line ("Your own Rust runs the code" or the
playground fallback).

## 13. distill

- The navy band no longer repeats the bar's nav. When a resume card is present the band has no buttons;
  otherwise it has one link to the progress page.
- The `[hidden]` attribute now wins over `.btn` display, so Retry, Show answer and Try again appear only
  when they apply.

## 14. adapt

- Optional table columns (`.opt-col`) are hidden under 760px. Data tables otherwise scroll inside their frame.
- `pointer: coarse` hides the keyboard hint.

## 15. harden

- Local runs have timeouts through `AbortController`: 90 s for a program and 180 s for a test run.
  A hung rustc now reports an error instead of leaving the row spinning.
- `/api/run-tests` writes only the exercise's declared starter files, with a realpath check.
- The progress POST response is read, so Chromium closes the request (audit, performance).

## 16. overdrive: errors marked in the editor

Lukasz chose this direction on the decision card.

- rustc diagnostics are parsed for severity, code, message, file, line, column and caret span.
- Each one is drawn over the code as a wavy underline on the exact span, with a bar in the gutter:
  coral for errors and amber for warnings.
- The "line N" in the console summary is a button. It opens the sheet on phones, puts the caret on the
  error column and scrolls to it.
- Only diagnostics for the file in the current tab are drawn. Marks clear on the next keystroke or reload.

Checked in Chromium: the `ch02-play-mutable` playground underlines `count = 2` on line 3 (E0384), and the
jump puts the caret on line 3. Screenshots are in `.impeccable/review/v4/`.

## 17. quieter / bolder (evaluated, not run)

Neither applies. After distill the course page has one lit surface (the band) and lessons have one (the console),
which is the balance DESIGN-1 asks for. Making it louder would break the Lit Console Rule, and making it quieter
would lose the ledger character.

## 18. polish

- No critique snapshot was current, because `index.html` changed after the critique, so this was an independent pass.
- Fixed the drift the scan found: a 3px inline-code radius became the 4px control radius, 17px glossary terms
  became the 18px title size, and 11px flow-step numbers became the 12px label size.
- The console output and editor borders use Ledger Navy instead of translucent black.

## 19. extract

Values used across the console and editor became tokens on `:root`. They are always dark, so they have no dark
override: `--con-pass`, `--con-fail`, `--con-out`, `--con-out-fg`, `--con-out-dim`, `--con-idle`, `--con-offline`,
`--ed-err`, `--ed-warn`, `--code-gutter` and `--code-btn`. `--navy-hover` and `--warn-rule` were also added, and
`--warn-rule` has a dark value. No components were extracted: the player is one file of small DOM helpers, and each
block type is already a single function.

The exception is the two wavy-underline SVGs, which keep literal colours because CSS variables cannot reach into a
data URI.

## 20. document

`DESIGN.md` (with token frontmatter) and `.impeccable/design.json` were written in scan mode. The qualitative language
(the North Star "The Verification Console", colour names and rules) comes from Lukasz's DESIGN-1 guide instead of a
new interview, since that guide is his statement of the visual world. Once DESIGN.md existed, the detector checked
the CSS against it. Every finding was fixed except the deliberate grid background (see the departures above).
