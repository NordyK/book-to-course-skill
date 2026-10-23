#!/usr/bin/env python3
"""Local helper for the course page (standard library only, listens on 127.0.0.1).

What it does:
  * serves the course (index.html, assets/, course/data.js)
  * saves learner progress to progress/progress.json  (GET/POST /api/progress)
  * runs an exercise's unit tests when you click "Run tests"  (POST /api/run-tests); the console's edits
    to the exercise's starter files are written into exercises/<dir>/ first, so your editor sees them too
  * compiles and runs a single Rust program from the console  (POST /api/run, needs rustc on PATH)

The command that gets run comes from exercises/<dir>/exercise.json (written when the course was
built), never from the browser request. Only the folder name is taken from the request.

Usage:  python3 serve.py [--port 8765] [--no-browser]
"""
import argparse
import http.server
import json
import os
import re
import shutil
import socketserver
import subprocess
import sys
import tempfile
import threading
import time
import webbrowser

ROOT = os.path.dirname(os.path.abspath(__file__))
PROGRESS_DIR = os.path.join(ROOT, "progress")
PROGRESS = os.path.join(PROGRESS_DIR, "progress.json")
EXERCISES = os.path.join(ROOT, "exercises")
MAX_BODY = 5 * 1024 * 1024
OUTPUT_LIMIT = 20000
BLOCKED_PREFIXES = ("/.build", "/progress", "/exercises/")  # raw files are not served; the page gets what it needs via data.js


