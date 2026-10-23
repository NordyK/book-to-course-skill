#!/usr/bin/env python3
"""Replace the text of every `code` block labelled "Compiler output" in .build/authoring/CHAPTER/*.yaml with what the
installed rustc really prints (first line must already match). Usage: python3 .build/tools/fix_compiler_output.py COURSE_DIR CHAPTER"""
import json, os, sys, subprocess, tempfile, re, glob
course, ch = sys.argv[1], sys.argv[2]
def ce(code):
    with tempfile.TemporaryDirectory() as t:
        os.makedirs(t+"/src"); open(t+"/src/main.rs","w").write(code)
        c=subprocess.run(["rustc","--edition","2024","-A","warnings","-o",t+"/m","src/main.rs"],cwd=t,capture_output=True,text=True)
        return c.stderr
for fn in sorted(os.listdir(f"{course}/content/{ch}")):
    y=f"{course}/.build/authoring/{ch}/{fn[:-5]}.yaml"; ys=open(y).read(); changed=False
    for b in json.load(open(f"{course}/content/{ch}/{fn}")).get("blocks",[]):
        if b.get("type")!="code" or b.get("output_label")!="Compiler output": continue
        code=b["code"] if "fn main(" in b["code"] else b["code"]+"\nfn main() {}\n"
        err=ce(code); new=err.split("\nerror: aborting")[0].rstrip()
        new=re.sub(r"\n\s*\|\s*\Z","",new).rstrip()
        old=b["output"].rstrip("\n")
        if not err.startswith(old.split("\n")[0]): print("first line differs!",fn,err[:200]); continue
        if new==old: continue
        ind=lambda s:"\n".join(("      "+l) if l else "" for l in s.split("\n"))
        if ind(old) in ys: ys=ys.replace(ind(old),ind(new)); changed=True; print("fixed",fn,old.split("\n")[0][:60])
        else: print("could not locate",fn)
    if changed: open(y,"w").write(ys)
