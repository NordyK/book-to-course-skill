# Rust course: work log

The durable record of what has been built, so a new session can pick up without asking again.
Read this first, then run `python3 ~/.claude/skills/book-to-course/scripts/course_state.py courses/rust-course status`.

## How to resume (any session)

1. Install the skills: copy `book-to-course/` from this repo and `.claude/skills/impeccable/` from NordyK/impeccable into `~/.claude/skills/`.
2. `.build/source/` is gitignored (extracted book text). Rebuild it from the PDFs in the project files:
   `courses/rust-course/.build/tools/rebuild_sources.sh /mnt/project-files`
3. `course_state.py courses/rust-course status` shows the next chapter; `.build/plan.md` lists the lessons it must contain and their source sections.
4. Per chapter: read the source files listed in `.build/source/chapters/<ch>.md`, write `content/<ch>/…`, add exercises under `exercises/<ch>-<name>/`,
   then `build_course.py`, `verify_exercises.py`, `course_state.py … set <ch> done`, add a line to the log below, commit, push.
5. The player in this course is custom (designed with impeccable, see `DESIGN.md` / `PRODUCT.md` / `design-log.md`).
   **Never run `init_course.py --update` on this course**: it would overwrite the custom player with the default template.

## Decisions (keep consistent)

- Course language English; code and identifiers English. Spine = TRPL 2024 edition; older TRPL editions are duplicates (see plan.md).
- Exercise layout: library crate, learner edits `src/lib.rs`; tests in `src/tests.rs` via `#[cfg(test)] mod tests;`; command `cargo test --offline --quiet`; edition 2024; std only.
- Block ids: `<ch>-quiz-<topic>`, `<ch>-ex-<topic>`, `<ch>-cards-<topic>`, `<ch>-play-<topic>`; tests `<ch>-test`.
- `playground` block = editable, runnable Rust snippet (local rustc via serve.py, else play.rust-lang.org).

## Log

- 2026-10-23: Skills installed. Six PDFs extracted; three are editions of TRPL (2018, 2021, 2024) → 2024 is the spine, the other two marked duplicates. Merged 25-chapter plan written (`.build/plan.md`). Course scaffold created.

## 2026-10-23 (continued)

- Chapter 1 done and the custom player is built (reading column + Verification Console, mobile bottom sheet,
  progress v2 with activity log, study time, streak, export/import). serve.py gained `/api/run` and file-writing
  `/api/run-tests`. Draft PR: NordyK/book-to-course-skill#2.
- Authoring pipeline: `.build/tools/yaml2json.py` now writes Rust exercise folders from the `rust:` key of an
  `exercise` block, and moves playground `solution_code` / `starter_ok` into `.build/verify/playgrounds.json`.
  `.build/tools/verify_playgrounds.py` compiles every playground (starter must run unless `starter_ok` says it should
  panic or fail to compile) and checks each solution against `expected_output`.
- Per-chapter loop: write YAML → `yaml2json.py` → `build_course.py .` → `CARGO_TARGET_DIR=$PWD/.cargo-target
  verify_exercises.py .` → `verify_playgrounds.py` → `course_state.py . set chNN done` → WORKLOG entry → commit, push.
- Chapter 2 done (Guessing Game): 3 lessons + test; exercises ch02-compare, ch02-parse-guess, ch02-play-game
  (all verified: fail on starter, pass on solution); 6 playgrounds verified.
- impeccable: critique running next; log of every command goes in `design-log.md`.
