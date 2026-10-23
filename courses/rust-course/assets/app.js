/* Rust course player: reading column + live Verification Console.
   No dependencies. Works from file:// (progress in the browser, code runs on play.rust-lang.org)
   and through serve.py (progress in progress/progress.json, code runs on the learner's own rustc/cargo).
   The LaTeX, mini-markdown and highlighter sections come from the book-to-course template player. */
(function () {
"use strict";
const C = window.COURSE;
if (!C) { document.body.innerHTML = "<p style='padding:2rem;font-family:sans-serif'>Missing course/data.js. Run book-to-course's build_course.py first.</p>"; return; }

const STR = {
  home: "Course", progress: "Progress", glossary: "Glossary", resume: "Resume", start: "Start the course",
  savedFile: "Saved to file", savedBrowser: "Saved in browser", serverErr: "Not saved: server unreachable",
  min: "min", lesson: "Lesson", test: "Chapter test", chapter: "Chapter",
  prev: "Previous", next: "Next", step: "Step {0} of {1}", showAll: "Show all", stepByStep: "Step by step", result: "Result",
  reveal: "Show answer", hide: "Hide answer", thinkFirst: "Decide on your answer first, then check.",
  check: "Check", retry: "Try again", showAnswer: "Show answer", hint: "Hint", correct: "Correct", wrong: "Not quite",
  checkTest: "Submit test", score: "Score", passed: "Passed", failed: "Not passed yet", needScore: "Pass mark {0}%", best: "Best",
  unanswered: "{0} question(s) are unanswered and will count as wrong. Submit anyway?", yes: "Yes", no: "Cancel",
  tTrue: "True", tFalse: "False", moveUp: "Move up", moveDown: "Move down", yourAnswer: "Your answer",
  know: "I knew it", notYet: "Not yet", allKnown: "All cards known.", again: "Start over", cardsLeft: "{0} left", flipHint: "Click the card or press Space to flip",
  exercise: "Exercise", goal: "Goal", behavior: "What it must do", examples: "Examples", input: "Input", output: "Output",
  stPassed: "Passed", stFailed: "Tests failing", stTodo: "To do", markDone: "Mark as done", markUndone: "Mark as not done",
  showSolution: "Show solution", hideSolution: "Hide solution", confirmSolution: "Have a real try first: the struggle is where the learning happens. Show the solution anyway?",
  solution: "Reference solution", checklist: "Checklist", whereToWork: "Work in your own editor (optional)", runCmd: "Run the tests in a terminal",
  loadConsole: "Open in console", loaded: "In console", tryIt: "Try it",
  finishLesson: "Mark lesson complete", lessonDone: "Lesson complete", nextLesson: "Next lesson", yourPractice: "Your practice in this lesson",
  terms: "New terms", frombook: "From the book", summary: "Key points",
  resetConfirm: "Erase all progress, saved code and scores? This cannot be undone.", reset: "Reset progress",
  copy: "Copy", copied: "Copied",
};
const L = C.ui || {};
const t = (k, ...a) => { let s = STR[k] != null ? STR[k] : (L[k] != null ? L[k] : k); a.forEach((v, i) => { s = s.split("{" + i + "}").join(v); }); return s; };

/* ───────────── tiny DOM helpers ───────────── */
function h(tag, props, ...kids) {
  const el = document.createElement(tag);
  if (props) for (const [k, v] of Object.entries(props)) {
    if (v == null || v === false) continue;
    if (k === "class") el.className = v;
    else if (k === "html") el.innerHTML = v;
    else if (k.slice(0, 2) === "on" && typeof v === "function") el.addEventListener(k.slice(2), v);
    else el.setAttribute(k, v === true ? "" : v);
  }
  for (const kid of kids.flat(Infinity)) { if (kid == null || kid === false) continue; el.append(kid.nodeType ? kid : document.createTextNode(String(kid))); }
  return el;
}
const esc = s => String(s).replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");
const $ = (s, r) => (r || document).querySelector(s);
const $$ = (s, r) => Array.from((r || document).querySelectorAll(s));
function fill(el, ...kids) { el.replaceChildren(...kids.flat(Infinity).filter(k => k != null && k !== false).map(k => (k.nodeType ? k : document.createTextNode(String(k))))); return el; }
const reduceMotion = () => window.matchMedia && window.matchMedia("(prefers-reduced-motion: reduce)").matches;

/* ───────────── LaTeX → MathML translator ─────────────
   A small, dependency-free subset: algebra, functions, big operators, matrices/cases,
   sets/logic, Greek letters, common fonts and spacing. Unknown input never throws past
   tex2mml() — it falls back to the raw source so a broken formula never blanks the page.
   Command names below are also the lint allowlist read by scripts/build_course.py — keep
   the two in sync by only adding names inside the MATH-CMDS-START/END string literals. */
/* MATH-CMDS-START */
const MATH_CMD_NAMES = (
  "alpha beta gamma delta epsilon zeta eta theta iota kappa lambda mu nu xi pi rho sigma tau upsilon phi chi psi omega " +
  "varepsilon vartheta varpi varrho varsigma varphi Gamma Delta Theta Lambda Xi Pi Sigma Upsilon Phi Psi Omega " +
  "cdot times div pm mp ast circ ne neq le leq ge geq ll gg approx equiv sim simeq cong propto infty partial nabla degree " +
  "to rightarrow leftarrow leftrightarrow Rightarrow Leftarrow Leftrightarrow implies iff mapsto " +
  "in notin ni subset subseteq supset supseteq cup cap setminus emptyset varnothing forall exists nexists neg lnot land wedge lor vee mid colon " +
  "ldots cdots vdots ddots dots langle rangle lfloor rfloor lceil rceil vert Vert lvert rvert lVert rVert " +
  "sin cos tan cot sec csc arcsin arccos arctan sinh cosh tanh log ln lg exp deg gcd dim ker Pr det " +
  "sum prod coprod int iint iiint oint bigcup bigcap bigoplus lim limsup liminf max min sup inf limits nolimits " +
  "hat widehat bar overline vec tilde widetilde dot ddot overbrace underbrace overrightarrow underline " +
  "mathbb mathcal mathfrak mathbf mathit mathsf mathtt mathrm boldsymbol " +
  "frac dfrac tfrac binom sqrt left right begin end text textrm textbf textit operatorname quad qquad phantom pmod bmod " +
  "matrix pmatrix bmatrix Bmatrix vmatrix Vmatrix cases aligned array"
).split(" ");
/* MATH-CMDS-END */

const GREEK = {
  alpha: "α", beta: "β", gamma: "γ", delta: "δ", epsilon: "ε", zeta: "ζ", eta: "η", theta: "θ", iota: "ι",
  kappa: "κ", lambda: "λ", mu: "μ", nu: "ν", xi: "ξ", pi: "π", rho: "ρ", sigma: "σ", tau: "τ", upsilon: "υ",
  phi: "φ", chi: "χ", psi: "ψ", omega: "ω",
  varepsilon: "ϵ", vartheta: "ϑ", varpi: "ϖ", varrho: "ϱ", varsigma: "ς", varphi: "ϕ",
  Gamma: "Γ", Delta: "Δ", Theta: "Θ", Lambda: "Λ", Xi: "Ξ", Pi: "Π", Sigma: "Σ", Upsilon: "Υ", Phi: "Φ", Psi: "Ψ", Omega: "Ω"
};
const SYM = {
  cdot: "·", times: "×", div: "÷", pm: "±", mp: "∓", ast: "∗", circ: "∘",
  ne: "≠", neq: "≠", le: "≤", leq: "≤", ge: "≥", geq: "≥", ll: "≪", gg: "≫",
  approx: "≈", equiv: "≡", sim: "∼", simeq: "≃", cong: "≅", propto: "∝",
  infty: "∞", partial: "∂", nabla: "∇", degree: "°",
  to: "→", rightarrow: "→", leftarrow: "←", leftrightarrow: "↔",
  Rightarrow: "⇒", Leftarrow: "⇐", Leftrightarrow: "⇔", implies: "⇒", iff: "⇔", mapsto: "↦",
  in: "∈", notin: "∉", ni: "∋", subset: "⊂", subseteq: "⊆", supset: "⊃", supseteq: "⊇",
  cup: "∪", cap: "∩", setminus: "∖", emptyset: "∅", varnothing: "∅",
  forall: "∀", exists: "∃", nexists: "∄", neg: "¬", lnot: "¬", land: "∧", wedge: "∧", lor: "∨", vee: "∨",
  mid: "∣", colon: ":", ldots: "…", cdots: "⋯", vdots: "⋮", ddots: "⋱", dots: "…",
  langle: "⟨", rangle: "⟩", lfloor: "⌊", rfloor: "⌋", lceil: "⌈", rceil: "⌉",
  vert: "|", Vert: "‖", lvert: "|", rvert: "|", lVert: "‖", rVert: "‖"
};
const FUNCS = {}; "sin cos tan cot sec csc arcsin arccos arctan sinh cosh tanh log ln lg exp deg gcd dim ker Pr det".split(" ").forEach(w => { FUNCS[w] = 1; });
const BIGOPS = {
  sum: { sym: "∑", limits: true }, prod: { sym: "∏", limits: true }, coprod: { sym: "∐", limits: true },
  int: { sym: "∫", limits: false }, iint: { sym: "∬", limits: false }, iiint: { sym: "∭", limits: false }, oint: { sym: "∮", limits: false },
  bigcup: { sym: "⋃", limits: true }, bigcap: { sym: "⋂", limits: true }, bigoplus: { sym: "⊕", limits: true },
  lim: { text: true, limits: true }, limsup: { text: true, limits: true }, liminf: { text: true, limits: true },
  max: { text: true, limits: true }, min: { text: true, limits: true }, sup: { text: true, limits: true }, inf: { text: true, limits: true }
};
const ACCENT_OVER = { hat: "^", widehat: "^", bar: "‾", overline: "‾", vec: "→", tilde: "~", widetilde: "~", dot: "˙", ddot: "¨", overbrace: "⏞", overrightarrow: "→" };
const ACCENT_UNDER = { underline: "_", underbrace: "⏟" };
const FONT_CMDS = {}; "mathbb mathcal mathfrak mathbf mathit mathsf mathtt mathrm boldsymbol".split(" ").forEach(w => { FONT_CMDS[w] = 1; });
const SPACE_EM = { ",": 0.1667, ":": 0.2222, ";": 0.2778, "!": -0.1667, " ": 0.25 };
const DELIMS = {
  "(": "(", ")": ")", "[": "[", "]": "]", "|": "|", ".": "",
  "\\{": "{", "\\}": "}", "\\|": "‖", "\\langle": "⟨", "\\rangle": "⟩",
  "\\lfloor": "⌊", "\\rfloor": "⌋", "\\lceil": "⌈", "\\rceil": "⌉"
};
const ENVS = {
  matrix: { delim: null }, pmatrix: { delim: ["(", ")"] }, bmatrix: { delim: ["[", "]"] },
  Bmatrix: { delim: ["\\{", "\\}"] }, vmatrix: { delim: ["|", "|"] }, Vmatrix: { delim: ["‖", "‖"] },
  cases: { delim: ["\\{", null], align: "left" }, aligned: { delim: null, align: "right left" }, array: { delim: null }
};
const CODEPOINT_FONTS = {
  mathbb: { upper: 0x1D538, lower: 0x1D552, digit: 0x1D7D8, exceptions: { C: "ℂ", H: "ℍ", N: "ℕ", P: "ℙ", Q: "ℚ", R: "ℝ", Z: "ℤ" } },
  mathcal: { upper: 0x1D49C, lower: 0x1D4B6, digit: null, exceptions: { B: "ℬ", E: "ℰ", F: "ℱ", H: "ℋ", I: "ℐ", L: "ℒ", M: "ℳ", R: "ℛ", e: "ℯ", g: "ℊ", o: "ℴ" } },
  mathfrak: { upper: 0x1D504, lower: 0x1D51E, digit: null, exceptions: { C: "ℭ", H: "ℌ", I: "ℑ", R: "ℜ", Z: "ℨ" } }
};
function mapMathLetter(ch, font) {
  const def = CODEPOINT_FONTS[font];
  if (def.exceptions[ch]) return def.exceptions[ch];
  if (/[A-Z]/.test(ch)) return String.fromCodePoint(def.upper + (ch.charCodeAt(0) - 65));
  if (/[a-z]/.test(ch)) return String.fromCodePoint(def.lower + (ch.charCodeAt(0) - 97));
  if (/[0-9]/.test(ch) && def.digit != null) return String.fromCodePoint(def.digit + (ch.charCodeAt(0) - 48));
  return ch;
}

function mathTokenize(src) {
  const re = /\\\\|\\[A-Za-z]+|\\[,:;!\s{}|%$&#_^~]|[{}^_&]|\d+(?:\.\d+)?|[A-Za-z]|\s+|./gs;
  const toks = []; let m;
  while ((m = re.exec(src))) {
    const v = m[0];
    if (/^\s+$/.test(v)) continue;
    let k;
    if (v === "\\\\") k = "rowbreak";
    else if (v[0] === "\\" && v.length > 1 && /[A-Za-z]/.test(v[1])) k = "cmd";
    else if (v[0] === "\\") k = "esc";
    else if (v === "{" || v === "}") k = "brace";
    else if (v === "^" || v === "_") k = "script";
    else if (v === "&") k = "amp";
    else if (/^\d/.test(v)) k = "num";
    else if (/^[A-Za-z]$/.test(v)) k = "ident";
    else k = "op";
    toks.push({ k, v, pos: m.index });
  }
  return toks;
}

function mathParse(src) {
  const tokens = mathTokenize(src);
  let i = 0;
  const peek = () => tokens[i];
  const atEnd = () => i >= tokens.length;
  const next = () => tokens[i++];
  function fail(msg) { throw { mathErr: true, message: msg }; }

  function wrapRow(parts) { return parts.length === 1 ? parts[0].mml : "<mrow>" + parts.map(p => p.mml).join("") + "</mrow>"; }

  function parseScripted() {
    const base = parseAtom();
    let limits = !!base.limits;
    if (!atEnd() && peek().k === "cmd" && (peek().v === "\\limits" || peek().v === "\\nolimits")) limits = next().v === "\\limits";
    let sub = null, sup = null;
    while (!atEnd() && peek().k === "script") {
      const s = next(); const val = parseArg();
      if (s.v === "_") { if (sub) fail("double subscript"); sub = val; }
      else { if (sup) fail("double superscript"); sup = val; }
    }
    if (!sub && !sup) return base;
    const tag = limits ? (sub && sup ? "munderover" : sub ? "munder" : "mover") : (sub && sup ? "msubsup" : sub ? "msub" : "msup");
    const kids = [base.mml]; if (sub) kids.push(sub.mml); if (sup) kids.push(sup.mml);
    return { mml: "<" + tag + ">" + kids.join("") + "</" + tag + ">" };
  }

  function parseGroupInner() {
    next();
    const parts = [];
    while (!atEnd() && !(peek().k === "brace" && peek().v === "}")) parts.push(parseScripted());
    if (atEnd()) fail("unbalanced { }");
    next();
    return wrapRow(parts.length ? parts : [{ mml: "" }]);
  }

  function parseArg() {
    if (atEnd()) fail("expected an argument");
    if (peek().k === "brace" && peek().v === "{") return { mml: parseGroupInner() };
    return parseAtom();
  }

  function optArg() {
    if (!atEnd() && peek().k === "op" && peek().v === "[") {
      next(); const parts = [];
      while (!atEnd() && !(peek().k === "op" && peek().v === "]")) parts.push(parseScripted());
      if (atEnd()) fail("unbalanced [ ]");
      next();
      return wrapRow(parts.length ? parts : [{ mml: "" }]);
    }
    return null;
  }

  function readRawArg() {
    if (atEnd()) fail("expected an argument");
    if (peek().k === "brace" && peek().v === "{") {
      const open = next(); let depth = 1, endPos = null;
      while (!atEnd() && depth > 0) {
        const tk = next();
        if (tk.k === "brace" && tk.v === "{") depth++;
        else if (tk.k === "brace" && tk.v === "}") { depth--; if (depth === 0) endPos = tk.pos; }
      }
      if (endPos == null) fail("unbalanced { }");
      return src.slice(open.pos + 1, endPos);
    }
    return next().v;
  }

  function readDelim() {
    if (atEnd()) fail("expected a delimiter after \\left or \\right");
    const tok = next();
    if (!(tok.v in DELIMS)) fail("'" + tok.v + "' is not a valid \\left/\\right delimiter");
    return DELIMS[tok.v];
  }

  function decorate(tag, ch) {
    const a = parseArg();
    return { mml: "<" + tag + ' accent="true">' + a.mml + '<mo stretchy="true">' + esc(ch) + "</mo></" + tag + ">" };
  }

  function fracAtom(name) {
    const num = parseArg(), den = parseArg();
    const disp = name === "dfrac" ? ' displaystyle="true"' : name === "tfrac" ? ' displaystyle="false"' : "";
    return { mml: "<mfrac" + disp + ">" + num.mml + den.mml + "</mfrac>" };
  }
  function binomAtom() {
    const n = parseArg(), k = parseArg();
    return { mml: '<mrow><mo stretchy="true">(</mo><mfrac linethickness="0">' + n.mml + k.mml + '</mfrac><mo stretchy="true">)</mo></mrow>' };
  }
  function sqrtAtom() {
    const idx = optArg(); const a = parseArg();
    return idx ? { mml: "<mroot>" + a.mml + idx + "</mroot>" } : { mml: "<msqrt>" + a.mml + "</msqrt>" };
  }
  function leftAtom() {
    const open = readDelim(); const parts = [];
    while (!atEnd() && !(peek().k === "cmd" && peek().v === "\\right")) parts.push(parseScripted());
    if (atEnd()) fail("\\left without a matching \\right");
    next(); const close = readDelim();
    const o = open ? '<mo stretchy="true" fence="true">' + esc(open) + "</mo>" : "";
    const c = close ? '<mo stretchy="true" fence="true">' + esc(close) + "</mo>" : "";
    return { mml: "<mrow>" + o + wrapRow(parts.length ? parts : [{ mml: "" }]) + c + "</mrow>" };
  }
  function readEnvName() {
    if (atEnd() || !(peek().k === "brace" && peek().v === "{")) fail("expected { after \\begin or \\end");
    return readRawArg();
  }
  function envAtom() {
    const name = readEnvName();
    if (!(name in ENVS)) fail("unknown environment '" + name + "'");
    const def = ENVS[name];
    const rows = [[[]]];
    for (;;) {
      if (atEnd()) fail("\\begin{" + name + "} without a matching \\end");
      const p = peek();
      if (p.k === "cmd" && p.v === "\\end") {
        next(); const en = readEnvName();
        if (en !== name) fail("\\begin{" + name + "} closed by \\end{" + en + "}");
        break;
      }
      if (p.k === "rowbreak") { next(); rows.push([[]]); continue; }
      if (p.k === "amp") { next(); rows[rows.length - 1].push([]); continue; }
      rows[rows.length - 1][rows[rows.length - 1].length - 1].push(parseScripted());
    }
    const aligns = (def.align || "center").split(" ");
    const trs = rows.filter(r => r.length > 1 || r[0].length > 0)
      .map(cells => "<mtr>" + cells.map((parts, ci) => '<mtd columnalign="' + aligns[Math.min(ci, aligns.length - 1)] + '">' + wrapRow(parts.length ? parts : [{ mml: "" }]) + "</mtd>").join("") + "</mtr>")
      .join("");
    let mml = "<mtable>" + trs + "</mtable>";
    if (def.delim) {
      const [o, c] = def.delim;
      const om = o ? '<mo stretchy="true" fence="true">' + esc(DELIMS[o] != null ? DELIMS[o] : o) + "</mo>" : "";
      const cm = c ? '<mo stretchy="true" fence="true">' + esc(DELIMS[c] != null ? DELIMS[c] : c) + "</mo>" : "";
      mml = "<mrow>" + om + mml + cm + "</mrow>";
    }
    return { mml };
  }
  function textAtom(style) {
    const raw = readRawArg();
    const sty = style === "bold" ? ' style="font-weight:bold"' : style === "italic" ? ' style="font-style:italic"' : "";
    return { mml: "<mtext" + sty + ">" + esc(raw) + "</mtext>" };
  }
  function operatornameAtom() { return { mml: "<mi>" + esc(readRawArg()) + "</mi>" }; }
  function pmodAtom() {
    const a = parseArg();
    return { mml: '<mrow><mspace width="0.3em"></mspace><mo>(</mo><mi>mod</mi><mspace width="0.3em"></mspace>' + a.mml + "<mo>)</mo></mrow>" };
  }
  function fontAtom(name) {
    const raw = readRawArg();
    if (name === "mathbb" || name === "mathcal" || name === "mathfrak") {
      const mapped = [...raw].map(ch => /[A-Za-z0-9]/.test(ch) ? mapMathLetter(ch, name) : ch).join("");
      return { mml: "<mi>" + esc(mapped) + "</mi>" };
    }
    if (name === "mathrm") return { mml: '<mi mathvariant="normal">' + esc(raw) + "</mi>" };
    const sty = name === "mathbf" || name === "boldsymbol" ? "font-weight:bold" : name === "mathit" ? "font-style:italic" : name === "mathsf" ? "font-family:sans-serif" : name === "mathtt" ? "font-family:monospace" : "";
    return { mml: '<mi style="' + sty + '">' + esc(raw) + "</mi>" };
  }

  function cmdAtom(name) {
    if (name in GREEK) return { mml: "<mi>" + GREEK[name] + "</mi>" };
    if (name in SYM) return { mml: "<mo>" + SYM[name] + "</mo>" };
    if (name in FUNCS) return { mml: "<mi>" + name + "</mi>" };
    if (name in BIGOPS) { const o = BIGOPS[name]; return { mml: o.text ? "<mi>" + name + "</mi>" : "<mo>" + o.sym + "</mo>", limits: o.limits }; }
    if (name in ACCENT_OVER) return decorate("mover", ACCENT_OVER[name]);
    if (name in ACCENT_UNDER) return decorate("munder", ACCENT_UNDER[name]);
    if (name in FONT_CMDS) return fontAtom(name);
    switch (name) {
      case "frac": case "dfrac": case "tfrac": return fracAtom(name);
      case "binom": return binomAtom();
      case "sqrt": return sqrtAtom();
      case "left": return leftAtom();
      case "right": fail("\\right without a matching \\left"); break;
      case "begin": return envAtom();
      case "end": fail("\\end without a matching \\begin"); break;
      case "text": case "textrm": return textAtom();
      case "textbf": return textAtom("bold");
      case "textit": return textAtom("italic");
      case "operatorname": return operatornameAtom();
      case "quad": return { mml: '<mspace width="1em"></mspace>' };
      case "qquad": return { mml: '<mspace width="2em"></mspace>' };
      case "phantom": parseArg(); return { mml: "<mspace></mspace>" };
      case "pmod": return pmodAtom();
      case "bmod": return { mml: '<mo lspace="0.3em" rspace="0.3em">mod</mo>' };
      default: fail("unknown command \\" + name);
    }
  }

  function opAtom(ch) { return { mml: ch === "'" ? "<mo>′</mo>" : "<mo>" + esc(ch) + "</mo>" }; }
  function escAtom(ch) {
    if (ch in SPACE_EM) return { mml: '<mspace width="' + SPACE_EM[ch] + 'em"></mspace>' };
    if (ch === "~") return { mml: '<mspace width="0.25em"></mspace>' };
    return { mml: "<mo>" + esc(ch) + "</mo>" };
  }
  function parseAtom() {
    if (atEnd()) fail("unexpected end of formula");
    const tok = peek();
    if (tok.k === "brace" && tok.v === "{") return { mml: parseGroupInner() };
    if (tok.k === "brace") fail("unexpected }");
    if (tok.k === "num") { next(); return { mml: "<mn>" + esc(tok.v) + "</mn>" }; }
    if (tok.k === "ident") { next(); return { mml: "<mi>" + esc(tok.v) + "</mi>" }; }
    if (tok.k === "amp" || tok.k === "rowbreak") fail("unexpected '" + tok.v + "' outside a matrix/cases environment");
    if (tok.k === "op") { next(); return opAtom(tok.v); }
    if (tok.k === "esc") { next(); return escAtom(tok.v.slice(1)); }
    if (tok.k === "cmd") { next(); return cmdAtom(tok.v.slice(1)); }
    fail("unexpected token '" + tok.v + "'");
  }

  const parts = [];
  while (!atEnd()) parts.push(parseScripted());
  return wrapRow(parts.length ? parts : [{ mml: "" }]);
}

/* MathML Core shipped in Chrome/Edge 109 (Jan 2023); older browsers treat <math>/<mfrac>/…
   as unknown inline elements with no special layout, so a real fraction renders no taller
   than a single bare number — that gap is the signal this probe checks for. */
let mathmlOk = null;
function mathmlSupported() {
  if (mathmlOk !== null) return mathmlOk;
  try {
    const probe = h("div", { style: "position:absolute;visibility:hidden;left:-9999px;top:-9999px" });
    document.body.append(probe);
    // measure the <math> element itself, not the wrapper div — an unsupported browser gives
    // it no special layout, so its own height stays flat instead of growing with the fraction
    probe.innerHTML = '<math><mfrac><mn>1</mn><mn>2</mn></mfrac></math>';
    const fracH = probe.firstChild.getBoundingClientRect().height;
    probe.innerHTML = '<math><mn>1</mn></math>';
    const plainH = probe.firstChild.getBoundingClientRect().height;
    probe.remove();
    // no usable layout yet (e.g. probed inside a hidden/display:none ancestor) — can't tell
    // either way, so assume supported and don't cache a guess; a later real call can re-probe
    if (!plainH) return true;
    mathmlOk = fracH > plainH * 1.3;
  } catch (e) { mathmlOk = true; }
  return mathmlOk;
}
let mathmlWarned = false;
function tex2mml(src, display) {
  if (!mathmlSupported()) {
    if (!mathmlWarned) { mathmlWarned = true; document.body.prepend(h("div", { class: "mathml-warn" }, "⚠ " + t("mathmlUnsupported"))); }
    const raw = (display ? "$$" : "$") + src + (display ? "$$" : "$");
    return '<span class="math-raw" title="' + esc(t("mathmlUnsupported")) + '">' + esc(raw) + "</span>";
  }
  try {
    const body = mathParse(String(src == null ? "" : src));
    return '<math display="' + (display ? "block" : "inline") + '">' + body + "</math>";
  } catch (e) {
    const msg = (e && e.mathErr) ? e.message : "math error";
    const raw = (display ? "$$" : "$") + src + (display ? "$$" : "$");
    return '<span class="math-err" title="' + esc(msg) + '">' + esc(raw) + "</span>";
  }
}

/* ───────────── mini markdown (safe: escapes everything first) ───────────── */
function inline(s) {
  const codes = [], maths = [];
  s = String(s).replace(/`([^`]+)`/g, (m, c) => { codes.push(c); return "\u0000" + (codes.length - 1) + "\u0000"; });
  s = s.replace(/\\\$/g, () => "\u0002");
  s = s.replace(/\$\$([^\n]+?)\$\$|\$(?=\S)([^$\n]*[^\s$])\$(?!\d)/g, (m, disp, inl) => {
    const body = disp != null ? disp : inl; maths.push(tex2mml(body, disp != null));
    return "\u0001" + (maths.length - 1) + "\u0001";
  });
  s = esc(s);
  s = s.replace(/\*\*([^*]+)\*\*/g, "<strong>$1</strong>").replace(/(^|[^*\w])\*([^*\s][^*]*?)\*(?!\w)/g, "$1<em>$2</em>");
  s = s.replace(/\[([^\]]+)\]\((https?:[^)\s]+)\)/g, '<a href="$2" target="_blank" rel="noopener noreferrer">$1</a>');
  s = s.replace(/\u0001(\d+)\u0001/g, (m, i) => maths[+i]);
  s = s.replace(/\u0002/g, "$");
  return s.replace(/\u0000(\d+)\u0000/g, (m, i) => "<code>" + esc(codes[+i]) + "</code>");
}
function md(src) {
  const lines = String(src == null ? "" : src).replace(/\r/g, "").split("\n");
  const isList = l => /^\s*([-*]|\d+\.)\s+/.test(l);
  let html = "", i = 0;
  while (i < lines.length) {
    const l = lines[i];
    if (!l.trim()) { i++; continue; }
    let m;
    if (/^```/.test(l)) {
      const lang = l.replace(/^```/, "").trim(); const buf = []; i++;
      while (i < lines.length && !/^```/.test(lines[i])) buf.push(lines[i++]);
      i++;
      html += '<div class="code"><pre><code>' + highlightLines(buf.join("\n"), lang).map(x => '<span class="ln">' + x + "</span>").join("") + "</code></pre></div>";
      continue;
    }
    if (/^\$\$/.test(l)) {
      let rest = l.replace(/^\$\$/, ""); const buf = [];
      const closeIdx = rest.indexOf("$$");
      if (closeIdx >= 0) { buf.push(rest.slice(0, closeIdx)); i++; }
      else {
        buf.push(rest); i++;
        while (i < lines.length && lines[i].indexOf("$$") < 0) buf.push(lines[i++]);
        if (i < lines.length) { buf.push(lines[i].slice(0, lines[i].indexOf("$$"))); i++; }
      }
      html += '<div class="mathblock">' + tex2mml(buf.join("\n"), true) + "</div>";
      continue;
    }
    if ((m = /^(#{1,4})\s+(.*)$/.exec(l))) { const n = Math.min(m[1].length + 2, 6); html += "<h" + n + ">" + inline(m[2]) + "</h" + n + ">"; i++; continue; }
    if (/^>\s?/.test(l)) { const buf = []; while (i < lines.length && /^>\s?/.test(lines[i])) buf.push(lines[i++].replace(/^>\s?/, "")); html += "<blockquote>" + inline(buf.join(" ")) + "</blockquote>"; continue; }
    if (isList(l)) {
      const ordered = /^\s*\d+\./.test(l); const items = [];
      while (i < lines.length && isList(lines[i])) items.push(lines[i++].replace(/^\s*([-*]|\d+\.)\s+/, ""));
      const tag = ordered ? "ol" : "ul";
      html += "<" + tag + ">" + items.map(x => "<li>" + inline(x) + "</li>").join("") + "</" + tag + ">"; continue;
    }
    const buf = [];
    while (i < lines.length && lines[i].trim() && !isList(lines[i]) && !/^#{1,4}\s/.test(lines[i]) && !/^>/.test(lines[i]) && !/^```/.test(lines[i]) && !/^\$\$/.test(lines[i])) buf.push(lines[i++]);
    html += "<p>" + inline(buf.join(" ")) + "</p>";
  }
  return html;
}
/* ───────────── syntax highlighting (small, dependency-free) ───────────── */
const KW = {
  go: ["break case chan const continue default defer else fallthrough for func go goto if import interface map package range return select struct switch type var", "true false nil iota", "append cap close copy delete len make new panic print println recover string int int8 int16 int32 int64 uint uint8 uint16 uint32 uint64 uintptr float32 float64 bool byte rune error any"],
  python: ["and as assert async await break class continue def del elif else except finally for from global if import in is lambda nonlocal not or pass raise return try while with yield match case", "True False None self cls", "print len range int str float list dict set tuple bool type input open enumerate zip map filter sum min max abs sorted reversed isinstance super"],
  js: ["async await break case catch class const continue debugger default delete do else export extends finally for from function if import in instanceof let new of return static super switch this throw try typeof var void while with yield interface type enum implements public private protected readonly", "true false null undefined NaN Infinity", "console Math JSON Object Array String Number Boolean Promise Map Set Date Error require module process"],
  java: ["abstract assert break case catch class const continue default do else enum extends final finally for goto if implements import instanceof interface native new package private protected public return static strictfp super switch synchronized this throw throws transient try volatile while var record", "true false null", "String Integer Long Double Boolean List Map Set System Object Math ArrayList HashMap"],
  c: ["auto break case const continue default do else enum extern for goto if inline register return sizeof static struct switch typedef union volatile while class namespace template typename public private protected virtual new delete using nullptr", "true false NULL", "int char float double long short unsigned signed void bool size_t printf scanf malloc free std string vector cout cin endl"],
  rust: ["as async await break const continue crate dyn else enum extern fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait type unsafe use where while gen", "true false None Some Ok Err", "i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 usize isize f32 f64 bool char str String Vec Option Result Box Rc RefCell Arc Mutex RwLock Cell HashMap HashSet BTreeMap VecDeque Iterator IntoIterator Fn FnMut FnOnce Copy Clone Debug Display Default Drop Deref From Into Send Sync Sized PartialEq Eq PartialOrd Ord Hash"],
  bash: ["if then else elif fi for while do done case esac in function return exit local export source alias unset set", "true false", "echo cd ls cat grep sed awk chmod mkdir rm cp mv curl git sudo npm pip python go"],
  sql: ["select from where and or not insert into values update set delete create table drop alter add join inner left right outer on group by order having limit offset as distinct union all null is in like between primary key foreign references index view", "true false null", "count sum avg min max coalesce"]
};
const ALIAS = { toml: "toml",  golang: "go", py: "python", python3: "python", javascript: "js", node: "js", jsx: "js", ts: "js", typescript: "js", tsx: "js", cpp: "c", "c++": "c", "c#": "java", cs: "java", kotlin: "java", shell: "bash", sh: "bash", zsh: "bash", rs: "rust", mysql: "sql", postgres: "sql", sqlite: "sql" };
const COMMENTS = { go: ["//", "/*"], js: ["//", "/*"], java: ["//", "/*"], c: ["//", "/*"], rust: ["//", "/*"], python: ["#"], bash: ["#"], sql: ["--", "/*"] };
const reCache = {};
function langDef(lang) {
  const k = ALIAS[String(lang || "").toLowerCase()] || String(lang || "").toLowerCase();
  if (!KW[k]) return null;
  if (!reCache[k]) {
    const [kw, lit, bi] = KW[k].map(s => new Set(s.split(" ")));
    const com = COMMENTS[k] || [];
    const parts = [];
    if (com.indexOf("/*") >= 0) parts.push("\\/\\*[\\s\\S]*?(?:\\*\\/|$)");
    if (com.indexOf("//") >= 0) parts.push("\\/\\/[^\\n]*");
    if (com.indexOf("#") >= 0) parts.push("#[^\\n]*");
    if (com.indexOf("--") >= 0) parts.push("--[^\\n]*");
    if (k === "python") parts.push('"""[\\s\\S]*?(?:"""|$)', "'''[\\s\\S]*?(?:'''|$)");
    if (k === "rust") parts.push('b?"(?:\\\\.|[^"\\\\])*"', "b?'(?:\\\\.|[^'\\\\\\n])'", "'[A-Za-z_]\\w*");
    else parts.push('"(?:\\\\.|[^"\\\\\\n])*"', "'(?:\\\\.|[^'\\\\\\n])*'");
    if (k === "go" || k === "js") parts.push("`[^`]*`");
    parts.push("\\b0x[0-9a-fA-F_]+\\b|\\b\\d[\\d_]*(?:\\.\\d+)?(?:[eE][+-]?\\d+)?\\b", "[A-Za-z_$][\\w$]*");
    reCache[k] = { re: new RegExp(parts.join("|"), "g"), kw, lit, bi, ci: k === "sql", rust: k === "rust" };
  }
  return reCache[k];
}
function highlightLines(code, lang) {
  const def = langDef(lang);
  const lines = [""];
  const push = (cls, text) => {
    const pieces = text.split("\n");
    pieces.forEach((p, i) => {
      if (i > 0) lines.push("");
      if (p) lines[lines.length - 1] += cls ? '<span class="tk-' + cls + '">' + esc(p) + "</span>" : esc(p);
    });
  };
  code = String(code == null ? "" : code).replace(/\r/g, "").replace(/\n+$/, "");
  if (!def) { push("", code); return lines; }
  let last = 0, m;
  def.re.lastIndex = 0;
  while ((m = def.re.exec(code))) {
    if (m.index > last) push("", code.slice(last, m.index));
    const tok = m[0]; let cls = "";
    const c = tok[0];
    if (tok.startsWith("//") || tok.startsWith("/*") || tok.startsWith("--") || (c === "#" && !/^#include|^#define/.test(tok) && true)) cls = "com";
    else if (def.rust && c === "'" && !(tok.length >= 3 && tok[tok.length - 1] === "'")) cls = "lit";
    else if (c === '"' || c === "'" || c === "`" || (def.rust && c === "b" && (tok[1] === '"' || tok[1] === "'"))) cls = "str";
    else if (/^\d/.test(tok)) cls = "num";
    else {
      const w = def.ci ? tok.toLowerCase() : tok;
      if (def.kw.has(w)) cls = "kw"; else if (def.lit.has(w)) cls = "lit"; else if (def.bi.has(w)) cls = "bi";
      else if (def.rust && code[def.re.lastIndex] === "!") cls = "mac";
      else if (code[def.re.lastIndex] === "(") cls = "fn";
    }
    push(cls, tok); last = def.re.lastIndex;
    if (tok.length === 0) def.re.lastIndex++;
  }
  if (last < code.length) push("", code.slice(last));
  return lines;
}
const EXT = { go: "go", py: "python", js: "js", mjs: "js", ts: "ts", java: "java", c: "c", h: "c", cpp: "cpp", rs: "rust", sh: "bash", sql: "sql", json: "json" };
const langOf = p => EXT[String(p || "").split(".").pop().toLowerCase()] || "";

/* ───────────── icons (authored, one stroke weight) ───────────── */
const ICON = {
  check: '<path d="M4 12.5l5 5L20 6.5"/>', cross: '<path d="M6 6l12 12M18 6L6 18"/>', play: '<path d="M7 4.5v15l12-7.5z"/>',
  reset: '<path d="M4 12a8 8 0 1 0 2.4-5.7M4 4v4.5h4.5"/>', bulb: '<path d="M9 18h6M10 21h4M8.5 14.5A6 6 0 1 1 15.5 14.5c-.8.7-1.5 1.6-1.5 2.5h-4c0-.9-.7-1.8-1.5-2.5z"/>',
  book: '<path d="M4 5.5A2.5 2.5 0 0 1 6.5 3H20v15H6.5A2.5 2.5 0 0 0 4 20.5zM4 20.5A2.5 2.5 0 0 0 6.5 21H20"/>',
  flag: '<path d="M5 21V4M5 4h11l-2 4 2 4H5"/>', chev: '<path d="M9 6l6 6-6 6"/>', chevL: '<path d="M15 6l-6 6 6 6"/>',
  menu: '<path d="M4 7h16M4 12h16M4 17h16"/>', sun: '<circle cx="12" cy="12" r="4"/><path d="M12 2v2M12 20v2M2 12h2M20 12h2M4.9 4.9l1.4 1.4M17.7 17.7l1.4 1.4M4.9 19.1l1.4-1.4M17.7 6.3l1.4-1.4"/>',
  moon: '<path d="M20 14.5A8 8 0 0 1 9.5 4 8 8 0 1 0 20 14.5z"/>', term: '<path d="M3 5h18v14H3zM7 9l3 3-3 3M12 15h5"/>',
  copy: '<path d="M8 8h12v12H8zM16 8V4H4v12h4"/>', eye: '<path d="M2 12s3.5-7 10-7 10 7 10 7-3.5 7-10 7S2 12 2 12z"/><circle cx="12" cy="12" r="3"/>',
  plus: '<path d="M12 4v16M4 12h16"/>', clock: '<circle cx="12" cy="12" r="9"/><path d="M12 7v5l3 2"/>', file: '<path d="M6 3h8l4 4v14H6zM14 3v4h4"/>',
  alert: '<path d="M12 3l10 18H2zM12 10v5M12 18v.5"/>', info: '<circle cx="12" cy="12" r="9"/><path d="M12 11v6M12 7.5v.5"/>',
  key: '<circle cx="8" cy="15" r="4"/><path d="M11 12l9-9M16 7l3 3M14 9l2 2"/>', puzzle: '<path d="M4 8h4a2 2 0 1 1 4 0h4v4a2 2 0 1 1 0 4v4H4z"/>',
  search: '<circle cx="11" cy="11" r="7"/><path d="M20 20l-4-4"/>', spin: '<path d="M12 3a9 9 0 1 1-9 9"/>', dot: '<circle cx="12" cy="12" r="3"/>',
  close: '<path d="M6 6l12 12M18 6L6 18"/>', download: '<path d="M12 3v12M7 10l5 5 5-5M4 21h16"/>', upload: '<path d="M12 21V9M7 14l5-5 5 5M4 3h16"/>',
  cards: '<path d="M4 7h12v13H4zM8 7V4h12v13h-4"/>', steps: '<path d="M4 20h5v-5h5v-5h5V5"/>', flow: '<path d="M3 12h6M15 12h6M9 8h6v8H9z"/>',
  list: '<path d="M8 6h13M8 12h13M8 18h13M3.5 6h.5M3.5 12h.5M3.5 18h.5"/>', tool: '<path d="M14.5 6.5a4 4 0 0 0 5 5L12 19a2.1 2.1 0 0 1-3-3l7.5-7.5a4 4 0 0 0-2-2z"/>',
  spark: '<path d="M12 3v6M12 15v6M3 12h6M15 12h6"/>', quote: '<path d="M5 18c3 0 4-2 4-5V7H4v6h4M15 18c3 0 4-2 4-5V7h-5v6h4"/>'
};
const ic = (n, cls) => { const s = document.createElementNS("http://www.w3.org/2000/svg", "svg"); s.setAttribute("viewBox", "0 0 24 24"); s.setAttribute("class", "i" + (cls ? " " + cls : "")); s.setAttribute("aria-hidden", "true"); s.innerHTML = ICON[n] || ""; return s; };
function plusMark(x, y, side) { const p = ic("plus", "plus"); p.style[side || "left"] = x; p.style.top = y; return p; }

/* ───────────── progress model ───────────── */
const BLANK = () => ({ version: 2, lessons: {}, quizzes: {}, exercises: {}, playgrounds: {}, cards: {}, code: {}, activity: [], days: {}, last: null, started: null, updated: null });
let P = BLANK();
let serverMode = false, serverCaps = {}, saveTimer = null, saveErr = false;
const LS_KEY = "course-progress:" + C.meta.id;
function migrate(o) {
  const p = Object.assign(BLANK(), o || {});
  ["lessons", "quizzes", "exercises", "playgrounds", "cards", "code", "days"].forEach(k => { if (!p[k] || typeof p[k] !== "object" || Array.isArray(p[k])) p[k] = {}; });
  if (!Array.isArray(p.activity)) p.activity = [];
  p.version = 2; return p;
}
async function loadProgress() {
  let local = null;
  try { local = JSON.parse(localStorage.getItem(LS_KEY) || "null"); } catch (e) { /* storage blocked */ }
  let remote = null;
  if (location.protocol.indexOf("http") === 0) {
    try {
      const r = await fetch("api/ping", { cache: "no-store" });
      const j = r.ok ? await r.json() : null;
      if (j && j.ok) { serverMode = true; serverCaps = j.run || {}; remote = await (await fetch("api/progress", { cache: "no-store" })).json(); }
    } catch (e) { serverMode = false; }
  }
  const newer = (a, b) => ((a && a.updated) || "") >= ((b && b.updated) || "") ? a : b;
  const pick = serverMode ? (remote && remote.updated ? newer(remote, local) : local || remote) : local;
  P = migrate(pick);
  if (serverMode && (!remote || !remote.updated || (local && local.updated > remote.updated))) save(true);
}
function save(now) {
  P.updated = new Date().toISOString();
  try { localStorage.setItem(LS_KEY, JSON.stringify(P)); } catch (e) { /* ignore */ }
  const post = () => {
    if (!serverMode) return;
    fetch("api/progress", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(P) })
      .then(r => { if (!r.ok) throw new Error(); saveErr = false; setSaveState(); })
      .catch(() => { saveErr = true; setSaveState(); });
  };
  clearTimeout(saveTimer);
  if (now) post(); else saveTimer = setTimeout(post, 400);
  setSaveState(); refreshChrome();
}
function setSaveState() {
  const s = $("#saveState"); if (!s) return;
  s.className = "save-state" + (saveErr ? " err" : "");
  fill(s, ic(saveErr ? "alert" : "check", "sm"), h("span", null, saveErr ? t("serverErr") : serverMode ? t("savedFile") : t("savedBrowser")));
}
const today = () => { const d = new Date(); return d.getFullYear() + "-" + String(d.getMonth() + 1).padStart(2, "0") + "-" + String(d.getDate()).padStart(2, "0"); };
function log(kind, id, detail) {
  P.activity.unshift({ at: new Date().toISOString(), kind, id, detail: detail || "" });
  if (P.activity.length > 400) P.activity.length = 400;
  const d = today(); P.days[d] = P.days[d] || { minutes: 0, actions: 0 }; P.days[d].actions++;
  if (!P.started) P.started = new Date().toISOString();
}

/* active study time: counted while the tab is visible and the learner did something in the last 2 minutes */
let lastInput = Date.now();
["pointerdown", "keydown", "scroll", "input"].forEach(ev => window.addEventListener(ev, () => { lastInput = Date.now(); }, { passive: true }));
setInterval(() => {
  if (document.hidden || Date.now() - lastInput > 120000) return;
  const d = today(); P.days[d] = P.days[d] || { minutes: 0, actions: 0 }; P.days[d].minutes += 0.5;
  if (currentRoute && C.lessons[currentRoute]) { const lp = P.lessons[currentRoute] = P.lessons[currentRoute] || {}; lp.seconds = (lp.seconds || 0) + 30; }
  save();
}, 30000);

/* ───────────── course model ───────────── */
const ORDER = [], CH_OF = {};
C.chapters.forEach((ch, ci) => ch.lessons.forEach(id => { ORDER.push(id); CH_OF[id] = ci; }));
const lessonDone = id => !!(P.lessons[id] && P.lessons[id].completed);
const lessonStarted = id => !!(P.lessons[id] && (P.lessons[id].visited || P.lessons[id].completed));
const chapterPct = ch => ch.lessons.length ? Math.round(100 * ch.lessons.filter(lessonDone).length / ch.lessons.length) : 0;
const overallPct = () => ORDER.length ? Math.round(100 * ORDER.filter(lessonDone).length / ORDER.length) : 0;
const ROADMAP = (C.meta.roadmap || []).filter(r => !C.chapters.some(ch => ch.id === r.id));
function practiceOf(les) {
  const r = { qT: 0, qC: 0, eT: 0, eP: 0, pT: 0, pP: 0 };
  (les.blocks || []).forEach(b => {
    if (b.type === "quiz" && les.kind !== "test") { r.qT += b.questions.length; const s = P.quizzes[b.id]; r.qC += s && s.correct ? s.correct.length : 0; }
    if (b.type === "exercise") { r.eT++; if (P.exercises[b.id] && P.exercises[b.id].status === "passed") r.eP++; }
    if (b.type === "playground" && (b.expected_output != null || (b.must_contain && b.must_contain.length))) { r.pT++; if (P.playgrounds[b.id] && P.playgrounds[b.id].passed) r.pP++; }
  });
  return r;
}
function nextLessonId() { const first = ORDER.find(id => !lessonDone(id)); return (P.last && ORDER.indexOf(P.last) >= 0 && !lessonDone(P.last)) ? P.last : first; }
function streak() {
  let n = 0; const d = new Date();
  const key = x => x.getFullYear() + "-" + String(x.getMonth() + 1).padStart(2, "0") + "-" + String(x.getDate()).padStart(2, "0");
  if (!P.days[key(d)]) d.setDate(d.getDate() - 1);
  while (P.days[key(d)]) { n++; d.setDate(d.getDate() - 1); }
  return n;
}
function totals() {
  const r = { les: ORDER.filter(lessonDone).length, ex: 0, exP: 0, q: 0, qC: 0, pg: 0, pgP: 0, runs: 0, minutes: 0, tests: 0, testsP: 0 };
  ORDER.forEach(id => { const les = C.lessons[id]; const p = practiceOf(les); r.ex += p.eT; r.exP += p.eP; r.q += p.qT; r.qC += p.qC; r.pg += p.pT; r.pgP += p.pP; if (les.kind === "test") { r.tests++; const qb = les.blocks.find(b => b.type === "quiz"); if (qb && P.quizzes[qb.id] && P.quizzes[qb.id].passed) r.testsP++; } });
  Object.values(P.playgrounds).forEach(x => { r.runs += x.runs || 0; }); Object.values(P.exercises).forEach(x => { r.runs += x.attempts || 0; });
  Object.values(P.days).forEach(x => { r.minutes += x.minutes || 0; });
  return r;
}
function maybeAutoComplete(les) {
  if (!les || les.kind === "test" || lessonDone(les.id)) return;
  const p = practiceOf(les);
  if (p.qT + p.eT + p.pT === 0) return;
  if (p.qC === p.qT && p.eP === p.eT && p.pP === p.pT) completeLesson(les.id, true);
}
function completeLesson(id, auto) {
  P.lessons[id] = Object.assign(P.lessons[id] || {}, { completed: true, completedAt: new Date().toISOString() });
  log("lesson", id, auto ? "all practice done" : ""); save(); stamp(t("lessonDone"));
  if (typeof window.__drawFinish === "function") window.__drawFinish();
}

/* ───────────── feedback ───────────── */
function ask(msg, yes) {
  const close = () => { ov.remove(); document.removeEventListener("keydown", key); };
  const key = e => { if (e.key === "Escape") close(); };
  const ok = h("button", { class: "btn", type: "button", onclick: () => { close(); yes(); } }, t("yes"));
  const ov = h("div", { class: "dlg-ov", onclick: e => { if (e.target === ov) close(); } },
    h("div", { class: "dlg", role: "dialog", "aria-modal": "true" }, h("p", null, msg),
      h("div", { class: "dlg-act" }, h("button", { class: "btn ghost", type: "button", onclick: close }, t("no")), ok)));
  document.addEventListener("keydown", key); document.body.append(ov); ok.focus();
}
function toast(msg, icon) { const el = h("div", { class: "toast", role: "status" }, ic(icon || "check"), msg); document.body.append(el); setTimeout(() => el.remove(), 2600); }
function stamp(text) { if (reduceMotion()) { toast(text); return; } const el = h("div", { class: "stamp", "aria-hidden": "true" }, h("div", null, "✓ " + text)); document.body.append(el); setTimeout(() => el.remove(), 1400); }
function copyText(text, btn) {
  const done = () => { const o = btn.lastChild.textContent; btn.lastChild.textContent = t("copied"); setTimeout(() => { btn.lastChild.textContent = o; }, 1200); };
  if (navigator.clipboard && navigator.clipboard.writeText) navigator.clipboard.writeText(text).then(done, done); else done();
}
function sanitizeSvg(src) {
  const tpl = document.createElement("template"); tpl.innerHTML = String(src);
  tpl.content.querySelectorAll("script,foreignObject,iframe,object,embed").forEach(n => n.remove());
  tpl.content.querySelectorAll("*").forEach(n => [...n.attributes].forEach(a => { if (/^on/i.test(a.name) || (/^(xlink:)?href$/i.test(a.name) && /^\s*javascript:/i.test(a.value))) n.removeAttribute(a.name); }));
  return tpl.innerHTML;
}

/* ───────────── code runner: local rustc/cargo via serve.py, else play.rust-lang.org ───────────── */
const PLAY = "https://play.rust-lang.org/execute";
const runnerLabel = () => serverMode && serverCaps.rust ? "Local Rust" : serverMode ? "Rust Playground (no local Rust)" : "Rust Playground";
const usesLocal = () => serverMode && !!serverCaps.rust;
function inlineTests(lib, tests) {
  const re = /#\[cfg\(test\)\]\s*mod\s+tests\s*;/;
  const body = "#[cfg(test)]\nmod tests {\n" + tests.replace(/^/gm, "    ") + "\n}\n";
  return re.test(lib) ? lib.replace(re, body) : lib + "\n\n" + body;
}
async function playground(code, tests) {
  const body = { channel: "stable", mode: "debug", edition: "2024", crateType: tests ? "lib" : "bin", tests: !!tests, code, backtrace: false };
  const ctl = new AbortController(); const timer = setTimeout(() => ctl.abort(), 30000);
  try {
    const r = await fetch(PLAY, { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(body), signal: ctl.signal });
    if (!r.ok) throw new Error("HTTP " + r.status);
    const j = await r.json();
    const stderr = j.stderr || "", compileFailed = /error(\[E\d+\])?:/.test(stderr) && /could not compile/.test(stderr);
    return { ok: !!j.success, compiled: !compileFailed, stdout: j.stdout || "", stderr, timed_out: false, exit: j.exitDetail || "" };
  } finally { clearTimeout(timer); }
}
async function runProgram(code) {
  if (usesLocal()) {
    const r = await fetch("api/run", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ lang: "rust", code }) });
    const j = await r.json(); if (!r.ok) throw new Error(j.error || "server error");
    return { ok: j.ok, compiled: j.compiled, stdout: j.stdout || "", stderr: (j.compile_output || "") + (j.stderr || ""), timed_out: j.timed_out, via: "local" };
  }
  return Object.assign(await playground(code, false), { via: "play" });
}
async function runTests(dir, files, testsSrc) {
  if (usesLocal()) {
    const r = await fetch("api/run-tests", { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify({ dir, files }) });
    const j = await r.json(); if (!r.ok) throw new Error(j.error || "server error");
    return { ok: j.ok, output: j.output || "", timed_out: j.timed_out, via: "local" };
  }
  const lib = files["src/lib.rs"] || Object.values(files)[0] || "";
  const res = await playground(inlineTests(lib, testsSrc || ""), true);
  return { ok: res.ok, output: (res.stderr ? res.stderr + "\n" : "") + res.stdout, timed_out: false, via: "play" };
}
function parseTests(out) {
  const tests = []; const re = /^test (\S+) \.\.\. (ok|FAILED|ignored)/gm; let m;
  while ((m = re.exec(out))) tests.push({ name: m[1].replace(/^tests::/, ""), status: m[2] });
  const compileErr = /error(\[E\d+\])?:[\s\S]*could not compile/.test(out) || (/^error(\[E\d+\])?:/m.test(out) && !tests.length);
  return { tests, compileErr };
}
const normOut = s => String(s == null ? "" : s).replace(/\r/g, "").split("\n").map(l => l.replace(/\s+$/, "")).join("\n").replace(/^\n+|\n+$/g, "");

/* ───────────── editor: textarea over a highlighted layer ───────────── */
function makeEditor(onRun) {
  const ta = h("textarea", { spellcheck: "false", autocapitalize: "off", autocomplete: "off", "aria-label": "Code editor", wrap: "off" });
  const hl = h("pre", { class: "ed-hl", "aria-hidden": "true" }), gutter = h("div", { class: "ed-gutter", "aria-hidden": "true" });
  const box = h("div", { class: "ed" }, gutter, hl, ta);
  let lang = "rust", onChange = null;
  function paint() {
    const v = ta.value; const lines = highlightLines(v + (v.endsWith("\n") ? " " : ""), lang);
    hl.innerHTML = lines.join("\n") + "\n";
    gutter.textContent = lines.map((_, i) => i + 1).join("\n");
    ta.style.height = "auto"; ta.style.height = Math.max(ta.scrollHeight, box.clientHeight) + "px";
    ta.style.width = "auto"; ta.style.width = Math.max(hl.scrollWidth, box.clientWidth) + "px";
  }
  ta.addEventListener("input", () => { paint(); if (onChange) onChange(ta.value); });
  ta.addEventListener("keydown", e => {
    if ((e.ctrlKey || e.metaKey) && e.key === "Enter") { e.preventDefault(); onRun(); return; }
    if (ta.readOnly) return;
    const s = ta.selectionStart, en = ta.selectionEnd, v = ta.value;
    if (e.key === "Tab" && !e.shiftKey && s === en) { e.preventDefault(); document.execCommand("insertText", false, "    ") || (ta.setRangeText("    ", s, en, "end"), ta.dispatchEvent(new Event("input"))); }
    else if (e.key === "Tab" && e.shiftKey) { e.preventDefault(); const ls = v.lastIndexOf("\n", s - 1) + 1; const m = /^ {1,4}/.exec(v.slice(ls)); if (m) { ta.setRangeText("", ls, ls + m[0].length, "preserve"); ta.dispatchEvent(new Event("input")); } }
    else if (e.key === "Enter" && s === en) {
      const ls = v.lastIndexOf("\n", s - 1) + 1; const indent = /^\s*/.exec(v.slice(ls, s))[0]; const extra = /[{([]\s*$/.test(v.slice(ls, s)) ? "    " : "";
      e.preventDefault(); const ins = "\n" + indent + extra; if (!document.execCommand("insertText", false, ins)) { ta.setRangeText(ins, s, en, "end"); ta.dispatchEvent(new Event("input")); }
    } else if (e.key === "Escape") ta.blur();
  });
  new ResizeObserver(() => paint()).observe(box);
  return { el: box, ta, set(v, l, ro) { lang = l || "rust"; ta.value = v; ta.readOnly = !!ro; paint(); }, get: () => ta.value, onChange(fn) { onChange = fn; }, focus() { ta.focus(); } };
}

/* ───────────── the Verification Console ───────────── */
const Con = (() => {
  let item = null, busy = false, tab = 0;
  const editor = makeEditor(() => run());
  const head = h("div", { class: "cs-head" }, ic("term"), h("span", { class: "label" }, "Console"), h("span", { class: "runner" }, h("span", { class: "dot" }), h("span", { class: "rn" })),
    h("button", { class: "icon-btn cs-close", type: "button", "aria-label": "Close console", onclick: () => sheet(false) }, ic("close")));
  const title = h("div", { class: "cs-title" }), task = h("div", { class: "cs-task" });
  const tabs = h("div", { class: "cs-tabs", role: "tablist" });
  const edWrap = h("div", { class: "cs-editor" }, tabs, editor.el);
  const runBtn = h("button", { class: "btn", type: "button", onclick: () => run() }, ic("play"), h("span", null, "Run"));
  const resetBtn = h("button", { class: "btn ghost sm", type: "button", onclick: () => resetCode() }, ic("reset", "sm"), "Reset");
  const copyBtn = h("button", { class: "btn ghost sm", type: "button", onclick: () => copyText(editor.get(), copyBtn) }, ic("copy", "sm"), h("span", null, t("copy")));
  const actions = h("div", { class: "cs-actions" }, runBtn, resetBtn, copyBtn, h("span", { class: "kbd" }, "Ctrl / ⌘ + Enter"));
  const rows = h("div", { class: "cs-rows", role: "status", "aria-live": "polite" });
  const out = h("div", { class: "cs-out" }, h("pre"));
  const body = h("div", { style: "display:flex;flex-direction:column;flex:1;min-height:0" }, title, task, edWrap, actions, rows, out);
  const empty = h("div", { class: "cs-empty" }, h("h3", null, "Nothing loaded yet"), h("p", null, "Every runnable example in the lesson has an “Open in console” button. Load one, edit it, press Run."));
  const el = h("section", { class: "console", "aria-label": "Code console" }, plusMark("8px", "8px"), plusMark("8px", "8px", "right"), head, empty, body);
  const dock = h("aside", { class: "console-dock", id: "consoleDock" }, el);
  const peekT = h("span", { class: "pk-t" }, "Console");
  const peek = h("button", { class: "peek", type: "button", hidden: true, onclick: () => sheet(true) }, ic("term"), h("span", { class: "label" }, "Console"), peekT, h("span", { class: "btn sm" }, "Open"));
  body.style.display = "none";
  editor.onChange(v => { if (!item) return; const f = item.files[tab]; if (f && f.editable) { f.content = v; P.code[f.key] = v; save(); } });
  function sheet(open) { document.body.classList.toggle("console-open", open); if (open) setTimeout(() => editor.focus(), 350); }
  document.addEventListener("keydown", e => { if (e.key === "Escape" && document.body.classList.contains("console-open")) sheet(false); });
  function setRunner() { $(".rn", head).textContent = runnerLabel(); $(".dot", head).className = "dot" + (usesLocal() ? "" : " off"); }
  function drawTabs() {
    fill(tabs, item.files.map((f, i) => h("button", { type: "button", role: "tab", "aria-selected": String(i === tab), onclick: () => { tab = i; drawTabs(); editor.set(item.files[i].content, "rust", !item.files[i].editable); } }, f.path)),
      item.files[tab] && !item.files[tab].editable ? h("span", { class: "ro" }, "read only") : null);
  }
  function row(status, label, detail) {
    const icn = status === "pass" ? "check" : status === "fail" ? "cross" : status === "wait" ? "spin" : "dot";
    return h("div", { class: "cs-row enter " + status }, ic(icn), h("span", null, label), h("span", { class: "label" }, detail || ""));
  }
  function setRows(list) { fill(rows, list.map((r, i) => { const e = row(r[0], r[1], r[2]); e.style.animationDelay = (i * 70) + "ms"; return e; })); }
  function setOut(text, cls) { const pre = $("pre", out); pre.className = ""; pre.textContent = text; if (cls) pre.className = cls; }
  function load(it, opts) {
    item = it; tab = 0; empty.style.display = "none"; body.style.display = "flex";
    title.textContent = it.title || "Code";
    if (it.task) { task.style.display = ""; task.innerHTML = '<span class="label">Task</span>' + inline(it.task); } else task.style.display = "none";
    drawTabs(); editor.set(it.files[0].content, "rust", !it.files[0].editable);
    runBtn.lastChild.textContent = it.kind === "exercise" ? "Run tests" : "Run";
    setRows(it.kind === "exercise" ? [["idle", "Compile"], ["idle", "Tests"]] : it.check ? [["idle", "Compile"], ["idle", "Run"], ["idle", "Output check"]] : [["idle", "Compile"], ["idle", "Run"]]);
    setOut(it.lastOutput || "Output appears here.", it.lastOutput ? "" : "dim");
    peek.hidden = false; peekT.textContent = it.title || "Code"; setRunner();
    $$(".runnable.loaded").forEach(x => x.classList.remove("loaded")); if (it.card) it.card.classList.add("loaded");
    if (opts && opts.open && window.matchMedia("(max-width: 1180px)").matches) sheet(true);
    if (opts && opts.focus && !window.matchMedia("(max-width: 1180px)").matches) editor.focus();
  }
  function resetCode() {
    if (!item) return;
    ask("Put the original code back? Your edits in this file will be lost.", () => {
      item.files.forEach(f => { if (f.editable) { f.content = f.original; delete P.code[f.key]; } }); save();
      editor.set(item.files[tab].content, "rust", !item.files[tab].editable);
    });
  }
  async function run() {
    if (!item || busy) return; busy = true; runBtn.disabled = true;
    const it = item; const started = Date.now();
    try {
      if (it.kind === "exercise") {
        setRows([["wait", "Compiling and running tests", runnerLabel()]]); setOut("Running cargo test…", "dim");
        const files = {}; it.files.filter(f => f.editable).forEach(f => { files[f.path] = f.content; });
        const testsSrc = (it.files.find(f => !f.editable) || {}).content;
        const res = await runTests(it.dir, files, testsSrc);
        const pt = parseTests(res.output);
        const list = [[pt.compileErr ? "fail" : "pass", "Compile", pt.compileErr ? "errors" : ""]];
        pt.tests.forEach(x => list.push([x.status === "ok" ? "pass" : x.status === "ignored" ? "idle" : "fail", x.name.replace(/_/g, " "), x.status === "ok" ? "pass" : x.status]));
        if (res.timed_out) list.push(["fail", "Timed out", ""]);
        if (!pt.tests.length && !pt.compileErr) list.push([res.ok ? "pass" : "fail", "Tests", res.ok ? "pass" : "fail"]);
        list.push([res.ok ? "pass" : "fail", res.ok ? "All tests pass" : "Not passing yet", ((Date.now() - started) / 1000).toFixed(1) + "s"]);
        setRows(list); setOut(res.output.trim() || "(no output)", res.ok ? "" : "");
        it.lastOutput = res.output; if (it.onResult) it.onResult(res.ok, res);
      } else {
        setRows([["wait", "Compiling", runnerLabel()], ["idle", "Run"]].concat(it.check ? [["idle", "Output check"]] : [])); setOut("Compiling…", "dim");
        const res = await runProgram(editor.get());
        const list = [[res.compiled ? "pass" : "fail", "Compile", res.compiled ? "" : "errors"]];
        let passed = null;
        if (res.compiled) {
          list.push([res.ok ? "pass" : "fail", res.ok ? "Run" : (res.timed_out ? "Run timed out" : "Run stopped with an error"), ((Date.now() - started) / 1000).toFixed(1) + "s"]);
          if (it.check) {
            const o = normOut(res.stdout);
            passed = (it.check.expected == null || o === normOut(it.check.expected)) && (it.check.contains || []).every(s => o.indexOf(s) >= 0);
            list.push([passed ? "pass" : "fail", passed ? "Output matches the task" : "Output does not match yet", ""]);
          }
        } else if (it.check) list.push(["idle", "Output check", ""]);
        setRows(list);
        const text = (res.stderr ? res.stderr.trim() + (res.stdout ? "\n\n" : "") : "") + (res.stdout || "");
        setOut(text.trim() || "(the program printed nothing)");
        if (passed === false && it.check && it.check.expected != null) $("pre", out).append("\n\n— expected —\n" + it.check.expected);
        it.lastOutput = text; if (it.onResult) it.onResult(passed, res);
      }
    } catch (e) {
      setRows([["fail", usesLocal() ? "Could not reach the local helper" : "Could not reach play.rust-lang.org", ""]]);
      setOut(usesLocal() ? "The course helper (serve.py) did not answer. Is the terminal running start.sh still open?" :
        "Running code needs either the course helper (start.sh / start.bat with Rust installed) or an internet connection to play.rust-lang.org.\n\n" + (e.message || e), "err");
    } finally { busy = false; runBtn.disabled = false; }
  }
  return { dock, peek, load, run, setRunner, get item() { return item; }, clear() { item = null; empty.style.display = ""; body.style.display = "none"; peek.hidden = true; } };
})();

/* ───────────── blocks ───────────── */
function codeView(o) {
  const hl = new Set(o.highlight || []);
  const lines = highlightLines(o.code, o.lang || langOf(o.file));
  const copy = h("button", { type: "button", onclick: () => copyText(o.code, copy) }, ic("copy", "sm"), h("span", null, t("copy")));
  const tools = [copy];
  if (o.runnable) tools.unshift(h("button", { class: "load", type: "button", onclick: o.runnable }, ic("play", "sm"), h("span", null, t("tryIt"))));
  return h("div", { class: "code" }, h("div", { class: "code-head" }, h("span", { class: "fname" }, o.file || o.lang || ""), h("span", { class: "sp" }), tools),
    h("pre", null, h("code", null, lines.map((ln, i) => h("span", { class: "ln" + (hl.has(i + 1) ? " hl" : ""), html: ln })))));
}
const isRunnableRust = (code, lang) => /^(rust|rs)$/i.test(lang || "") && /fn\s+main\s*\(/.test(code || "");
const CALLOUT = { tip: ["bulb", "Tip"], note: ["info", "Note"], warning: ["alert", "Watch out"], analogy: ["puzzle", "Analogy"], key: ["key", "Remember"], example: ["search", "Example"] };

function renderBlock(b, les) {
  switch (b.type) {
    case "text": return h("div", { class: "block prose" }, b.title ? h("h3", null, b.title) : null, h("div", { html: md(b.md) }));
    case "callout": { const k = CALLOUT[b.kind] ? b.kind : "note"; return h("aside", { class: "block callout " + k }, h("div", { class: "c-h" }, ic(CALLOUT[k][0], "sm"), h("span", { class: "label" }, CALLOUT[k][1]), b.title ? h("b", null, b.title) : null), h("div", { class: "prose", html: md(b.md) })); }
    case "code": {
      const run = isRunnableRust(b.code, b.lang || langOf(b.file)) ? () => Con.load(scratchItem(b, les), { open: true, focus: true }) : null;
      const out = b.output ? h("div", { class: "out" }, h("span", { class: "label" }, b.output_label || t("output")), h("pre", null, b.output)) : null;
      const cv = codeView({ code: b.code, lang: b.lang, file: b.file, highlight: b.highlight, runnable: run }); if (out) cv.append(out);
      return h("div", { class: "block" }, b.title ? h("h3", { class: "block-h" }, b.title) : null, cv, b.caption ? h("div", { class: "caption", html: inline(b.caption) }) : null, b.explain ? h("div", { class: "prose explain", html: md(b.explain) }) : null);
    }
    case "stepper": return stepperBlock(b);
    case "derivation": return derivationBlock(b);
    case "reveal": return revealBlock(b);
    case "from_book": return h("figure", { class: "block frombook", style: "margin-left:auto;margin-right:auto" }, h("div", { class: "label" }, ic("book", "sm"), t("frombook") + (b.source ? " · " + b.source : "")), h("div", { class: "prose", html: md(b.md) }), b.code ? codeView({ code: b.code, lang: b.lang }) : null);
    case "table": return h("div", { class: "block" }, b.title ? h("h3", { class: "block-h" }, b.title) : null, h("div", { class: "tablewrap" }, h("table", null, h("thead", null, h("tr", null, b.headers.map(x => h("th", { html: inline(x) })))), h("tbody", null, b.rows.map(r => h("tr", null, r.map(c => h("td", { html: inline(c) }))))))));
    case "flow": return flowBlock(b);
    case "svg": return h("figure", { class: "block figure", style: "margin-left:auto;margin-right:auto" }, h("div", { html: sanitizeSvg(b.svg) }), b.caption ? h("figcaption", { html: inline(b.caption) }) : null);
    case "quiz": return quizBlock(b, les);
    case "exercise": return exerciseBlock(b, les);
    case "playground": return playgroundBlock(b, les);
    case "flashcards": return flashBlock(b);
    case "summary": return h("div", { class: "block summary" }, h("h3", null, b.title || t("summary")), h("ol", null, b.points.map(p => h("li", { html: inline(p) }))));
    default: return h("div", { class: "block" }, "[" + b.type + "]");
  }
}
function scratchItem(b, les) {
  const key = "code:" + les.id + ":" + (b.file || "") + ":" + b.code.length;
  const content = P.code[key] != null ? P.code[key] : b.code;
  return { kind: "code", title: b.title || (b.file ? b.file : "Example from the lesson"), files: [{ path: b.file && /\.rs$/.test(b.file) ? b.file : "main.rs", content, original: b.code, editable: true, key }] };
}

function stepControls(n, onStep) {
  let idx = 0;
  const ticks = h("div", { class: "ticks" }), meta = h("span", { class: "label", style: "color:var(--muted)" });
  const prev = h("button", { class: "btn ghost sm", type: "button", onclick: () => go(idx - 1, "prev") }, ic("chevL", "sm"), t("prev"));
  const next = h("button", { class: "btn sm", type: "button", onclick: () => go(idx + 1, "next") }, t("next"), ic("chev", "sm"));
  function go(i, dir) {
    idx = Math.max(0, Math.min(n - 1, i));
    fill(ticks, Array.from({ length: n }, (_, j) => h("button", { class: j === idx ? "on" : j < idx ? "past" : "", type: "button", "aria-label": t("step", j + 1, n), onclick: () => go(j, j > idx ? "next" : "prev") })));
    meta.textContent = t("step", idx + 1, n); prev.disabled = idx === 0; next.disabled = idx === n - 1; onStep(idx, dir);
  }
  return { foot: h("div", { class: "p-f" }, prev, next, meta, ticks), go, get idx() { return idx; } };
}
function stepperBlock(b) {
  const stage = h("div", { class: "p-b" });
  const ctl = stepControls(b.steps.length, (idx, dir) => {
    const s = b.steps[idx]; stage.className = "p-b"; void stage.offsetWidth; if (dir && !reduceMotion()) stage.classList.add("stage-anim-" + dir);
    const cv = s.code ? codeView({ code: s.code, lang: s.lang || b.lang, highlight: s.highlight, file: s.file }) : null;
    if (cv && s.output) cv.append(h("div", { class: "out" }, h("span", { class: "label" }, t("output")), h("pre", null, s.output)));
    fill(stage, h("div", { class: "step-title" }, s.title || ""), s.md ? h("div", { class: "prose", style: "margin-bottom:12px", html: md(s.md) }) : null, cv);
  });
  const root = h("div", { class: "block panel" }, h("div", { class: "p-h" }, ic("steps"), b.title || "Walkthrough", h("span", { class: "label" }, b.steps.length + " steps")), stage, ctl.foot);
  ctl.go(0); return root;
}
function derivationBlock(b) {
  const rel0 = b.rel || "="; let showAll = false;
  const rows = b.steps.map(s => h("div", { class: "dv-row" }, h("span", { class: "dv-lhs", html: s.lhs ? tex2mml(s.lhs, false) : "" }), h("span", { class: "dv-rel", html: tex2mml(s.rel || rel0, false) }), h("span", { class: "dv-rhs", html: tex2mml(s.rhs, false) }), h("div", { class: "dv-why", html: s.why ? md(s.why) : "" })));
  const sync = idx => rows.forEach((r, i) => { r.classList.toggle("hidden", !showAll && i > idx); r.classList.toggle("cur", !showAll && i === idx); });
  const ctl = stepControls(b.steps.length, idx => sync(idx));
  const all = h("button", { class: "btn ghost sm", type: "button", onclick: () => { showAll = !showAll; all.textContent = showAll ? t("stepByStep") : t("showAll"); sync(ctl.idx); } }, t("showAll"));
  ctl.foot.insertBefore(all, ctl.foot.children[2]);
  const root = h("div", { class: "block panel" }, h("div", { class: "p-h" }, ic("steps"), b.title || "Derivation"), h("div", { class: "p-b" }, b.intro ? h("div", { class: "prose", html: md(b.intro) }) : null, h("div", { class: "dv-rows" }, rows),
    b.result ? h("p", null, h("b", null, t("result") + ": "), h("span", { html: tex2mml(String(b.result), false) })) : null, b.note ? h("div", { class: "prose", html: md(b.note) }) : null), ctl.foot);
  ctl.go(0); return root;
}
function revealBlock(b) {
  const ans = h("div", { class: "answer prose", html: md(b.md) });
  const btn = h("button", { class: "btn ghost sm", type: "button", "aria-expanded": "false", onclick: () => { const open = !ans.isConnected; if (open) { root.append(ans); ans.classList.add("pop"); } else ans.remove(); btn.setAttribute("aria-expanded", String(open)); btn.lastChild.textContent = open ? t("hide") : t("reveal"); } }, ic("eye", "sm"), h("span", null, t("reveal")));
  const root = h("div", { class: "block reveal" }, h("div", { class: "c-h label", style: "color:var(--muted);margin-bottom:8px" }, "Predict"), h("div", { class: "prompt prose", html: md(b.prompt) }), h("p", { class: "caption" }, t("thinkFirst")), btn);
  return root;
}
function flowBlock(b) {
  let cur = -1; const nodes = [];
  const track = h("div", { class: "flow-track" }), detail = h("div", { class: "flow-detail prose" });
  b.nodes.forEach((n, i) => { const el = h("button", { class: "flow-node", type: "button", onclick: () => show(i) }, h("span", { class: "n label" }, String(i + 1).padStart(2, "0")), h("span", null, n.label)); nodes.push(el); track.append(el); });
  function show(i) { cur = i; nodes.forEach((n, j) => { n.classList.toggle("on", j <= i); n.classList.toggle("cur", j === i); }); detail.innerHTML = md(b.nodes[i].detail || ""); nxt.disabled = i >= b.nodes.length - 1; }
  const nxt = h("button", { class: "btn sm", type: "button", onclick: () => show(cur + 1) }, t("next"), ic("chev", "sm"));
  const rst = h("button", { class: "btn ghost sm", type: "button", onclick: () => { cur = -1; nodes.forEach(n => n.classList.remove("on", "cur")); detail.textContent = ""; nxt.disabled = false; } }, t("again"));
  return h("div", { class: "block panel" }, h("div", { class: "p-h" }, ic("flow"), b.title || "Process", h("span", { class: "label" }, b.nodes.length + " stages")), h("div", { class: "p-b" }, track, detail), h("div", { class: "p-f" }, nxt, rst));
}

/* ───────────── quizzes and tests ───────────── */
function rng(seed) { let a = 0; for (let i = 0; i < seed.length; i++) a = (a * 31 + seed.charCodeAt(i)) >>> 0; return () => { a |= 0; a = (a + 0x6D2B79F5) | 0; let x = Math.imul(a ^ (a >>> 15), 1 | a); x = (x + Math.imul(x ^ (x >>> 7), 61 | x)) ^ x; return ((x ^ (x >>> 14)) >>> 0) / 4294967296; }; }
const normTxt = (s, ci) => { s = String(s == null ? "" : s).trim().replace(/\s+/g, " "); return ci === false ? s : s.toLowerCase(); };
function qChoice(q, uid, multi) {
  const sel = new Set(), labels = [], inputs = [];
  const el = h("div", { class: "opts", role: multi ? "group" : "radiogroup" }, q.options.map((o, i) => {
    const inp = h("input", { type: multi ? "checkbox" : "radio", name: uid, onchange: () => { if (!multi) sel.clear(); inp.checked ? sel.add(i) : sel.delete(i); labels.forEach((l, j) => l.classList.toggle("sel", sel.has(j))); } });
    const lab = h("label", { class: "opt" }, inp, h("span", { html: inline(o) })); labels.push(lab); inputs.push(inp); return lab;
  }));
  const want = new Set([].concat(q.answer));
  return { el, answered: () => sel.size > 0, correct: () => sel.size === want.size && [...sel].every(x => want.has(x)), lock: v => inputs.forEach(i => { i.disabled = v; }),
    mark: () => labels.forEach((l, j) => { l.classList.remove("good", "bad"); if (sel.has(j)) l.classList.add(want.has(j) ? "good" : "bad"); }),
    reveal: () => labels.forEach((l, j) => { l.classList.remove("bad", "good"); if (want.has(j)) l.classList.add("good"); }),
    reset: () => { sel.clear(); inputs.forEach(i => { i.checked = false; }); labels.forEach(l => l.classList.remove("sel", "good", "bad")); } };
}
function numReadings(s) { s = String(s == null ? "" : s).trim().replace(/[\s ]+/g, ""); if (!s) return []; const out = [Number(s.replace(/,/g, ""))]; if (s.includes(",")) out.push(Number(s.replace(/\./g, "").replace(",", "."))); return out.filter(n => !Number.isNaN(n)); }
function numericMatches(input, answer, tol) { const sl = String(input).indexOf("/"); const c = sl < 0 ? numReadings(input) : [].concat(...numReadings(String(input).slice(0, sl)).map(n => numReadings(String(input).slice(sl + 1)).filter(d => d !== 0).map(d => n / d))); return c.some(v => Math.abs(v - answer) <= tol); }
function qText(q, numeric) {
  const inp = h("input", { type: "text", inputmode: numeric ? "decimal" : null, autocomplete: "off", spellcheck: "false", placeholder: q.placeholder || "Type your answer", "aria-label": t("yourAnswer") });
  const el = h("div", { class: "fill-in" }, inp, numeric && q.unit ? h("span", { class: "caption" }, " " + q.unit) : null);
  const acc = [].concat(q.answer).map(a => normTxt(a, q.ignore_case)), tol = Math.abs(q.tolerance || 0);
  return { el, input: inp, answered: () => inp.value.trim() !== "", correct: () => numeric ? numericMatches(inp.value, q.answer, tol) : acc.indexOf(normTxt(inp.value, q.ignore_case)) >= 0, lock: v => { inp.disabled = v; },
    mark: ok => { el.classList.remove("good", "bad"); el.classList.add(ok ? "good" : "bad"); }, reveal: () => { inp.value = String([].concat(q.answer)[0]); el.classList.remove("bad"); el.classList.add("good"); },
    reset: () => { inp.value = ""; el.classList.remove("good", "bad"); } };
}
function qOrder(q, uid) {
  const n = q.items.length; let order = [...Array(n).keys()]; const rnd = rng(uid);
  const shuffle = () => { for (let k = 0; k < 6; k++) { for (let i = n - 1; i > 0; i--) { const j = Math.floor(rnd() * (i + 1)); [order[i], order[j]] = [order[j], order[i]]; } if (order.some((v, i) => v !== i)) break; } };
  shuffle(); const ul = h("ol", { class: "order-list" }); let locked = false;
  const draw = () => ul.replaceChildren(...order.map((idx, pos) => h("li", null, h("span", { html: inline(q.items[idx]) }),
    h("button", { class: "btn ghost sm", type: "button", disabled: locked || pos === 0, "aria-label": t("moveUp"), onclick: () => { [order[pos - 1], order[pos]] = [order[pos], order[pos - 1]]; draw(); ul.children[pos - 1].querySelector("button").focus(); } }, "↑"),
    h("button", { class: "btn ghost sm", type: "button", disabled: locked || pos === n - 1, "aria-label": t("moveDown"), onclick: () => { [order[pos + 1], order[pos]] = [order[pos], order[pos + 1]]; draw(); const b2 = ul.children[pos + 1].querySelectorAll("button")[1]; if (!b2.disabled) b2.focus(); } }, "↓"))));
  draw();
  return { el: ul, answered: () => true, correct: () => order.every((v, i) => v === i), lock: v => { locked = v; draw(); }, mark: ok => { ul.classList.remove("good", "bad"); ul.classList.add(ok ? "good" : "bad"); },
    reveal: () => { order = [...Array(n).keys()]; ul.classList.remove("bad"); ul.classList.add("good"); draw(); }, reset: () => { order = [...Array(n).keys()]; shuffle(); ul.classList.remove("good", "bad"); draw(); } };
}
function makeQuestion(q0, qi, quizId) {
  const q = q0.type === "truefalse" ? Object.assign({}, q0, { type: "single", options: [t("tTrue"), t("tFalse")], answer: q0.answer ? 0 : 1 }) : q0;
  const uid = quizId + "-" + qi;
  const impl = q.type === "multi" ? qChoice(q, uid, true) : q.type === "fill" ? qText(q, false) : q.type === "numeric" ? qText(q, true) : q.type === "order" ? qOrder(q, uid) : qChoice(q, uid, false);
  const fb = h("div", { class: "q-feedback", role: "status" });
  const root = h("div", { class: "question" }, h("div", { class: "q-head" }, h("span", { class: "q-num" }, String(qi + 1).padStart(2, "0")), h("div", { class: "q-text prose", html: md(q.q) })),
    q.code ? codeView({ code: q.code, lang: q.lang || "rust" }) : null, impl.el);
  const hints = q.hints || []; let used = 0; const hintBox = h("div", { class: "hints" }); let hintBtn = null;
  if (hints.length) {
    const label = () => { hintBtn.lastChild.textContent = t("hint") + " " + used + "/" + hints.length; hintBtn.disabled = used >= hints.length; };
    hintBtn = h("button", { class: "btn ghost sm", type: "button", onclick: () => { hintBox.append(h("div", { class: "hint pop" }, ic("bulb", "sm"), h("span", { html: inline(hints[used]) }))); used++; label(); } }, ic("bulb", "sm"), h("span"));
    label();
  }
  const actions = h("div", { class: "q-actions" });
  const api = { el: root, impl, actions, hintBtn, answered: impl.answered, hintsUsed: () => used };
  api.show = (ok, reveal) => { impl.mark(ok); if (reveal && !ok) impl.reveal();
    fb.replaceChildren(h("div", { class: "fb pop " + (ok ? "good" : "bad") }, h("strong", null, ic(ok ? "check" : "cross", "sm"), ok ? t("correct") : t("wrong")), (ok || reveal) && q.explanation ? h("div", { class: "prose", html: md(q.explanation) }) : (!ok ? h("div", null, "Use a hint or try again.") : null))); };
  api.clear = () => { fb.replaceChildren(); impl.reset(); impl.lock(false); };
  if (impl.input) impl.input.addEventListener("keydown", e => { if (e.key === "Enter") { e.preventDefault(); const c = actions.querySelector("button"); if (c && !c.disabled) c.click(); } });
  root.append(actions, hintBox, fb); if (hintBtn) actions.append(hintBtn);
  return api;
}
function quizBlock(b, les) {
  const isTest = les.kind === "test" || b.mode === "test", need = b.pass_score != null ? b.pass_score : 0.7;
  const st = P.quizzes[b.id] || (P.quizzes[b.id] = { attempts: 0, best: 0, correct: [], passed: false });
  const badge = h("span", { class: "tag" });
  const showBadge = () => {
    if (isTest) { badge.className = "tag " + (st.passed ? "pass" : st.attempts ? "fail" : ""); badge.textContent = st.attempts ? t("best") + " " + Math.round(st.best * 100) + "%" : t("needScore", Math.round(need * 100)); }
    else { badge.className = "tag " + (st.correct.length === b.questions.length ? "pass" : ""); badge.textContent = st.correct.length + " / " + b.questions.length + " correct"; }
  };
  const root = h("section", { class: "block quiz" + (isTest ? " is-test" : "") }, h("div", { class: "p-h" }, ic(isTest ? "flag" : "list"), b.title || (isTest ? t("test") : "Quiz"), badge));
  showBadge();
  if (b.intro) root.append(h("div", { class: "quiz-intro prose", html: md(b.intro) }));
  const body = h("div"); root.append(body); let qs = [];
  function build() {
    qs = b.questions.map((q, i) => makeQuestion(q, i, b.id)); body.replaceChildren(...qs.map(x => x.el));
    if (isTest) return;
    qs.forEach((x, i) => {
      const chk = h("button", { class: "btn sm", type: "button" }, t("check")), retry = h("button", { class: "btn ghost sm", type: "button", hidden: true }, t("retry")), rev = h("button", { class: "btn ghost sm", type: "button", hidden: true }, t("showAnswer"));
      if (st.correct.indexOf(i) >= 0) { const done = h("span", { class: "tag pass" }, ic("check", "sm"), "Answered correctly before"); x.actions.prepend(done); }
      chk.onclick = () => {
        if (!x.answered()) { toast("Choose or type an answer first.", "info"); return; }
        const ok = x.impl.correct(); st.attempts++;
        if (ok) { x.impl.lock(true); x.show(true); chk.disabled = true; retry.hidden = rev.hidden = true; if (st.correct.indexOf(i) < 0) { st.correct.push(i); log("quiz", b.id, "q" + (i + 1)); } }
        else { x.show(false); retry.hidden = rev.hidden = false; }
        showBadge(); save(); maybeAutoComplete(les); if (window.__drawFinish) window.__drawFinish();
      };
      retry.onclick = () => { x.clear(); chk.disabled = false; retry.hidden = rev.hidden = true; };
      rev.onclick = () => { x.impl.lock(true); x.show(false, true); chk.disabled = true; rev.hidden = true; retry.hidden = false; };
      x.actions.prepend(chk, retry, rev);
    });
  }
  build();
  if (isTest) {
    const res = h("div", { role: "status" }), go = h("button", { class: "btn", type: "button" }, ic("flag", "sm"), t("checkTest")), again = h("button", { class: "btn ghost", type: "button", hidden: true }, ic("reset", "sm"), t("retry"));
    const grade = () => {
      let right = 0; qs.forEach(x => { const ok = x.answered() && x.impl.correct(); if (ok) right++; x.impl.lock(true); x.show(ok, true); });
      const frac = right / qs.length; st.attempts++; st.best = Math.max(st.best, frac); st.last = frac; st.history = (st.history || []).concat([{ at: new Date().toISOString(), score: frac }]).slice(-20);
      const pass = frac >= need; log("test", b.id, Math.round(frac * 100) + "%");
      if (pass) { st.passed = true; P.lessons[les.id] = Object.assign(P.lessons[les.id] || {}, { completed: true, completedAt: new Date().toISOString() }); stamp(t("passed")); }
      fill(res, h("div", { class: "scorebar " + (pass ? "pass" : "fail") }, h("span", { class: "label" }, t("score")), h("b", null, right + " / " + qs.length + " · " + Math.round(frac * 100) + "%"),
        h("p", { style: "margin:6px 0 0" }, pass ? "Passed. Every answer below now shows its explanation; read the ones you missed." : "Not passed yet (" + t("needScore", Math.round(need * 100)) + "). Read the explanations below, revisit the lessons, and try again; your best score is kept.")));
      go.hidden = true; again.hidden = false; showBadge(); save(); res.scrollIntoView({ behavior: reduceMotion() ? "auto" : "smooth", block: "center" });
      if (window.__drawFinish) window.__drawFinish();
    };
    go.onclick = () => { const miss = qs.filter(x => !x.answered()).length; if (miss) ask(t("unanswered", miss), grade); else grade(); };
    again.onclick = () => { build(); res.replaceChildren(); go.hidden = false; again.hidden = true; };
    root.append(h("div", { class: "quiz-foot" }, go, again), res);
  }
  return root;
}

/* ───────────── playground + exercise (load into the console) ───────────── */
function playgroundBlock(b, les) {
  const st = P.playgrounds[b.id] || (P.playgrounds[b.id] = { runs: 0, passed: false });
  const check = b.expected_output != null || (b.must_contain && b.must_contain.length) ? { expected: b.expected_output, contains: b.must_contain } : null;
  const key = "pg:" + b.id;
  const tag = h("span", { class: "tag" });
  const setTag = () => { if (check) { tag.className = "tag " + (st.passed ? "pass" : ""); fill(tag, st.passed ? [ic("check", "sm"), "Passed"] : "Challenge"); } else { tag.className = "tag blue"; tag.textContent = st.runs ? "Ran " + st.runs + "×" : "Playground"; } };
  setTag();
  const hints = b.hints || []; let used = 0; const hintBox = h("div");
  const hintBtn = hints.length ? h("button", { class: "btn ghost sm", type: "button", onclick: () => { hintBox.append(h("div", { class: "hint pop" }, ic("bulb", "sm"), h("span", { html: inline(hints[used]) }))); used++; hintBtn.lastChild.textContent = t("hint") + " " + used + "/" + hints.length; hintBtn.disabled = used >= hints.length; } }, ic("bulb", "sm"), h("span", null, t("hint") + " 0/" + hints.length)) : null;
  const card = h("section", { class: "block runnable", id: "b-" + b.id });
  const item = () => ({
    kind: "play", id: b.id, card, title: b.title || "Playground", task: b.task, check,
    files: [{ path: "main.rs", content: P.code[key] != null ? P.code[key] : b.code, original: b.code, editable: true, key }],
    onResult: (passed) => { st.runs++; st.lastRun = new Date().toISOString(); if (passed === true && !st.passed) { st.passed = true; log("playground", b.id, "passed"); stamp("Verified"); } else log("run", b.id); setTag(); save(); maybeAutoComplete(les); if (window.__drawFinish) window.__drawFinish(); }
  });
  const open = h("button", { class: "btn", type: "button", onclick: () => Con.load(item(), { open: true, focus: true }) }, ic("term", "sm"), t("loadConsole"));
  fill(card, h("div", { class: "p-h" }, ic(check ? "flag" : "term"), b.title || "Playground", tag),
    h("div", { class: "p-b" }, b.md ? h("div", { class: "prose", style: "margin-bottom:14px", html: md(b.md) }) : null,
      b.task ? h("div", { class: "task" }, h("span", { class: "label" }, "Task"), h("div", { class: "prose", html: md(b.task) })) : null,
      codeView({ code: P.code[key] != null ? P.code[key] : b.code, lang: "rust", file: "main.rs" }), hintBox),
    h("div", { class: "p-f" }, open, hintBtn));
  card.__load = () => Con.load(item());
  return card;
}
function exerciseBlock(b, les) {
  const st = P.exercises[b.id] || (P.exercises[b.id] = { status: "todo", attempts: 0, hints: 0, checked: [] });
  const tag = h("span", { class: "tag" });
  const setTag = () => { tag.className = "tag " + (st.status === "passed" ? "pass" : st.status === "failed" ? "fail" : ""); fill(tag, st.status === "passed" ? [ic("check", "sm"), t("stPassed")] : st.status === "failed" ? t("stFailed") : t("stTodo")); };
  setTag();
  const card = h("section", { class: "block runnable", id: "b-" + b.id });
  const mode = b.mode === "tests" ? "tests" : "selfcheck";
  const parts = [];
  parts.push(h("div", { class: "prose", html: md(b.goal) }));
  if (b.behavior) parts.push(h("div", { class: "ex-section" }, h("span", { class: "label" }, t("behavior")), h("div", { class: "prose", html: md(b.behavior) })));
  if (b.examples && b.examples.length) {
    const hasIn = b.examples.some(e => e.input != null), hasNote = b.examples.some(e => e.note);
    parts.push(h("div", { class: "ex-section" }, h("span", { class: "label" }, t("examples")), h("div", { class: "tablewrap" }, h("table", null,
      h("thead", null, h("tr", null, hasIn ? h("th", null, t("input")) : null, h("th", null, t("output")), hasNote ? h("th", null, "Note") : null)),
      h("tbody", null, b.examples.map(e => h("tr", null, hasIn ? h("td", null, h("code", null, e.input == null ? "" : e.input)) : null, h("td", null, h("code", null, e.output == null ? "" : e.output)), hasNote ? h("td", { html: inline(e.note || "") }) : null)))))));
  }
  const foot = h("div", { class: "p-f" });
  if (mode === "tests") {
    const f = b.files || {};
    const editable = (f.starter || []).filter(x => /\.rs$/.test(x.path));
    const item = () => ({
      kind: "exercise", id: b.id, dir: b.dir, card, title: b.title, task: b.goal,
      files: editable.map(x => { const key = "ex:" + b.id + ":" + x.path; return { path: x.path, content: P.code[key] != null ? P.code[key] : x.content, original: x.content, editable: true, key }; })
        .concat((f.tests || []).map(x => ({ path: x.path, content: x.content, original: x.content, editable: false }))),
      onResult: ok => { st.attempts++; st.lastRun = new Date().toISOString(); const was = st.status; st.status = ok ? "passed" : "failed"; if (ok && was !== "passed") { log("exercise", b.id, "passed"); stamp("Tests pass"); } else log("run", b.id, ok ? "pass" : "fail"); setTag(); save(); maybeAutoComplete(les); if (window.__drawFinish) window.__drawFinish(); }
    });
    card.__load = () => Con.load(item());
    foot.append(h("button", { class: "btn", type: "button", onclick: () => Con.load(item(), { open: true, focus: true }) }, ic("term", "sm"), "Open in console"));
    parts.push(h("details", { class: "files" }, h("summary", null, t("whereToWork")),
      h("div", { style: "padding:14px" }, h("p", { class: "caption", style: "margin-top:0" }, serverMode ? "Edits you make in the console are saved straight into this folder, so you can switch between the page and your editor." : "Started through start.sh / start.bat, the console saves your edits into this folder."),
        h("div", { class: "cmdline" }, h("span", null, "cd exercises/" + b.dir + "\n" + (b.command_display || "cargo test")), (() => { const c = h("button", { class: "btn ghost sm", type: "button", style: "color:#fff;border-color:rgba(255,255,255,.4)", onclick: () => copyText("cd exercises/" + b.dir + " && " + b.command_display, c) }, ic("copy", "sm"), h("span", null, t("copy"))); return c; })()))));
  } else if (b.checklist && b.checklist.length) {
    parts.push(h("div", { class: "ex-section" }, h("span", { class: "label" }, t("checklist")), h("ul", { class: "checklist" }, b.checklist.map((c, i) => h("li", null, h("label", null,
      h("input", Object.assign({ type: "checkbox", onchange: e => { const k = st.checked.indexOf(i); if (e.target.checked && k < 0) st.checked.push(i); if (!e.target.checked && k >= 0) st.checked.splice(k, 1); if (st.checked.length === b.checklist.length && st.status !== "passed") { st.status = "passed"; log("exercise", b.id, "checklist"); stamp("Done"); setTag(); maybeAutoComplete(les); } save(); if (window.__drawFinish) window.__drawFinish(); } }, st.checked.indexOf(i) >= 0 ? { checked: true } : {})),
      h("span", { html: inline(c) })))))));
    const done = h("button", { class: "btn ghost sm", type: "button", onclick: () => { st.status = st.status === "passed" ? "todo" : "passed"; if (st.status === "passed") log("exercise", b.id, "marked done"); setTag(); save(); maybeAutoComplete(les); done.textContent = st.status === "passed" ? t("markUndone") : t("markDone"); if (window.__drawFinish) window.__drawFinish(); } }, st.status === "passed" ? t("markUndone") : t("markDone"));
    foot.append(done);
  }
  const hints = b.hints || []; const hintBox = h("div", { class: "ex-section" });
  const drawHint = i => hintBox.append(h("div", { class: "hint" }, ic("bulb", "sm"), h("span", { html: inline(hints[i]) })));
  for (let i = 0; i < Math.min(st.hints || 0, hints.length); i++) drawHint(i);
  if (hints.length) {
    const hb = h("button", { class: "btn ghost sm", type: "button", onclick: () => { drawHint(st.hints); st.hints++; lab(); save(); } }, ic("bulb", "sm"), h("span"));
    const lab = () => { hb.lastChild.textContent = t("hint") + " " + Math.min(st.hints, hints.length) + "/" + hints.length; hb.disabled = st.hints >= hints.length; }; lab(); foot.append(hb);
  }
  const solBox = h("div");
  const sol = mode === "tests" ? ((b.files && b.files.solution) || []) : (typeof b.solution === "string" ? [{ path: b.solution_file || "", content: b.solution, lang: b.solution_lang || "rust" }] : []);
  if (sol.length || b.solution_explain) {
    const sb = h("button", { class: "btn ghost sm", type: "button" }, ic("eye", "sm"), h("span", null, t("showSolution")));
    sb.onclick = () => {
      if (solBox.childNodes.length) { solBox.replaceChildren(); sb.lastChild.textContent = t("showSolution"); return; }
      ask(t("confirmSolution"), () => { st.sawSolution = true; save(); fill(solBox, h("div", { class: "ex-section" }, h("span", { class: "label" }, t("solution")), sol.map(x => codeView({ code: x.content, file: x.path, lang: x.lang || langOf(x.path) || "rust" })), b.solution_explain ? h("div", { class: "prose explain", html: md(b.solution_explain) }) : null)); sb.lastChild.textContent = t("hideSolution"); });
    };
    foot.append(sb);
  }
  fill(card, h("div", { class: "p-h" }, ic("tool"), t("exercise") + ": " + b.title, b.difficulty ? h("span", { class: "tag", style: "margin-left:auto" }, b.difficulty) : null, tag),
    h("div", { class: "p-b" }, parts, hintBox, solBox), foot);
  if (b.difficulty) tag.style.marginLeft = "0";
  return card;
}
function flashBlock(b) {
  const st = P.cards[b.id] || (P.cards[b.id] = { known: [] });
  let queue = b.cards.map((_, i) => i).filter(i => st.known.indexOf(i) < 0), flipped = false;
  const stage = h("div", { class: "p-b" }), meta = h("span", { class: "label", style: "color:var(--muted);margin-left:auto" });
  function draw() {
    flipped = false;
    if (!queue.length) { meta.textContent = b.cards.length + " / " + b.cards.length; fill(stage, h("p", { style: "margin:0" }, t("allKnown")), h("button", { class: "btn ghost sm", type: "button", onclick: () => { st.known = []; queue = b.cards.map((_, i) => i); save(); draw(); } }, ic("reset", "sm"), t("again"))); return; }
    const c = b.cards[queue[0]];
    const card = h("button", { class: "card", type: "button", "aria-label": "Flashcard, press to flip", onclick: () => { flipped = !flipped; card.classList.toggle("flipped", flipped); } }, h("div", { class: "card-in" }, h("div", { class: "face front", html: inline(c.front) }), h("div", { class: "face back", html: inline(c.back) })));
    meta.textContent = (b.cards.length - queue.length) + " / " + b.cards.length + " known";
    fill(stage, card, h("div", { class: "flash-meta" }, t("cardsLeft", queue.length) + " · " + t("flipHint")),
      h("div", { style: "display:flex;gap:8px" }, h("button", { class: "btn sm", type: "button", onclick: () => { st.known.push(queue.shift()); save(); draw(); } }, ic("check", "sm"), t("know")), h("button", { class: "btn ghost sm", type: "button", onclick: () => { queue.push(queue.shift()); draw(); } }, ic("reset", "sm"), t("notYet"))));
  }
  const root = h("div", { class: "block panel flash" }, h("div", { class: "p-h" }, ic("cards"), b.title || "Flashcards", meta), stage); draw(); return root;
}

/* ───────────── pages ───────────── */
const main = () => $("#main");
let currentRoute = "home", observer = null;
function armReveal(container) {
  const blocks = container.querySelectorAll(".block");
  if (!("IntersectionObserver" in window) || reduceMotion()) return;
  if (observer) observer.disconnect();
  observer = new IntersectionObserver(es => es.forEach(e => { if (e.isIntersecting) { e.target.classList.add("in"); observer.unobserve(e.target); } }), { rootMargin: "0px 0px -40px 0px" });
  blocks.forEach(b => { const r = b.getBoundingClientRect(); if (r.top > innerHeight) { b.classList.add("rv"); observer.observe(b); } });
}
function fmtMin(m) { m = Math.round(m); return m < 60 ? m + " min" : Math.floor(m / 60) + " h " + (m % 60) + " min"; }
function ago(iso) { const s = (Date.now() - new Date(iso)) / 1000; if (s < 60) return "just now"; if (s < 3600) return Math.round(s / 60) + " min ago"; if (s < 86400) return Math.round(s / 3600) + " h ago"; return Math.round(s / 86400) + " d ago"; }
function activityText(a) {
  const les = C.lessons[a.id], title = les ? les.title : a.id;
  switch (a.kind) {
    case "lesson": return ["check", "Completed lesson “" + title + "”"];
    case "test": return ["flag", "Chapter test scored " + a.detail];
    case "exercise": return ["tool", "Exercise " + a.id + " " + (a.detail === "passed" ? "passed its tests" : a.detail)];
    case "playground": return ["term", "Challenge " + a.id + " verified"];
    case "quiz": return ["list", "Answered " + a.id + " " + a.detail + " correctly"];
    case "run": return ["play", "Ran " + a.id];
    default: return ["dot", a.kind + " " + a.id];
  }
}
function weekBars() {
  const days = []; const d = new Date(); d.setDate(d.getDate() - 6);
  for (let i = 0; i < 7; i++) { const k = d.getFullYear() + "-" + String(d.getMonth() + 1).padStart(2, "0") + "-" + String(d.getDate()).padStart(2, "0"); days.push({ k, label: d.toLocaleDateString(undefined, { weekday: "short" }), m: (P.days[k] && P.days[k].minutes) || 0 }); d.setDate(d.getDate() + 1); }
  const max = Math.max(15, ...days.map(x => x.m));
  return h("div", { class: "week", role: "img", "aria-label": "Minutes studied in the last 7 days: " + days.map(x => x.label + " " + Math.round(x.m)).join(", ") }, days.map(x => h("div", null, h("span", null, Math.round(x.m) || ""), h("i", { class: x.m ? "" : "zero", style: "height:" + Math.max(2, 100 * x.m / max) + "%" }), h("span", null, x.label))));
}
function ledger() {
  const rows = C.chapters.map((ch, ci) => {
    const isCur = P.last && CH_OF[P.last] === ci;
    return h("div", { class: "ledger-row" + (isCur ? " cur" : "") }, h("div", { class: "ledger-num" }, String(ci + 1).padStart(2, "0")),
      h("div", { class: "ledger-title" }, h("a", { href: "#/lesson/" + ch.lessons[0] }, ch.title), ch.summary ? h("small", null, ch.summary) : null),
      h("div", { class: "ledger-cells" }, ch.lessons.map(id => { const les = C.lessons[id]; return h("a", { class: "cell" + (lessonDone(id) ? " done" : lessonStarted(id) ? " started" : "") + (les.kind === "test" ? " gate" : ""), href: "#/lesson/" + id, title: les.title + (lessonDone(id) ? " (complete)" : ""), "aria-label": les.title + (lessonDone(id) ? ", complete" : lessonStarted(id) ? ", started" : "") }); })),
      h("div", { class: "ledger-pct" }, chapterPct(ch) + "%"));
  });
  ROADMAP.forEach((r, i) => rows.push(h("div", { class: "ledger-row pending" }, h("div", { class: "ledger-num" }, String(C.chapters.length + i + 1).padStart(2, "0")),
    h("div", { class: "ledger-title" }, h("a", { href: "#/progress", "aria-disabled": "true", tabindex: "-1" }, r.title), h("small", null, "Being written")), h("div", { class: "ledger-cells" }, h("span", { class: "cell locked" }), h("span", { class: "cell locked" }), h("span", { class: "cell locked" }), h("span", { class: "cell locked gate" })), h("div", { class: "ledger-pct" }, "–"))));
  return h("div", { class: "ledger" }, rows);
}
function renderHome() {
  const el = main(); const tot = totals(); const nid = nextLessonId(); const nles = nid ? C.lessons[nid] : null; const pct = overallPct();
  const nch = nles ? C.chapters[CH_OF[nid]] : null;
  const firstVisit = !P.started;
  const statement = firstVisit ? h("h1", { class: "display" }, "Learn Rust by ", h("em", null, "running it"), ", one verified step at a time.")
    : pct === 100 ? h("h1", { class: "display" }, "Every lesson ", h("em", null, "verified"), ".")
    : h("h1", { class: "display" }, "Chapter " + (CH_OF[nid] + 1) + ": ", h("em", null, nch.title), ".");
  const lead = firstVisit ? (C.meta.subtitle || "") : pct === 100 ? "You have completed every written lesson. New chapters appear here as they are added." : tot.les + " of " + ORDER.length + " lessons complete. Your code, answers and scores are saved " + (serverMode ? "to progress/progress.json." : "in this browser.");
  const resume = nles ? h("div", { class: "resume-card" }, h("span", { class: "label" }, firstVisit ? "First lesson" : "Up next"), h("h2", null, nles.title), h("p", null, nles.summary || ""),
    h("div", { class: "resume-meter" }, h("div", { class: "meter" }, h("i", { style: "width:" + chapterPct(nch) + "%" })), h("span", { class: "meter-num" }, chapterPct(nch) + "% of chapter")),
    h("a", { class: "btn solid-light", href: "#/lesson/" + nid }, firstVisit ? t("start") : t("resume"), ic("chev", "sm"))) : null;
  const band = h("section", { class: "band" }, plusMark("24px", "24px"), plusMark("24px", "24px", "right"),
    h("div", { class: "band-inner" }, h("div", null, statement, h("p", { class: "band-lead" }, lead), h("div", { class: "band-actions" }, !resume && nid ? h("a", { class: "btn solid-light", href: "#/lesson/" + nid }, firstVisit ? t("start") : t("resume"), ic("chev", "sm")) : null, h("a", { class: "btn on-dark", href: "#/progress" }, "See progress"), h("a", { class: "btn on-dark", href: "#/glossary" }, "Glossary"))), resume));
  const tile = (label, val, sub) => h("div", { class: "tile" }, h("span", { class: "label" }, label), h("b", null, val), sub ? h("small", null, sub) : null);
  const tiles = h("div", { class: "tiles" }, tile("Lessons", tot.les + "/" + ORDER.length, pct + "% of written lessons"), tile("Exercises", tot.exP + "/" + tot.ex, "passing their tests"),
    tile("Challenges", tot.pgP + "/" + tot.pg, "outputs verified"), tile("Quiz answers", tot.qC + "/" + tot.q, "answered correctly"), tile("Chapter tests", tot.testsP + "/" + tot.tests, "passed"), tile("Streak", streak() + (streak() === 1 ? " day" : " days"), fmtMin(tot.minutes) + " studied"));
  const recent = P.activity.slice(0, 6);
  fill(el, band, h("div", { class: "wrap" }, tiles,
    h("section", { class: "section" }, h("div", { class: "section-head" }, h("h2", { class: "h2" }, "The course"), h("p", null, (C.chapters.length + ROADMAP.length) + " chapters from first program to systems programming. Each square is a lesson; the framed square is the chapter test.")),
      ledger(), h("div", { class: "legend" }, h("span", null, h("i", { class: "cell" }), "Not started"), h("span", null, h("i", { class: "cell started" }), "Started"), h("span", null, h("i", { class: "cell done" }), "Complete"), h("span", null, h("i", { class: "cell gate" }), "Chapter test"))),
    h("section", { class: "section" }, h("div", { class: "section-head" }, h("h2", { class: "h2" }, "Your study log")),
      h("div", { class: "two-col" }, h("div", null, h("span", { class: "label", style: "color:var(--muted)" }, "Last 7 days, minutes"), weekBars()),
        h("div", null, h("span", { class: "label", style: "color:var(--muted)" }, "Recent activity"), recent.length ? h("ul", { class: "activity" }, recent.map(a => { const [i2, txt] = activityText(a); return h("li", null, h("span", { class: "ic" }, ic(i2, "sm")), h("span", null, txt), h("time", { datetime: a.at }, ago(a.at))); }))
          : h("p", { class: "empty-note" }, "Nothing yet. Your runs, answers and completed lessons will be listed here."))))),
    footer());
}
function footer() {
  const src = C.meta.source || {};
  return h("footer", { class: "footer" }, h("span", null, "Built from: " + (src.title || "")), h("span", null, serverMode ? "Progress file: progress/progress.json" : "Progress: this browser · start the course with start.sh to save to a file"));
}
function renderProgress() {
  const el = main(); const tot = totals();
  const rows = C.chapters.map((ch, ci) => {
    const lessons = ch.lessons.map(id => C.lessons[id]); let qT = 0, qC = 0, eT = 0, eP = 0, sec = 0, test = null;
    lessons.forEach(l => { const p = practiceOf(l); qT += p.qT; qC += p.qC; eT += p.eT; eP += p.eP; sec += (P.lessons[l.id] && P.lessons[l.id].seconds) || 0; if (l.kind === "test") { const qb = l.blocks.find(b => b.type === "quiz"); test = qb ? P.quizzes[qb.id] : null; } });
    const pc = chapterPct(ch); const bar = p => h("span", { class: "bar-inline" }, h("i", { style: "width:" + p + "%" }));
    return h("tr", null, h("td", null, h("a", { href: "#/lesson/" + ch.lessons[0] }, (ci + 1) + ". " + ch.title)), h("td", { class: "num" }, bar(pc), pc + "%"),
      h("td", { class: "num" }, qT ? qC + "/" + qT : "–"), h("td", { class: "num" }, eT ? eP + "/" + eT : "–"),
      h("td", { class: "num" }, test && test.attempts ? Math.round(test.best * 100) + "% " + (test.passed ? "✓" : "") : "–"), h("td", { class: "num" }, fmtMin(sec / 60)));
  });
  const exRows = []; ORDER.forEach(id => (C.lessons[id].blocks || []).forEach(b => { if (b.type === "exercise") { const s = P.exercises[b.id] || {}; exRows.push(h("tr", null, h("td", null, h("a", { href: "#/lesson/" + id }, b.title)), h("td", null, h("span", { class: "tag " + (s.status === "passed" ? "pass" : s.status === "failed" ? "fail" : "") }, s.status === "passed" ? "passed" : s.status === "failed" ? "failing" : "to do")), h("td", { class: "num" }, String(s.attempts || 0)), h("td", { class: "num" }, String(s.hints || 0)), h("td", { class: "num" }, s.lastRun ? ago(s.lastRun) : "–"))); } }));
  const fileIn = h("input", { type: "file", accept: "application/json", style: "display:none", onchange: e => { const f = e.target.files[0]; if (!f) return; f.text().then(txt => { try { const o = JSON.parse(txt); if (!o || typeof o !== "object" || !o.lessons) throw new Error(); ask("Replace your current progress with the imported file?", () => { P = migrate(o); save(true); toast("Progress imported"); route(); }); } catch (err) { toast("That file is not a course progress file.", "alert"); } }); } });
  fill(el, h("div", { class: "page" }, h("h1", { class: "page-title" }, "Progress"), h("p", { class: "page-lead" }, "Everything you have done in the course, chapter by chapter. Study time counts only while the page is open and you are active."),
    h("div", { class: "tiles", style: "border-top:1px solid var(--rule)" }, [["Lessons", tot.les + "/" + ORDER.length], ["Exercises", tot.exP + "/" + tot.ex], ["Challenges", tot.pgP + "/" + tot.pg], ["Quiz answers", tot.qC + "/" + tot.q], ["Code runs", String(tot.runs)], ["Time studied", fmtMin(tot.minutes)]].map(([a, b2]) => h("div", { class: "tile" }, h("span", { class: "label" }, a), h("b", null, b2)))),
    h("section", { class: "section" }, h("div", { class: "section-head" }, h("h2", { class: "h2" }, "Chapters")), h("div", { class: "tablewrap", style: "border-top:0" }, h("table", { class: "data-table" }, h("thead", null, h("tr", null, h("th", null, "Chapter"), h("th", { class: "num" }, "Lessons"), h("th", { class: "num" }, "Quizzes"), h("th", { class: "num" }, "Exercises"), h("th", { class: "num" }, "Test best"), h("th", { class: "num" }, "Time"))), h("tbody", null, rows)))),
    exRows.length ? h("section", { class: "section" }, h("div", { class: "section-head" }, h("h2", { class: "h2" }, "Exercises")), h("div", { class: "tablewrap", style: "border-top:0" }, h("table", { class: "data-table" }, h("thead", null, h("tr", null, h("th", null, "Exercise"), h("th", null, "Status"), h("th", { class: "num" }, "Runs"), h("th", { class: "num" }, "Hints used"), h("th", { class: "num" }, "Last run"))), h("tbody", null, exRows)))) : null,
    h("section", { class: "section" }, h("div", { class: "section-head" }, h("h2", { class: "h2" }, "Keep your progress safe")),
      h("p", { style: "max-width:68ch" }, serverMode ? "Your progress is written to progress/progress.json next to the course (with a .bak copy of the previous save). You can also download it here." : "Right now progress lives in this browser only. Download it regularly, or start the course with start.sh / start.bat so it is saved to a file."),
      h("div", { class: "tools-row" }, h("button", { class: "btn", type: "button", onclick: () => { const a = h("a", { href: URL.createObjectURL(new Blob([JSON.stringify(P, null, 2)], { type: "application/json" })), download: "rust-course-progress-" + today() + ".json" }); document.body.append(a); a.click(); a.remove(); } }, ic("download", "sm"), "Download progress"),
        h("button", { class: "btn ghost", type: "button", onclick: () => fileIn.click() }, ic("upload", "sm"), "Import progress"), fileIn,
        h("button", { class: "btn ghost", type: "button", onclick: () => ask(t("resetConfirm"), () => { P = BLANK(); save(true); Con.clear(); route(); }) }, ic("reset", "sm"), t("reset")))),
    h("section", { class: "section" }, h("div", { class: "section-head" }, h("h2", { class: "h2" }, "Full activity log")), P.activity.length ? h("ul", { class: "activity" }, P.activity.slice(0, 60).map(a => { const [i2, txt] = activityText(a); return h("li", null, h("span", { class: "ic" }, ic(i2, "sm")), h("span", null, txt), h("time", { datetime: a.at }, new Date(a.at).toLocaleString())); })) : h("p", { class: "empty-note" }, "No activity yet."))),
    footer());
}
function renderGlossary() {
  const el = main(); const list = h("div", { class: "gloss" });
  const draw = q => { const items = (C.glossary || []).filter(g => !q || (g.term + " " + g.def).toLowerCase().indexOf(q.toLowerCase()) >= 0).sort((a, b) => a.term.localeCompare(b.term, "en"));
    list.replaceChildren(...(items.length ? items.map(g => h("div", { class: "gloss-item" }, h("b", null, g.term), h("p", { html: inline(g.def) }), g.lesson ? h("a", { href: "#/lesson/" + g.lesson }, "Introduced in: " + (C.lessons[g.lesson] ? C.lessons[g.lesson].title : "")) : null)) : [h("p", { class: "empty-note" }, "No term matches “" + q + "”.")])); };
  const inp = h("input", { class: "search", type: "search", placeholder: "Search " + (C.glossary || []).length + " terms", "aria-label": "Search the glossary", oninput: e => draw(e.target.value) });
  fill(el, h("div", { class: "page" }, h("h1", { class: "page-title" }, "Glossary"), h("p", { class: "page-lead" }, "Every term the course defines, with the lesson that introduces it."), inp, list), footer()); draw("");
}
function renderLesson(id) {
  const les = C.lessons[id]; const el = main();
  const ci = CH_OF[id], ch = C.chapters[ci], pos = ORDER.indexOf(id), isTest = les.kind === "test";
  P.last = id; const lp = (P.lessons[id] = P.lessons[id] || {}); if (!lp.visited) { lp.visited = new Date().toISOString(); log("visit", id); } save();
  const li = ch.lessons.indexOf(id);
  const col = h("article", { class: "lesson-col" },
    h("div", { class: "crumb" }, h("a", { href: "#/home" }, "Course"), ic("chev", "sm"), h("span", null, t("chapter") + " " + (ci + 1) + " · " + ch.title)),
    h("h1", { class: "lesson-title" }, les.title), les.summary ? h("p", { class: "lead" }, les.summary) : null,
    h("div", { class: "meta-row" }, h("span", { class: "tag" }, ic("clock", "sm"), "~" + (les.minutes || 10) + " " + t("min")), h("span", { class: "tag blue" }, isTest ? t("test") : t("lesson") + " " + (li + 1) + " of " + ch.lessons.length), lessonDone(id) ? h("span", { class: "tag pass" }, ic("check", "sm"), t("lessonDone")) : null));
  (les.blocks || []).forEach(b => col.append(renderBlock(b, les)));
  if (les.terms && les.terms.length) col.append(h("div", { class: "block terms" }, h("div", { class: "p-h" }, t("terms")), h("dl", null, les.terms.map(x => h("div", { class: "term-row" }, h("dt", null, x.term), h("dd", { html: inline(x.def) }))))));
  const fin = h("section", { class: "finish" });
  const drawFinish = () => {
    const p = practiceOf(les); const done = lessonDone(id);
    const chip = (label, a, b2) => b2 ? h("span", { class: "tag " + (a === b2 ? "pass" : "") }, label + " " + a + "/" + b2) : null;
    fill(fin, h("h3", null, done ? t("lessonDone") : isTest ? "Pass the test to complete the chapter" : t("yourPractice")),
      isTest ? null : h("div", { class: "row" }, chip("Quiz", p.qC, p.qT), chip("Exercises", p.eP, p.eT), chip("Challenges", p.pP, p.pT)),
      !done && !isTest && (p.qC < p.qT || p.eP < p.eT || p.pP < p.pT) ? h("p", { class: "caption", style: "margin:0" }, "The lesson completes itself once all practice above is done. You can also mark it complete now and come back later.") : null,
      h("div", { class: "row" }, !done && !isTest ? h("button", { class: "btn", type: "button", onclick: () => completeLesson(id) }, ic("check", "sm"), t("finishLesson")) : null,
        done && pos < ORDER.length - 1 ? h("a", { class: "btn", href: "#/lesson/" + ORDER[pos + 1] }, t("nextLesson"), ic("chev", "sm")) : null));
    const tagRow = $(".meta-row", col); if (done && tagRow && !tagRow.querySelector(".tag.pass")) tagRow.append(h("span", { class: "tag pass" }, ic("check", "sm"), t("lessonDone")));
  };
  window.__drawFinish = drawFinish; drawFinish(); col.append(fin);
  const prev = pos > 0 ? ORDER[pos - 1] : null, nxt = pos < ORDER.length - 1 ? ORDER[pos + 1] : null;
  col.append(h("nav", { class: "pager", "aria-label": "Lesson navigation" }, prev ? h("a", { href: "#/lesson/" + prev }, h("small", null, t("prev")), C.lessons[prev].title) : h("span"), nxt ? h("a", { class: "nx", href: "#/lesson/" + nxt }, h("small", null, t("next")), C.lessons[nxt].title) : h("span", { class: "nx" })));
  fill(el, h("div", { class: "lesson-grid" }, col, isTest ? null : Con.dock), isTest ? null : Con.peek);
  if (isTest) { el.firstChild.style.gridTemplateColumns = "minmax(0,1fr)"; }
  else {
    Con.setRunner();
    const loadedHere = Con.item && Con.item.card && col.contains(Con.item.card);
    if (!loadedHere) { const first = col.querySelector(".runnable"); if (first && first.__load) first.__load(); else Con.clear(); }
  }
  armReveal(col);
}

/* ───────────── chrome + routing ───────────── */
const openChapters = new Set();
function renderRail(current) {
  const rail = $("#rail"); const curCh = current && CH_OF[current] != null ? C.chapters[CH_OF[current]].id : null;
  if (curCh) openChapters.add(curCh);
  fill(rail, C.chapters.map((ch, ci) => {
    const open = openChapters.has(ch.id);
    const wrap = h("div", { class: "rail-ch" + (open ? " open" : "") + (ch.id === curCh ? " cur" : "") },
      h("button", { type: "button", "aria-expanded": String(open), onclick: () => { open ? openChapters.delete(ch.id) : openChapters.add(ch.id); renderRail(currentRoute); } },
        h("span", { class: "rail-num" }, String(ci + 1).padStart(2, "0")), h("span", { class: "rail-title" }, ch.title), h("span", { class: "rail-pct" }, chapterPct(ch) + "%")),
      h("ul", { class: "rail-lessons" }, ch.lessons.map(id => h("li", null, h("a", { href: "#/lesson/" + id, "aria-current": current === id ? "page" : null }, h("span", { class: "st" + (lessonDone(id) ? " done" : lessonStarted(id) ? " started" : "") + (C.lessons[id].kind === "test" ? " gate" : "") }), h("span", null, C.lessons[id].title))))));
    return wrap;
  }), ROADMAP.length ? h("p", { class: "caption", style: "padding:14px 16px" }, ROADMAP.length + " more chapters are being written: see the course page.") : null);
}
function refreshChrome() {
  const pct = overallPct(); const m = $("#meter > i"); if (m) m.style.width = pct + "%"; const mn = $("#meterNum"); if (mn) mn.textContent = pct + "%";
  renderRail(currentRoute);
}
function route() {
  const hash = location.hash.replace(/^#\/?/, "") || "home"; const [kind, ...rest] = hash.split("/"); const arg = rest.join("/");
  document.body.classList.remove("nav-open", "console-open");
  const shell = $("#shell");
  if (kind === "lesson" && C.lessons[arg]) { currentRoute = arg; shell.classList.remove("no-rail"); renderLesson(arg); document.title = C.lessons[arg].title + " · " + C.meta.title; }
  else if (kind === "glossary") { currentRoute = "glossary"; shell.classList.add("no-rail"); renderGlossary(); document.title = "Glossary · " + C.meta.title; }
  else if (kind === "progress") { currentRoute = "progress"; shell.classList.add("no-rail"); renderProgress(); document.title = "Progress · " + C.meta.title; }
  else { currentRoute = "home"; shell.classList.add("no-rail"); renderHome(); document.title = C.meta.title; }
  $$(".bar-nav a").forEach(a => a.setAttribute("aria-current", a.dataset.r === (C.lessons[currentRoute] ? "lesson" : currentRoute) ? "page" : "false"));
  refreshChrome(); window.scrollTo(0, 0); main().focus({ preventScroll: true });
}
function initChrome() {
  document.documentElement.lang = C.meta.language || "en";
  $("#brandName").textContent = C.meta.title.split(":")[0];
  let theme = null; try { theme = localStorage.getItem("course-theme"); } catch (e) { /* ignore */ }
  if (!theme) theme = window.matchMedia && matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  const setTheme = th => { document.documentElement.dataset.theme = th; fill($("#themeBtn"), ic(th === "dark" ? "sun" : "moon")); $("#themeBtn").setAttribute("aria-label", th === "dark" ? "Switch to light theme" : "Switch to dark theme"); };
  setTheme(theme);
  $("#themeBtn").onclick = () => { const n = document.documentElement.dataset.theme === "dark" ? "light" : "dark"; setTheme(n); try { localStorage.setItem("course-theme", n); } catch (e) { /* ignore */ } };
  fill($("#menuBtn"), ic("menu")); $("#menuBtn").onclick = () => document.body.classList.toggle("nav-open"); $("#scrim").onclick = () => document.body.classList.remove("nav-open");
}
(async function boot() {
  initChrome(); await loadProgress(); setSaveState();
  window.addEventListener("hashchange", route); route();
  window.__course = { get progress() { return P; }, route, save, tex2mml, Con };
})();
})();
