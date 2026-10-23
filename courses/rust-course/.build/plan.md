# Rust course: build plan and coverage map

Course: **Rust: From First Program to Systems Programming** (English, beginner-first).
Built with the `book-to-course` skill from five uploaded books. This file is the coverage map: every
lesson lists the sections it draws from. Keep it in sync when lessons change.

## Sources and deduplication

| Key | Book | Role in the course |
|---|---|---|
| TRPL | *The Rust Programming Language*, web print, Rust 1.97 / 2024 edition (`The Rust Programming Language.pdf`) | **Spine.** Chapter order and terminology follow it. |
| TRPL-2021 | Same book, Rust 1.65 / 2021 edition (`rust_book.pdf`) | Duplicate of TRPL, older. Not used as a separate source; consulted only if the 2024 text is garbled. |
| TRPL-2018 | Same book, No Starch 1st edition, Rust 1.21 (`1_The.Rust.Programming.Language.pdf`) | Duplicate of TRPL, oldest. Not used. Where it differs (e.g. old `try!` macro, pre-2018 module paths) the course teaches the current form only. |
| PR | *Programming Rust*, 2nd ed. (Blandy, Orendorff, Tindall) | Depth: fundamental types, expressions, references, utility traits, operator overloading, strings, I/O, async, macros, unsafe, FFI. |
| RiA | *Rust in Action* (McNamara, MEAP v15) | Systems projects: data representation, memory, files, networking, time, processes, kernel, signals. |
| CR | *Comprehensive Rust* (Google) | Short exercises and drills, memory model review, idiomatic Rust. Android/Chromium/bare-metal days are out of scope except a short bare-metal mention in ch24. |

Dedup rule: one concept is taught once, in the chapter where TRPL introduces it. PR/RiA/CR material on the
same concept is folded into that lesson as a deeper example, a pitfall, or an exercise, never as a second lesson.

## Learning arc

1. Part I (ch01–ch03): tooling, a first real program, the everyday language.
2. Part II (ch04–ch09): what makes Rust Rust: ownership, data modelling with structs and enums, modules, collections, errors.
3. Part III (ch10–ch14): abstraction and craft: generics, traits, lifetimes, tests, a CLI project, closures/iterators, Cargo.
4. Part IV (ch15–ch20): smart pointers, concurrency, async, trait objects, patterns, advanced features.
5. Part V (ch21–ch25): systems programming from Rust in Action, then the TRPL final project.

Running example: none across the whole course; each chapter keeps its book's own example
(guessing game, rectangles, IP addresses, minigrep, web server, ActionKV...).

