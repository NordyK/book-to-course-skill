#!/usr/bin/env python3
"""Check every Rust playground in content/: the starter compiles and runs (or fails the way `starter_ok` says it
should), and, for playgrounds with an output check, `solution_code` produces exactly the expected output.

Usage: python3 .build/tools/verify_playgrounds.py [COURSE_DIR] [CHAPTER ...]   (needs rustc)
"""
import json, os, subprocess, sys, tempfile

def run(code):
    with tempfile.TemporaryDirectory() as tmp:
        src, exe = os.path.join(tmp, "main.rs"), os.path.join(tmp, "main")
        open(src, "w").write(code)
        c = subprocess.run(["rustc", "--edition", "2024", "-o", exe, src], capture_output=True, text=True)
        if c.returncode:
            return "compile_error", c.stderr
        try:
            r = subprocess.run([exe], capture_output=True, text=True, timeout=10)
        except subprocess.TimeoutExpired:
            return "timeout", ""
        return ("run" if r.returncode == 0 else "panic"), r.stdout

def norm(s):
    return "\n".join(l.rstrip() for l in (s or "").replace("\r", "").split("\n")).strip("\n")

def main():
    here = os.path.dirname(os.path.abspath(__file__))
    course = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else os.path.dirname(os.path.dirname(here))
    only = set(sys.argv[2:])
    verify = json.load(open(os.path.join(course, ".build", "verify", "playgrounds.json")))
    bad = ok = 0
    content = os.path.join(course, "content")
    for ch in sorted(os.listdir(content)):
        if only and ch not in only or not os.path.isdir(os.path.join(content, ch)):
            continue
        for fn in sorted(os.listdir(os.path.join(content, ch))):
            les = json.load(open(os.path.join(content, ch, fn)))
            for b in les.get("blocks", []) if isinstance(les, dict) else []:
                if b.get("type") != "playground" or b.get("lang", "rust") != "rust":
                    continue
                v = verify.get(b["id"], {})
                want = v.get("starter_ok") or "run"
                status, out = run(b["code"])
                problems = []
                if status != want:
                    problems.append("starter: expected %s, got %s\n%s" % (want, status, out[-800:]))
                check = b.get("expected_output") is not None or b.get("must_contain")
                if check:
                    sol = v.get("solution_code")
                    if not sol:
                        problems.append("has an output check but no solution_code")
                    else:
                        st, o = run(sol)
                        if st != "run":
                            problems.append("solution: %s\n%s" % (st, o[-800:]))
                        elif b.get("expected_output") is not None and norm(o) != norm(b["expected_output"]):
                            problems.append("solution output differs:\n%r\nvs expected\n%r" % (norm(o), norm(b["expected_output"])))
                        elif any(m not in o for m in b.get("must_contain") or []):
                            problems.append("solution output misses must_contain")
                        if status == "run" and b.get("expected_output") is not None and norm(out) == norm(b["expected_output"]):
                            problems.append("starter already passes the check")
                if problems:
                    bad += 1
                    print("FAIL  %s/%s %s\n  %s" % (ch, fn, b["id"], "\n  ".join(problems)))
                else:
                    ok += 1
                    print("OK    %s" % b["id"])
    print("\n%d ok, %d failed" % (ok, bad))
    sys.exit(1 if bad else 0)

if __name__ == "__main__":
    main()
