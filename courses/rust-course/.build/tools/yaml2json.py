#!/usr/bin/env python3
"""Lessons for this course are authored in YAML (block scalars keep Rust code readable) and converted
to the JSON files book-to-course's build_course.py reads.

  .build/authoring/course.yaml            -> content/course.json
  .build/authoring/<ch>/chapter.yaml      -> content/<ch>/chapter.json
  .build/authoring/<ch>/NN-name.yaml      -> content/<ch>/NN-name.json

Rust exercises are authored inline too: an `exercise` block with `exercise_dir: chNN-name` and a `rust:` map
({starter, tests, solution} = the bodies of src/lib.rs, src/tests.rs and _solution/src/lib.rs) is written out as
exercises/<dir>/ (Cargo.toml, src/lib.rs with `#[cfg(test)] mod tests;`, src/tests.rs, exercise.json, _solution/),
and the `rust` key is dropped from the JSON.

Playgrounds may carry `solution_code` (a version that meets the task) and `starter_ok: compile_error` / `panic`
(the starter is meant to fail). Both are moved to .build/verify/playgrounds.json for verify_playgrounds.py.

Usage: python3 .build/tools/yaml2json.py [COURSE_DIR] [CHAPTER ...]
Edit the YAML, never the generated JSON (it is overwritten). Needs PyYAML (pip install pyyaml).
"""
import json, os, sys
import yaml

CARGO = """[package]
name = "%s"
version = "0.1.0"
edition = "2024"

[dependencies]
"""


def write_rust_exercise(course, b, where):
    d, r = b.get("exercise_dir"), b.pop("rust")
    if not d or not all(k in r for k in ("starter", "tests", "solution")):
        sys.exit("%s: a rust exercise needs exercise_dir and rust.starter/tests/solution" % where)
    ex = os.path.join(course, "exercises", d)
    os.makedirs(os.path.join(ex, "src"), exist_ok=True)
    os.makedirs(os.path.join(ex, "_solution", "src"), exist_ok=True)
    marker = "\n#[cfg(test)]\nmod tests;\n"
    def lib(body):
        body = body.rstrip("\n") + "\n"
        return body if "mod tests;" in body else body + marker
    files = {"Cargo.toml": CARGO % d.replace("-", "_"), "src/lib.rs": lib(r["starter"]),
             "src/tests.rs": r["tests"].rstrip("\n") + "\n", "_solution/src/lib.rs": lib(r["solution"])}
    for rel, txt in files.items():
        with open(os.path.join(ex, rel), "w", encoding="utf-8", newline="\n") as f:
            f.write(txt)
    cfg = {"title": b.get("title", d), "language": "rust", "command": ["cargo", "test", "--offline"],
           "timeout": 120, "starter": ["src/lib.rs", "Cargo.toml"], "tests": ["src/tests.rs"]}
    with open(os.path.join(ex, "exercise.json"), "w", encoding="utf-8") as f:
        json.dump(cfg, f, ensure_ascii=False, indent=2)
        f.write("\n")


def main():
    here = os.path.dirname(os.path.abspath(__file__))
    course = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.dirname(os.path.dirname(here))
    only = set(sys.argv[2:])
    src = os.path.join(course, ".build", "authoring")
    out = os.path.join(course, "content")
    n = 0
    vpath = os.path.join(course, ".build", "verify", "playgrounds.json")
    try:
        verify = json.load(open(vpath, encoding="utf-8"))
    except (OSError, ValueError):
        verify = {}
    if os.path.exists(os.path.join(src, "course.yaml")):
        json.dump(yaml.safe_load(open(os.path.join(src, "course.yaml"), encoding="utf-8")),
                  open(os.path.join(out, "course.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=2)
        n += 1
    for ch in sorted(os.listdir(src)):
        d = os.path.join(src, ch)
        if not os.path.isdir(d) or (only and ch not in only):
            continue
        os.makedirs(os.path.join(out, ch), exist_ok=True)
        want = set()
        for fn in sorted(os.listdir(d)):
            if not fn.endswith(".yaml"):
                continue
            try:
                data = yaml.safe_load(open(os.path.join(d, fn), encoding="utf-8"))
            except yaml.YAMLError as e:
                sys.exit("YAML error in %s/%s: %s" % (ch, fn, e))
            for b in (data or {}).get("blocks", []) if isinstance(data, dict) else []:
                if isinstance(b, dict) and b.get("type") == "exercise" and "rust" in b:
                    write_rust_exercise(course, b, "%s/%s" % (ch, fn))
                if isinstance(b, dict) and b.get("type") == "playground" and b.get("id"):
                    verify[b["id"]] = {"solution_code": b.pop("solution_code", None), "starter_ok": b.pop("starter_ok", "run")}
            name = fn[:-5] + ".json"
            want.add(name)
            with open(os.path.join(out, ch, name), "w", encoding="utf-8") as f:
                json.dump(data, f, ensure_ascii=False, indent=2)
                f.write("\n")
            n += 1
        for old in os.listdir(os.path.join(out, ch)):  # drop JSON whose YAML was removed or renamed
            if old.endswith(".json") and old not in want:
                os.remove(os.path.join(out, ch, old))
    os.makedirs(os.path.dirname(vpath), exist_ok=True)
    with open(vpath, "w", encoding="utf-8") as f:
        json.dump(verify, f, ensure_ascii=False, indent=1, sort_keys=True)
    print("converted %d file(s)" % n)

if __name__ == "__main__":
    main()
