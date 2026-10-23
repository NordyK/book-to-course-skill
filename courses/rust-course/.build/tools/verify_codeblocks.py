#!/usr/bin/env python3
"""Compile every Rust `code` block that has a main function and an `output`, and check the shown output is what the
program really prints (for blocks labelled "Compiler output", that the first error line matches).
Usage: python3 .build/tools/verify_codeblocks.py COURSE_DIR CHAPTER...   (needs rustc; blocks using crates are skipped)"""
import json, os, sys, subprocess, tempfile, re
course = sys.argv[1]; chs = sys.argv[2:]
def run(code):
    with tempfile.TemporaryDirectory() as t:
        os.makedirs(t+"/src"); src=t+"/src/main.rs"; open(src,"w").write(code)
        c=subprocess.run(["rustc","--edition","2024","-A","warnings","-o",t+"/m","src/main.rs"],cwd=t,capture_output=True,text=True)
        if c.returncode: return "CE", c.stderr
        r=subprocess.run([t+"/m"],capture_output=True,text=True,timeout=10)
        return ("ok" if r.returncode==0 else "panic"), r.stdout
n=lambda s:"\n".join(l.rstrip() for l in s.strip().split("\n"))
for ch in chs:
    for fn in sorted(os.listdir(f"{course}/content/{ch}")):
        for b in json.load(open(f"{course}/content/{ch}/{fn}")).get("blocks",[]):
            if b.get("type")!="code" or b.get("lang")!="rust" or not b.get("output"): continue
            code=b["code"]
            if "fn main(" not in code: continue
            if re.search(r"^use (rand)", code, re.M): continue
            st,out=run(code)
            if b.get("output_label")=="Compiler output":
                exp=b["output"].strip().split("\n")[0]
                good = st=="CE" and exp in out
                if not good: print("DIFF",ch,fn,b.get("title"),"\n  want:",exp,"\n  got:",st,out[:600])
                elif n(out[:len(b['output'])+400]).find(n(b['output']))<0: print("NOTE text differs",ch,fn,"\n"+out[:900])
                continue
            if st!="ok" or n(out)!=n(b["output"]): print("DIFF",ch,fn,b.get("title"),st,"\n  want:",repr(n(b["output"])),"\n  got: ",repr(n(out) if st!='CE' else out[:400]))
print("done")
