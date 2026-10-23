---
name: Rust, from first program to unsafe
description: A ledger you read, next to a console that checks your work.
colors:
  ledger-navy: "#09234f"
  registry-blue: "#384ce3"
  network-teal: "#1888a2"
  paper: "#ffffff"
  ink: "#0b0b0c"
  carbon: "#121012"
  mint-paper: "#eff5f0"
  sky-paper: "#c4f0ff"
  rule-gray: "#e5e5e5"
  steel-rule: "#ced3dc"
  quiet-gray: "#595959"
  link-blue: "#2a3cc8"
  pass-green: "#17784a"
  fail-red: "#b4302a"
  warn-amber: "#8f5a00"
  console-pass: "#9ef0c0"
  console-fail: "#ffb4ab"
  editor-error: "#ff8a80"
  editor-warning: "#f4bf5f"
  navy-hover: "#102f66"
  warn-rule: "#e5c58a"
  code-gutter: "#8f8a84"
  code-button: "#d7d5d0"
  code-highlight-gold: "#ffd479"
  console-output: "#071a3a"
  console-output-text: "#e9ecf6"
  console-output-dim: "#8ea0c4"
  console-idle: "#9aa0ad"
  console-offline: "#ffc66e"
  night-page: "#061733"
  night-surface: "#0a1f42"
  night-band: "#0c2c63"
  night-console-rule: "#5266ef"
  night-warn-rule: "#6b5320"
typography:
  display:
    fontFamily: "Cormorant Garamond, Iowan Old Style, Georgia, serif"
    fontSize: "clamp(2.25rem, 5vw, 3.5rem)"
    fontWeight: 500
    lineHeight: 1.05
    letterSpacing: "-0.005em"
  headline:
    fontFamily: "DM Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "22px"
    fontWeight: 500
    lineHeight: 1.3
    letterSpacing: "-0.015em"
  lead:
    fontFamily: "DM Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "20px"
    fontWeight: 400
    lineHeight: 1.4
    letterSpacing: "-0.01em"
  title:
    fontFamily: "DM Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "18px"
    fontWeight: 500
    lineHeight: 1.35
  body:
    fontFamily: "DM Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "16px"
    fontWeight: 400
    lineHeight: 1.55
  small:
    fontFamily: "DM Sans, ui-sans-serif, system-ui, sans-serif"
    fontSize: "15px"
    fontWeight: 400
    lineHeight: 1.5
  label:
    fontFamily: "IBM Plex Mono, ui-monospace, monospace"
    fontSize: "12px"
    fontWeight: 500
    lineHeight: 1.15
    letterSpacing: "0.04em"
  code:
    fontFamily: "IBM Plex Mono, ui-monospace, monospace"
    fontSize: "13.5px"
    fontWeight: 400
    lineHeight: 1.6
rounded:
  none: "0px"
  control: "4px"
spacing:
  gap: "12px"
  pad: "20px"
  section: "32px"
components:
  button-primary:
    backgroundColor: "{colors.ledger-navy}"
    textColor: "{colors.paper}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "10px 16px"
    height: "40px"
  button-ghost:
    backgroundColor: "{colors.paper}"
    textColor: "{colors.ledger-navy}"
    rounded: "{rounded.control}"
    padding: "10px 16px"
  console:
    backgroundColor: "{colors.registry-blue}"
    textColor: "{colors.paper}"
    rounded: "{rounded.control}"
  code-block:
    backgroundColor: "{colors.carbon}"
    typography: "{typography.code}"
    rounded: "{rounded.none}"
  tag:
    textColor: "{colors.quiet-gray}"
    typography: "{typography.label}"
    rounded: "{rounded.control}"
    padding: "4px 8px"
  card:
    backgroundColor: "{colors.paper}"
    rounded: "{rounded.none}"
    padding: "20px"
---

# Design System: Rust, from first program to unsafe

## Overview

**Creative North Star: "The Verification Console"**

