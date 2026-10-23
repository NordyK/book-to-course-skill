#!/usr/bin/env python3
"""Lessons for this course are authored in YAML (block scalars keep Rust code readable) and converted
to the JSON files book-to-course's build_course.py reads.

  .build/authoring/course.yaml            -> content/course.json
  .build/authoring/<ch>/chapter.yaml      -> content/<ch>/chapter.json
  .build/authoring/<ch>/NN-name.yaml      -> content/<ch>/NN-name.json

Usage: python3 .build/tools/yaml2json.py [COURSE_DIR] [CHAPTER ...]
Edit the YAML, never the generated JSON (it is overwritten). Needs PyYAML (pip install pyyaml).
"""
import json, os, sys
import yaml

def main():
    here = os.path.dirname(os.path.abspath(__file__))
    course = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.dirname(os.path.dirname(here))
    only = set(sys.argv[2:])
    src = os.path.join(course, ".build", "authoring")
    out = os.path.join(course, "content")
    n = 0
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
            name = fn[:-5] + ".json"
            want.add(name)
            with open(os.path.join(out, ch, name), "w", encoding="utf-8") as f:
                json.dump(data, f, ensure_ascii=False, indent=2)
                f.write("\n")
            n += 1
        for old in os.listdir(os.path.join(out, ch)):  # drop JSON whose YAML was removed or renamed
            if old.endswith(".json") and old not in want:
                os.remove(os.path.join(out, ch, old))
    print("converted %d file(s)" % n)

if __name__ == "__main__":
    main()