Exercise toolchain: Rust (`cargo test --offline`, edition 2024, std only). Each exercise is a library crate:
`src/lib.rs` (learner edits) + `src/tests.rs` (tests, pulled in by `#[cfg(test)] mod tests;`).
In the page the learner edits `src/lib.rs` in an in-browser editor; tests run locally through `serve.py`
(rustc/cargo on the learner's machine) or, with no local server, on play.rust-lang.org.

## Lessons per chapter (coverage map)

### ch01 Getting Started with Rust
- 01-why-rust: TRPL Introduction; PR ch1; RiA 1.1–1.10 (where Rust is used, safety/productivity/control goals, downsides). CR welcome.
- 02-install-hello: TRPL 1.1 Installation, 1.2 Hello, World! (rustc, anatomy of main, println! macro, compile vs run).
- 03-cargo: TRPL 1.3 Hello, Cargo! (cargo new/build/run/check, Cargo.toml, release builds); CR Using Cargo; RiA 1.4 phrase book bits.
- 99-test.

### ch02 Your First Program: a Guessing Game
- 01-setup-input: TRPL 2 setup, processing a guess (use, std::io, let/mut, String::new, read_line, Result/expect, println! placeholders).
- 02-random-compare: TRPL 2 secret number (crates, Cargo.lock, rand), comparing (match, Ordering, parse, shadowing).
- 03-loop-quit: TRPL 2 looping, quitting, invalid input handling; PR ch2 tour (gcd, unit tests, command-line args) as extra examples.
- 99-test.

### ch03 Common Programming Concepts
- 01-variables: TRPL 3.1 (mutability, constants, shadowing); CR variables.
- 02-scalar-types: TRPL 3.2 scalars (integers, overflow, floats, numeric ops, bool, char); PR ch3 integer/float detail; RiA 2 numbers.
- 03-compound-types: TRPL 3.2 tuples, arrays; PR ch3 arrays/tuples.
- 04-functions: TRPL 3.3 functions, parameters, statements vs expressions, return values; PR ch6 expressions; CR blocks/scopes.
- 05-control-flow: TRPL 3.5 if, loop/while/for, loop labels, values from loops; CR loops; RiA 2 control flow.
- (TRPL 3.4 comments folded into 04-functions.)
- 99-test.

### ch04 Ownership, Borrowing and Slices
- 01-stack-heap-ownership: TRPL 4.1 (stack/heap, ownership rules, String, move, clone, Copy, functions); CR memory review; PR ch4 moves.
- 02-references: TRPL 4.2 (borrowing, &mut, rules, dangling refs); PR ch5 references; RiA 4.
- 03-slices: TRPL 4.3 (string slices, first_word, other slices).
- 99-test.

### ch05 Structs
- 01-define-instantiate: TRPL 5.1 (fields, init shorthand, update syntax, tuple/unit structs, ownership of fields); PR ch9; CR structs.
- 02-example-program: TRPL 5.2 (rectangles, refactor with tuples then struct, Debug, dbg!).
- 03-methods: TRPL 5.3 (methods, &self, associated functions, multiple impl blocks); PR ch9 impl; RiA 3 (File struct, impl).
- 99-test.

### ch06 Enums and Pattern Matching
- 01-enums-option: TRPL 6.1 (IpAddr, data in variants, Option); PR ch10 enums.
- 02-match: TRPL 6.2 (match, binding patterns, Option<T>, exhaustiveness, catch-all/_ ).
- 03-if-let: TRPL 6.3 (if let, let...else); CR pattern drills.
- 99-test.

### ch07 Packages, Crates and Modules
- 01-packages-crates: TRPL 7.1.
- 02-modules-paths: TRPL 7.2–7.3 (mod, privacy, paths, pub, super, structs/enums pub).
- 03-use-files: TRPL 7.4–7.5 (use, as, pub use, external packages, nested paths, glob, splitting into files); PR ch8.
- 99-test.

### ch08 Common Collections and Text
- 01-vectors: TRPL 8.1; PR ch16 Vec.
- 02-strings: TRPL 8.2; PR ch17 (UTF-8, chars, bytes, formatting).
- 03-hashmaps: TRPL 8.3; PR ch16 HashMap/BTreeMap/sets.
- 99-test.

### ch09 Error Handling
- 01-panic: TRPL 9.1.
- 02-result: TRPL 9.2 (match on Result, unwrap/expect, ?, propagating, Box<dyn Error> in main); PR ch7.
- 03-when-to-panic: TRPL 9.3 (validation types like Guess); CR error drills.
- 99-test.

### ch10 Generics, Traits and Lifetimes
- 01-generics: TRPL 10.1 (functions, structs, enums, methods, monomorphization).
- 02-traits: TRPL 10.2 (define/implement, defaults, trait bounds, impl Trait, blanket impls); PR ch11.
- 03-lifetimes: TRPL 10.3 (annotations, elision, 'static).
- 04-std-traits: PR ch12 operator overloading, ch13 utility traits (Clone, Copy, Drop, Default, From/Into, AsRef, Display).
- 99-test.

### ch11 Writing Automated Tests
- 01-writing-tests: TRPL 11.1 (#[test], assert!, assert_eq!, should_panic, Result in tests).
- 02-running-tests: TRPL 11.2 (threads, output, filtering, ignore).
- 03-organizing: TRPL 11.3 (unit vs integration tests, tests/ dir).
- 99-test.

### ch12 Project: a Command-Line Search Tool (minigrep)
- 01-args-files: TRPL 12.1–12.2.
- 02-refactor: TRPL 12.3 (Config, separation of concerns, lib crate).
- 03-tdd-search: TRPL 12.4.
- 04-env-stderr: TRPL 12.5–12.6; PR ch18 readers/writers.
- 99-test.

### ch13 Closures and Iterators
- 01-closures: TRPL 13.1 (capture, Fn traits, move); PR ch14.
- 02-iterators: TRPL 13.2 (Iterator trait, adapters, consumers); PR ch15.
- 03-improving-minigrep: TRPL 13.3–13.4 (performance: zero-cost).
- 99-test.

### ch14 Cargo and Crates.io
- 01-profiles-docs: TRPL 14.1–14.2.
- 02-workspaces-install: TRPL 14.3–14.5.
- 99-test.

### ch15 Smart Pointers
- 01-box: TRPL 15.1; RiA 6 pointers.
- 02-deref-drop: TRPL 15.2–15.3.
- 03-rc-refcell: TRPL 15.4–15.5.
- 04-cycles-weak: TRPL 15.6.
- 99-test.

### ch16 Fearless Concurrency
- 01-threads: TRPL 16.1; RiA 10.
- 02-channels: TRPL 16.2.
- 03-shared-state: TRPL 16.3 (Mutex, Arc); PR ch19.
- 04-send-sync: TRPL 16.4.
- 99-test.

### ch17 Async Programming
- 01-futures-syntax: TRPL 17.1.
- 02-concurrency-with-async: TRPL 17.2–17.3.
- 03-streams-traits: TRPL 17.4–17.6; PR ch20.
- 99-test.

### ch18 Trait Objects and OOP in Rust
- 01-oop-characteristics: TRPL 18.1.
- 02-trait-objects: TRPL 18.2.
- 03-state-pattern: TRPL 18.3.
- 99-test.

### ch19 Patterns in Depth
- 01-where-patterns: TRPL 19.1–19.2 (refutability).
- 02-pattern-syntax: TRPL 19.3.
- 99-test.

### ch20 Advanced Features
- 01-unsafe: TRPL 20.1; PR ch22; CR unsafe.
- 02-advanced-traits: TRPL 20.2.
- 03-advanced-types-fns: TRPL 20.3–20.4.
- 04-macros: TRPL 20.5; PR ch21.
- 99-test.

### ch21 Data in Depth (RiA ch5)
- 01-bit-patterns-integers, 02-floats, 03-fixed-point-rng, 04-cpu-emulator, 99-test.

### ch22 Files, Storage and I/O (RiA ch7, PR ch18)
- 01-files-paths, 02-hexdump-binary, 03-actionkv, 99-test.

### ch23 Networking and Time (RiA ch8–9)
- 01-http-tcp, 02-traits-errors-net, 03-time-clocks, 99-test.

### ch24 Going Low-Level (PR ch23, RiA ch11–12, CR bare metal)
- 01-ffi, 02-no-std-kernel, 03-signals, 99-test.

### ch25 Final Project: a Multithreaded Web Server (TRPL 21, CR idiomatic Rust)
- 01-single-threaded, 02-thread-pool, 03-graceful-shutdown, 99-test.

## Deliberately left out
- Comprehensive Rust Android and Chromium days (platform-specific build systems, not general Rust).
- TRPL appendices are folded into lessons where they matter (derivable traits in ch05/ch10, tools in ch14, editions in ch01).
