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
- Exercise layout: library crate, learner edits `src/lib.rs`; tests in `src/tests.rs` via `#[cfg(test)] mod tests;`; command `cargo test --offline` (no `--quiet`: the player reads the per-test lines); edition 2024; std only.
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
- impeccable sequence finished: init, shape, craft, critique (26/40), audit (14/20), layout, typeset, colorize,
  clarify, animate, delight, onboard, distill, adapt, harden, overdrive (Lukasz chose: compiler errors marked in
  the editor), quieter/bolder (evaluated, not needed), polish, extract, document. `DESIGN.md` and
  `.impeccable/design.json` written; final `impeccable detect` is clean apart from the deliberate grid background.
  Every step is in `design-log.md`.
- Chapter 3 done (Common Programming Concepts): 5 lessons + 12-question test; exercises ch03-safe-arithmetic,
  ch03-tuples-arrays, ch03-functions, ch03-loops (all verified); 10 playgrounds verified.
- New check: `.build/tools/verify_codeblocks.py . chNN` compiles every runnable `code` block and compares its shown
  output. Add it to the per-chapter loop. (The ch02 `cargo run -- 4 8 15` block needs arguments and is the one
  expected difference.)
- Next: chapter 4 (Ownership), following `.build/plan.md`.
- Chapter 4 done (Ownership, Borrowing and Slices): 3 lessons + 12-question test; exercises ch04-ownership,
  ch04-borrowing, ch04-slices (verified); 4 playgrounds verified; compiler-error examples match rustc exactly.
- Tooling: `.build/tools/fix_compiler_output.py . chNN` rewrites "Compiler output" blocks to match the real rustc.
  **Always `rm -rf .cargo-target` before `verify_exercises.py`**: a reused target dir can serve a stale build, and
  then every starter "passes".
- Chapter 5 done (Structs): 3 lessons + 10-question test; exercises ch05-structs, ch05-bank-account (verified);
  3 playgrounds verified. `verify_codeblocks.py` compares stderr for blocks labelled "Standard error" (dbg!).
- Chapter 6 done (Enums and Pattern Matching): 3 lessons + 10-question test; exercises ch06-commands (stack
  machine), ch06-config (let...else parsing), both verified; 3 playgrounds verified. A playground whose starter
  already prints the target output gets no expected_output (the verifier rejects that), only a task.
- Chapter 7 done (Packages, Crates and Modules): 3 lessons + 10-question test; exercise ch07-library (verified);
  2 playgrounds. `fix_compiler_output.py` compiles `file: src/lib.rs` blocks as a library.
- Chapter 8 done (Common Collections and Text): vectors, strings/UTF-8, hash maps + collection guide, 10-question
  test; exercises ch08-vectors, ch08-text, ch08-maps (verified); 3 playgrounds verified. Examples that print maps use
  BTreeMap so their output is deterministic.
- Chapter 9 done (Error Handling): panic!/backtraces, Result + combinators + ? + custom error enums + Box<dyn Error>,
  when to panic + validation types, 10-question test; exercises ch09-errors, ch09-validated (verified); 3 playgrounds
  verified. `verify_codeblocks.py` lets a `fn main() -> Result` block exit non-zero (its stdout must still match).
- Chapter 10 done (Generics, Traits and Lifetimes): generics + const generics + monomorphization, traits (defaults,
  orphan rule, bounds, where, impl Trait, blanket impls, associated types), lifetimes (elision rules, structs,
  'static), standard traits (derives, Display, From/TryFrom, AsRef, Drop, operator overloading); 12-question test;
  exercises ch10-generics, ch10-traits, ch10-lifetimes, ch10-std-traits (verified); 3 playgrounds verified.
- Chapter 11 done (Writing Automated Tests): writing tests (asserts, should_panic, Result tests), running them
  (filters, ignore, threads, output), organizing (unit/private, tests/, common/mod.rs, doc tests); 10-question test;
  exercises ch11-tested (tests as a spec) and ch11-unit (private helpers under test), verified; 1 playground.
- Chapter 12 done (minigrep project): args and files, refactor to Config::build/run/lib.rs, TDD search, IGNORE_CASE +
  eprintln! + writing to any Write; 10-question test; exercise ch12-minigrep (whole tested library), verified;
  3 playgrounds (args are simulated, since the console cannot pass real arguments).
- Chapter 13 done (Closures and Iterators): closures (capture modes, move, Fn traits, taking/returning closures),
  iterators (trait, iter/iter_mut/into_iter, consumers, adapters, laziness stepper, own Iterator), minigrep with
  iterators + zero-cost; 12-question test; exercises ch13-closures, ch13-iterators (verified); 4 playgrounds.
  Compiler *warnings* go in a separate `lang: text` block (the checks compile with warnings off).
- Chapter 14 done (Cargo and Crates.io): profiles, doc comments + sections + //!, pub use re-exports, publishing and
  yanking, SemVer/Cargo.lock, workspaces, cargo install, custom commands, fmt/clippy/fix; 8-question test;
  exercise ch14-api (re-exports + doc example), verified; 1 playground.
- Chapter 15 done (Smart Pointers): pointers + Box + recursive types, Deref/coercion/Drop, Rc + RefCell + Cell + mock
  objects, cycles + Weak + trees; 12-question test; exercises ch15-box, ch15-deref, ch15-rc, ch15-tree (verified);
  5 playgrounds. `verify_codeblocks.py` now runs with RUST_BACKTRACE=0, strips the thread id newer rustc prints in
  panic messages, and accepts a non-zero exit for "Standard error" blocks that show a panic.