The course is an institutional document opened next to a live console. The reading column is white paper with
hairline rules and calm serif and sans type. Beside it, the console is a lit Registry Blue panel where the learner's
code is compiled, run and checked. The two sides never swap roles: the page explains, the console verifies.

Density is moderate. Lessons are one 42rem column of prose, code and practice. Chrome is quiet: a navy bar, a
ledger-style rail of chapters, and mono labels that read like field annotations. Colour is spent on state (pass,
fail, in progress) and on the console, not on decoration. The world comes from Lukasz's DESIGN-1 guide (Baselayer:
"electric ledger in midnight"), with the brand fonts swapped for open stand-ins.

**Key Characteristics:**
- Flat surfaces, 1px rules, 0px cards, 4px controls, no shadows.
- Mono uppercase labels for actions, status and metadata.
- One lit surface per screen: the console, or the navy band on the course page.
- Plus-sign registration marks, and a faint grid behind the course band.

## Colors

A navy and electric-blue ledger on white paper, with mint and sky papers for quiet panels.

### Primary
- **Ledger Navy** (#09234f): the top bar, filled buttons, the course band and focus of dark fields.
- **Registry Blue** (#384ce3): the console, progress fills, the current-lesson marker and the focus ring.

### Secondary
- **Network Teal** (#1888a2): used sparingly for diagram nodes in the flow and stepper blocks.

### Neutral
- **Paper** (#ffffff): the page and cards.
- **Ink** (#0b0b0c): body text. **Quiet Gray** (#595959): secondary text, 7:1 on paper.
- **Carbon** (#121012): every code block and the editor.
- **Mint Paper** (#eff5f0) and **Sky Paper** (#c4f0ff): alternate panels, callouts and hover states.
- **Rule Gray** (#e5e5e5) and **Steel Rule** (#ced3dc): hairlines and control borders.

### State
- **Pass Green** (#17784a), **Fail Red** (#b4302a) and **Warn Amber** (#8f5a00) on paper, each with a pale
  background tint. On the blue console they become **Console Pass** (#9ef0c0) and **Console Fail** (#ffb4ab).
- In the editor, compiler errors are **Editor Error** (#ff8a80) and warnings **Editor Warning** (#f4bf5f).

### Named Rules
**The Lit Console Rule.** Registry Blue fills exactly one large surface: the console. Elsewhere it appears only as
a line, a fill in a meter or a small marker.

**The State Owns Colour Rule.** Green, red and amber always mean pass, fail and caution, never decoration.

## Typography

**Display Font:** Cormorant Garamond (with Iowan Old Style, Georgia)
**Body Font:** DM Sans (with system UI)
**Label/Mono Font:** IBM Plex Mono

**Character:** an editorial serif for the few statements that frame a chapter, a compact sans for everything you
read, and a mono voice for everything the system says.

### Hierarchy
- **Display** (500, clamp 2.25 to 3.5rem, 1.05): course and lesson titles, tile numbers, key-idea callouts.
- **Headline** (500, 22px, 1.3, -0.015em): lesson sections and block titles. The outline is always h1 then h2.
- **Lead** (400, 20px, 1.4): the one-paragraph introduction under a lesson or course title.
- **Title** (500, 18px, 1.35): glossary terms, card and dialog titles.
- **Body** (400, 16px, 1.55): prose, held to a 42rem measure. Paragraphs use `text-wrap: pretty`.
- **Small** (400, 15px, 1.5): quiz options, hints, resume-card text and other dense UI copy.
- **Label** (500, 12px, 0.04em, uppercase): buttons, tags, breadcrumbs, save state and console status.
- **Code** (400, 13.5px, 1.6): code blocks and the editor.

### Named Rules
**The System Speaks Mono Rule.** If the interface is reporting a status or offering an action, it is set in
uppercase Plex Mono. Content the learner reads is never set in mono except code.

## Layout

Three columns on wide screens: a 288px rail, the 42rem reading column and a console of `min(520px, 38vw)` that stays
in view while the lesson scrolls. Lessons with nothing to run drop the console and use one column.

- At 1180px and below the console becomes a bottom sheet with a peek bar. "Try it" and error links open it.
- At 900px and below the rail becomes a drawer with a Course / Progress / Glossary row.
- At 760px and below optional table columns hide and the how-it-works steps stack.
- Spacing steps are 12px (gap), 20px (pad) and 32px (section). Phone side gutters are 16px.

## Elevation & Depth

The system is flat. There are no shadows. Depth comes from tone: paper, mint and sky panels on white, the lit blue
console, Carbon code blocks, and the navy bar. Separation is always a 1px rule. A faint 1px Registry Blue grid sits behind
the course band as a registration field.

### Named Rules
**The Hairline Rule.** Separate with a 1px rule or a tonal change, never with a shadow.

## Shapes

Cards, panels, code blocks and the console frame have square corners. Buttons, tags, inputs and quiz options use
4px. Plus-sign registration marks sit on the corners of large fields. Progress cells in the ledger are small squares
whose fill shows state: empty, a half fill for started, solid for done.

## Components

### Buttons
- **Shape:** 4px corners, minimum height 40px.
- **Primary:** Ledger Navy fill, white uppercase mono label, 10px 16px padding.
- **Ghost:** transparent with a navy 1px border. On the console both invert: white fill with navy text, or a white
  outline.
- **Hover / Focus:** hover shifts to Registry Blue or Sky Paper over 150ms. Focus is a 2px Registry Blue outline
  with a 2px offset.

### Tags
- **Style:** 1px Steel Rule border, 4px corners, 11px uppercase mono. Blue, green and red variants mark the
  console, a pass and a failure.

### Cards / Containers
- **Corner Style:** 0px.
- **Background:** Paper, Mint Paper or Sky Paper.
- **Shadow Strategy:** none (see Elevation).
- **Border:** 1px Rule Gray or Steel Rule.
- **Internal Padding:** 20px.

### Inputs / Quiz options
- **Style:** 1px Steel Rule border, 4px corners, 10px 12px padding, with a custom radio or checkbox.
- **Focus:** the Registry Blue outline. **Selected:** a Registry Blue border on a pale blue surface.
- **Feedback:** a Pass or Fail tinted panel under the question with a short heading and the explanation.

### Navigation
- **Bar:** Ledger Navy, 64px, brand mark, Course / Progress / Glossary, save state, progress meter and theme toggle.
- **Rail:** chapters as numbered ledger rows with a percentage. The current lesson has a Registry Blue marker
  and a pale blue row. On phones it is a drawer.

### Verification Console (signature component)
The console is the course's identity. It is a Registry Blue panel with a mono header, the task, a Carbon editor
with a line-number gutter, Run / Reset / Copy actions and a result list.

- Results tick in one row at a time: Compile, Run or Tests, and Output check.
- A one-line summary in plain language comes before the raw compiler output.
- Compiler errors are drawn in the editor as a wavy underline under the exact span, with a bar in the gutter.
  "Line N" in the summary jumps the caret there.
- A passed check sets a stamp inside the console.

## Do's and Don'ts

### Do:
- **Do** keep cards and panels at 0px corners and controls at 4px.
- **Do** set every action and status label in uppercase IBM Plex Mono at 11 to 12px.
- **Do** put new interactive practice in the console rather than in a new inline widget.
- **Do** explain every compiler or test result in one plain line before the raw output.
- **Do** check contrast on the blue console: white and the console state tokens only.

### Don't:
- **Don't** add shadows, glows or gradients to cards. The Ledger gradient is reserved for the course band.
- **Don't** use coloured side stripes to mark state. Use an outline, a tag or a fill.
- **Don't** fill a second large surface with Registry Blue on the same screen.
- **Don't** animate content into view on scroll. Only verification moves.
