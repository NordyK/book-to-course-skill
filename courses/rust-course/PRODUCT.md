# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Stack

Static HTML, CSS and JavaScript with no framework and no build step, plus the book-to-course `serve.py` helper (Python standard library) when the learner starts the course with `start.sh` / `start.bat`. Inferred from the book-to-course skill's player contract; the learner asked for "a frontend", not a specific framework. Must also work opened as a plain file and inside the skill's APK/AppImage wrappers.

## Users

Self-taught learners who want to learn Rust from zero. Confirmed by Lukasz (2026-10-23): the course must take a **complete beginner all the way to ultra-advanced** (unsafe, macros, async internals, FFI, kernels). Early chapters define every term; later chapters keep expert depth rather than simplifying. They study alone in 15–25 minute sessions on a laptop (sometimes a tablet), often in the evening, with nobody to ask when stuck.

## Product Purpose

One coherent Rust course that replaces reading five overlapping books (The Rust Programming Language 2024 edition, Programming Rust, Rust in Action, Comprehensive Rust; two older editions of TRPL are duplicates). The learner reads a short explanation, runs and edits real Rust in the page, and proves understanding with quizzes, exercises checked by real `cargo test` runs, and a chapter test. Success: the learner always knows where they are, what to do next, and can see what they have mastered.

## Positioning

A Codecademy-style learn-by-doing course whose content is drawn from the canonical Rust books with every topic taught once, where exercises are verified by the real Rust compiler and test runner (local toolchain or the official Rust Playground), not by string matching, and where progress is a file the learner owns.

## Operating Context

- Started with `start.sh` / `start.bat` (serve.py on 127.0.0.1): progress saved to `progress/progress.json`, code runs on the learner's own `rustc`/`cargo`, exercise edits are written to `exercises/<dir>/src/lib.rs`.
- Opened as a file: progress in browser storage, code runs on play.rust-lang.org (needs internet).
- Learners also edit exercise files in their own editor and run `cargo test` in a terminal.

## Capabilities and Constraints

- Lesson blocks from the book-to-course schema: text, callout, code, stepper, flow, reveal, table, svg, quiz, exercise, flashcards, summary, from_book, derivation, plus `playground` (editable runnable Rust, optionally checked against expected output).
- Every lesson has practice; every chapter has exercises and a test (pass mark 70%, retries allowed, best score kept).
- Progress tracked per lesson, quiz question, exercise, playground and flashcard; must persist across sessions and survive a reload.
- Content is generated from JSON (`course/data.js`); the player must render any course chapter count (25 planned).
- No external runtime dependencies beyond optional play.rust-lang.org; fonts and assets are self-hosted.

## Brand Commitments

The learner supplied `DESIGN 1.md` (a style reference called "Baselayer": editorial serif statements, compact sans UI, mono operational labels, navy and registry-blue system fields, square cards, 1px rules, no shadows) and asked that it be used as the design guide together with impeccable. Treated as a binding visual reference; the reference brand's name and marks are not used.

## Evidence on Hand

- Course content in `content/` (built from `.build/authoring/*.yaml`), coverage map in `.build/plan.md`.
- No testimonials, learner counts, ratings or completion statistics exist; none may be shown or invented.

## Product Principles

1. Doing beats reading: every idea is followed by something to run, change or answer.
2. Never lose the learner's work: progress, code edits and scores persist and are visible.
3. Real feedback: code is judged by the Rust compiler and tests, and errors are shown as Rust prints them.
4. Always a next step: from any screen the learner can resume exactly where they stopped.
5. Honest about the toolchain: say plainly when code cannot run (no server, no internet) and what to do instead.

## Accessibility & Inclusion

Keyboard-operable everything (quizzes, editor, steppers, flashcards), reduced-motion respected, WCAG AA contrast in light and dark themes. No product-specific standard was established beyond this.
