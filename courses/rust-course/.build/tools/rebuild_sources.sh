#!/bin/sh
# Recreate .build/source/ (gitignored: it holds the extracted book text) from the original PDFs.
# Usage: .build/tools/rebuild_sources.sh /path/to/folder-with-the-pdfs
# Needs pdftotext (poppler-utils) and python3. Expects these file names in the folder:
#   "The Rust Programming Language.pdf" (2024 ed.), "4_Programming Rust 2nd Edition.pdf",
#   "3_rust_in_action.pdf", "comprehensive-rust.pdf"
set -e
PDFS="$1"; [ -d "$PDFS" ] || { echo "usage: $0 PDF_FOLDER"; exit 1; }
HERE="$(cd "$(dirname "$0")" && pwd)"; COURSE="$(dirname "$(dirname "$HERE")")"
SKILL="${BOOK_TO_COURSE:-$HOME/.claude/skills/book-to-course}"
W="$(mktemp -d)"
python3 "$SKILL/scripts/extract_book.py" "$PDFS/The Rust Programming Language.pdf" "$W/trpl" >/dev/null
python3 "$SKILL/scripts/extract_book.py" "$PDFS/4_Programming Rust 2nd Edition.pdf" "$W/books/programming-rust-2e" >/dev/null
python3 "$SKILL/scripts/extract_book.py" "$PDFS/3_rust_in_action.pdf" "$W/books/rust-in-action" >/dev/null
python3 "$SKILL/scripts/extract_book.py" "$PDFS/comprehensive-rust.pdf" "$W/books/comprehensive-rust" >/dev/null
cat "$W"/trpl/chapters/*.md > "$W/trpl_all.md"
# NOTE: split_trpl.py holds line numbers measured on this exact extraction; re-check them if the PDF changes
python3 "$HERE/split_trpl.py" "$W/trpl_all.md" "$W/books/trpl-2024" >/dev/null
python3 "$HERE/make_merged.py" "$W" >/dev/null
mkdir -p "$COURSE/.build/source"
rm -rf "$COURSE/.build/source/books" "$COURSE/.build/source/chapters"
cp -r "$W/books" "$COURSE/.build/source/books"
cp -r "$W/merged/chapters" "$COURSE/.build/source/chapters"
cp "$W/merged/manifest.json" "$W/merged/outline.md" "$COURSE/.build/source/"
rm -rf "$W"
echo "Sources rebuilt in $COURSE/.build/source"