class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=ROOT, **kwargs)

    def log_message(self, fmt, *args):  # keep the terminal quiet
        if args and str(args[1]) not in ("200", "304"):
            sys.stderr.write("%s - %s\n" % (self.address_string(), fmt % args))

    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()

    # ---- helpers -------------------------------------------------------------------------
    def _host_ok(self):
        host = (self.headers.get("Host") or "").rsplit(":", 1)[0].strip("[]")
        return host in ("127.0.0.1", "localhost", "::1")

    def _json(self, code, obj):
        data = json.dumps(obj, ensure_ascii=False).encode("utf-8")
        self.send_response(code)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def _body(self):
        try:
            n = int(self.headers.get("Content-Length") or 0)
        except ValueError:
            n = 0
        if n <= 0 or n > MAX_BODY:
            return None
        try:
            return json.loads(self.rfile.read(n).decode("utf-8"))
        except (ValueError, UnicodeDecodeError):
            return None

    # ---- routes --------------------------------------------------------------------------
    def do_GET(self):
        if not self._host_ok():
            return self._json(403, {"error": "bad host"})
        path = self.path.split("?", 1)[0]
        if path == "/api/ping":
            return self._json(200, {"ok": True, "server": "course-helper", "version": 2,
                                    "run": {"rust": shutil.which("rustc") is not None,
                                            "cargo": shutil.which("cargo") is not None}})
        if path == "/api/progress":
            try:
                with open(PROGRESS, "r", encoding="utf-8") as f:
                    return self._json(200, json.load(f))
            except (OSError, ValueError):
                return self._json(200, {})
        if path.startswith(BLOCKED_PREFIXES) or path in ("/serve.py",):
            return self._json(404, {"error": "not found"})
        return super().do_GET()

    def do_POST(self):
        if not self._host_ok():
            return self._json(403, {"error": "bad host"})
        if "application/json" not in (self.headers.get("Content-Type") or ""):
            return self._json(415, {"error": "json only"})
        path = self.path.split("?", 1)[0]
        body = self._body()
        if not isinstance(body, dict):
            return self._json(400, {"error": "bad body"})
        if path == "/api/progress":
            return self._save_progress(body)
        if path == "/api/run-tests":
            return self._run_tests(body)
        if path == "/api/run":
            return self._run_program(body)
        return self._json(404, {"error": "not found"})

    def _save_progress(self, body):
        os.makedirs(PROGRESS_DIR, exist_ok=True)
        tmp = PROGRESS + ".tmp"
        with open(tmp, "w", encoding="utf-8") as f:
            json.dump(body, f, ensure_ascii=False, indent=2)
        if os.path.exists(PROGRESS):
            try:
                shutil.copyfile(PROGRESS, PROGRESS + ".bak")
            except OSError:
                pass
        os.replace(tmp, PROGRESS)
        return self._json(200, {"ok": True})

    def _run_tests(self, body):
        name = str(body.get("dir", ""))
        if not re.fullmatch(r"[A-Za-z0-9_.-]+", name) or name.startswith("."):
            return self._json(400, {"error": "bad exercise name"})
        ex_dir = os.path.join(EXERCISES, name)
        cfg_path = os.path.join(ex_dir, "exercise.json")
        try:
            with open(cfg_path, "r", encoding="utf-8") as f:
                cfg = json.load(f)
        except (OSError, ValueError):
            return self._json(404, {"error": "exercise not found"})
        cmd = cfg.get("command")
        if not (isinstance(cmd, list) and cmd and all(isinstance(c, str) for c in cmd)):
            return self._json(500, {"error": "exercise.json has no valid command"})
        timeout = min(max(int(cfg.get("timeout", 60)), 5), 300)
        files = body.get("files")
        if files is not None:
            err = self._write_starters(ex_dir, cfg, files)
            if err:
                return self._json(400, {"error": err})
        if shutil.which(cmd[0]) is None:
            return self._json(200, {"ok": False, "exit_code": 127, "timed_out": False,
                                    "output": "Program '%s' nie jest zainstalowany / is not installed.\n" % cmd[0]})
        env = dict(os.environ, PYTHONDONTWRITEBYTECODE="1", CI="1", RUST_BACKTRACE="0")
        t0 = time.time()
        timed_out = False
        try:
            p = subprocess.run(cmd, cwd=ex_dir, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                               timeout=timeout, stdin=subprocess.DEVNULL)
            code, out = p.returncode, p.stdout.decode("utf-8", "replace")
        except subprocess.TimeoutExpired as e:
            timed_out, code = True, -1
            out = (e.stdout or b"").decode("utf-8", "replace") if isinstance(e.stdout, bytes) else (e.stdout or "")
        if len(out) > OUTPUT_LIMIT:
            out = out[:OUTPUT_LIMIT // 2] + "\n… [obcięto / truncated] …\n" + out[-OUTPUT_LIMIT // 2:]
        return self._json(200, {"ok": code == 0 and not timed_out, "exit_code": code, "timed_out": timed_out,
                                "output": out, "duration_ms": int((time.time() - t0) * 1000)})


    def _write_starters(self, ex_dir, cfg, files):
        """Write console edits back into the exercise folder. Only files listed as starters may be written."""
        if not isinstance(files, dict):
            return "files must be an object"
        allowed = set(cfg.get("starter") or [])
        if not allowed:  # same default as build_course.py: every file that is not a test, README or exercise.json
            tests = set(cfg.get("tests") or [])
            for base, dirs, fs in os.walk(ex_dir):
                dirs[:] = [x for x in dirs if not x.startswith((".", "_")) and x != "target"]
                for fn in fs:
                    rel = os.path.relpath(os.path.join(base, fn), ex_dir).replace(os.sep, "/")
                    if rel != "exercise.json" and rel not in tests and not fn.lower().startswith("readme") and not fn.startswith("."):
                        allowed.add(rel)
        for rel, content in files.items():
            if rel not in allowed or not isinstance(content, str) or len(content) > 512 * 1024:
                return "not an editable file: %s" % rel
            dest = os.path.realpath(os.path.join(ex_dir, rel))
            if not dest.startswith(os.path.realpath(ex_dir) + os.sep):
                return "bad path"
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            with open(dest, "w", encoding="utf-8", newline="\n") as f:
                f.write(content)
        return None

    def _run_program(self, body):
        """Compile one Rust file with rustc and run it. Same trust model as the exercises: it runs the
        learner's own code on the learner's own machine, bound to 127.0.0.1 and checked by Host header."""
        code = body.get("code")
        if body.get("lang") != "rust" or not isinstance(code, str) or len(code) > 256 * 1024:
            return self._json(400, {"error": "send {lang: 'rust', code: '...'}"})
        rustc = shutil.which("rustc")
        if rustc is None:
            return self._json(200, {"ok": False, "compiled": False, "timed_out": False, "stdout": "", "stderr": "",
                                    "compile_output": "rustc is not installed. Install Rust from https://rustup.rs and restart start.sh.\n"})
        t0 = time.time()
        with tempfile.TemporaryDirectory(prefix="course-run-") as tmp:
            src, exe = os.path.join(tmp, "main.rs"), os.path.join(tmp, "main.exe" if os.name == "nt" else "main")
            with open(src, "w", encoding="utf-8") as f:
                f.write(code)
            try:
                c = subprocess.run([rustc, "--edition", "2024", "-C", "debuginfo=0", "--remap-path-prefix=%s=src" % tmp, "-o", exe, src], cwd=tmp,
                                   stdout=subprocess.PIPE, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL, timeout=60)
            except subprocess.TimeoutExpired:
                return self._json(200, {"ok": False, "compiled": False, "timed_out": True, "stdout": "", "stderr": "",
                                        "compile_output": "Compiling took longer than 60 seconds.\n"})
            comp = c.stdout.decode("utf-8", "replace").replace(src, "src/main.rs")
            if c.returncode != 0:
                return self._json(200, {"ok": False, "compiled": False, "timed_out": False, "stdout": "", "stderr": "",
                                        "compile_output": comp[:OUTPUT_LIMIT]})
            timed_out = False
            try:
                r = subprocess.run([exe], cwd=tmp, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                   stdin=subprocess.DEVNULL, timeout=10, env=dict(os.environ, RUST_BACKTRACE="0"))
                out, err, rc = r.stdout, r.stderr, r.returncode
            except subprocess.TimeoutExpired as e:
                timed_out, rc = True, -1
                out, err = e.stdout or b"", (e.stderr or b"") + b"\nStopped after 10 seconds (infinite loop?).\n"
        dec = lambda b: b.decode("utf-8", "replace")[:OUTPUT_LIMIT]
        return self._json(200, {"ok": rc == 0 and not timed_out, "compiled": True, "timed_out": timed_out, "exit_code": rc,
                                "compile_output": comp if "warning" in comp else "", "stdout": dec(out), "stderr": dec(err),
                                "duration_ms": int((time.time() - t0) * 1000)})


class Server(socketserver.ThreadingMixIn, http.server.HTTPServer):
    daemon_threads = True
    allow_reuse_address = True


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--port", type=int, default=8765)
    ap.add_argument("--no-browser", action="store_true")
    args = ap.parse_args()
    server = None
    for port in range(args.port, args.port + 30):
        try:
            server = Server(("127.0.0.1", port), Handler)
            break
        except OSError:
            continue
    if server is None:
        sys.exit("Nie znaleziono wolnego portu / no free port found.")
    url = "http://127.0.0.1:%d/" % server.server_address[1]
    print("Kurs działa / Course is running:  %s\nCtrl+C aby zakończyć / to stop." % url)
    if not args.no_browser:
        threading.Timer(0.6, lambda: webbrowser.open(url)).start()
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nZatrzymano / stopped.")


if __name__ == "__main__":
    main()
