---
version: 1
slug: "index-html"
primary_target: "index.html"
related_targets: []
---

# Surface: course player (index.html)

Scope: the whole course app: home (progress ledger), lesson reader with console, chapter test, glossary, progress page.
Mode: Operate (lesson reading inside it follows Read rules for the column).
Audience and job: a learner going from zero to expert Rust in 15–25 minute sessions; read a short idea, run it, change it, prove it; always resume exactly where they stopped.
Proof/content: real lessons from content/, real Rust compiled by local rustc/cargo or play.rust-lang.org. No invented stats or testimonials.
Constraints: static HTML/CSS/JS, no build step, works from file:// and serve.py, self-hosted fonts, keyboard operable, reduced motion respected.
Pinned visual reference: design-reference/DESIGN-1.md (Ledger Navy / Registry Blue fields, serif statements, sans UI, mono operational labels, 0px cards, 4px controls, 1px rules, no shadows). Mono labels are used for status and controls, never as eyebrows above headings.

## Direction contract

THESIS: Learning Rust as verification. Every lesson is a calm reading column paired with a live Verification Console that loads the example in hand, compiles it, and reports each check as a status row. Refuses the category default of a grey docs page with a bolt-on code box.

OWN-WORLD: Paper White reading ground, Ledger Navy top bar and console frame, Registry Blue console field with white inset rows, Mint Paper metric tiles, Sky Paper highlights; Cormorant Garamond 500 statements, DM Sans UI and body, IBM Plex Mono for code, labels, buttons; 1px Rule Gray divisions, square cards, 4px controls, plus-sign registration marks on dark fields, no shadows.

STORY: The learner sees where they are in 25 chapters, reads one idea, presses Run, watches compile and test rows turn to pass, and moves on knowing it is saved.

FIRST VIEWPORT: Lesson screen at 1440: 64px navy bar (course title, chapter position, overall progress meter, save state); left 280px chapter rail with lesson status marks; centre reading column at 68ch with serif title and lead; right 520px sticky console: file tab, editor, Run button, output rows. Home: navy band with serif statement of where the learner is and a Resume control, then a progress ledger grid (25 chapters x lessons) on white.

FORM: Reading + live console (dealt lead, structure 6 of 7). seed:d5cecc516bc82015

Signature interaction: "Load into console": every runnable example and playground has a control that loads it into the sticky console; runs report as rows (compile, run, each test) that tick from pending to pass/fail.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance
