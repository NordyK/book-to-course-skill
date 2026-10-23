#!/usr/bin/env python3
"""Insert `starter_ok` / `solution_code` into a playground block of an authoring YAML file, right after its id line.
Usage: addsol.py FILE ID [--starter-ok compile_error|panic] [--solution PATH]"""
import sys, argparse
ap = argparse.ArgumentParser(); ap.add_argument("file"); ap.add_argument("id")
ap.add_argument("--starter-ok"); ap.add_argument("--solution")
a = ap.parse_args()
lines = open(a.file, encoding="utf-8").read().split("\n")
i = next(k for k, l in enumerate(lines) if l.strip() == "id: " + a.id)
ind = lines[i][: len(lines[i]) - len(lines[i].lstrip())]
ins = []
if a.starter_ok:
    ins.append(ind + "starter_ok: " + a.starter_ok)
if a.solution:
    ins.append(ind + "solution_code: |")
    ins += [(ind + "  " + l) if l else "" for l in open(a.solution, encoding="utf-8").read().rstrip("\n").split("\n")]
lines[i + 1:i + 1] = ins
open(a.file, "w", encoding="utf-8").write("\n".join(lines))
