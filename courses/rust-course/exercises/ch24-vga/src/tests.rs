use super::*;
use core::fmt::Write;

fn row(buf: &[u16], width: usize, r: usize) -> String {
    buf[r * width..(r + 1) * width]
        .iter()
        .map(|&c| match (c & 0xff) as u8 {
            0xfe => '#',
            b => b as char,
        })
        .collect()
}

#[test]
fn attributes_and_cells() {
    assert_eq!(attribute(Color::BrightCyan, Color::Black), 0x0b);
    assert_eq!(attribute(Color::Yellow, Color::Blue), 0x1e);
    assert_eq!(cell(b'A', 0x1e), 0x1e41);
    assert_eq!(Color::White as u8, 15);
}

#[test]
fn new_clears() {
    let mut buf = [0xffffu16; 6];
    let w = Writer::new(&mut buf, 3, 2);
    assert_eq!(w.position(), (0, 0));
    assert!(buf.iter().all(|&c| c == 0x0720));
}

#[test]
#[should_panic]
fn wrong_size_panics() {
    let mut buf = [0u16; 5];
    Writer::new(&mut buf, 3, 2);
}

#[test]
fn writing_and_colors() {
    let mut buf = [0u16; 8];
    let mut w = Writer::new(&mut buf, 4, 2);
    w.write_byte(b'h');
    w.set_color(Color::Yellow, Color::Blue);
    w.write_byte(b'i');
    assert_eq!(w.position(), (0, 2));
    assert_eq!(buf[0], 0x0768);
    assert_eq!(buf[1], 0x1e69);
    assert_eq!(buf[2], 0x0720);
}

#[test]
fn newline_and_wrap() {
    let mut buf = [0u16; 15];
    let mut w = Writer::new(&mut buf, 5, 3);
    write!(w, "ab\ncdefgh").unwrap();
    assert_eq!(w.position(), (2, 1));
    assert_eq!(row(&buf, 5, 0), "ab   ");
    assert_eq!(row(&buf, 5, 1), "cdefg");
    assert_eq!(row(&buf, 5, 2), "h    ");
}

#[test]
fn exact_fill_waits_to_wrap() {
    let mut buf = [0u16; 6];
    let mut w = Writer::new(&mut buf, 3, 2);
    write!(w, "abc").unwrap();
    assert_eq!(w.position(), (0, 3));
    w.write_byte(b'\n');
    assert_eq!(w.position(), (1, 0));
}

#[test]
fn scrolling() {
    let mut buf = [0u16; 6];
    let mut w = Writer::new(&mut buf, 3, 2);
    write!(w, "one\ntwo\nsix").unwrap();
    assert_eq!(w.position(), (1, 3));
    assert_eq!(row(&buf, 3, 0), "two");
    assert_eq!(row(&buf, 3, 1), "six");
}

#[test]
fn scroll_blanks_in_current_color() {
    let mut buf = [0u16; 4];
    let mut w = Writer::new(&mut buf, 2, 2);
    w.set_color(Color::White, Color::Red);
    write!(w, "a\nb\n").unwrap();
    assert_eq!(w.position(), (1, 0));
    assert_eq!(row(&buf, 2, 0), "b ");
    assert_eq!(buf[0], 0x4f62);
    assert_eq!(buf[1], 0x0720);
    assert_eq!(buf[2], 0x4f20);
    assert_eq!(buf[3], 0x4f20);
}

#[test]
fn wrapping_on_last_row_scrolls() {
    let mut buf = [0u16; 4];
    let mut w = Writer::new(&mut buf, 2, 2);
    write!(w, "abcdef").unwrap();
    assert_eq!(w.position(), (1, 2));
    assert_eq!(row(&buf, 2, 0), "cd");
    assert_eq!(row(&buf, 2, 1), "ef");
}

#[test]
fn non_printable_bytes() {
    let mut buf = [0u16; 8];
    let mut w = Writer::new(&mut buf, 8, 1);
    write!(w, "é\t~").unwrap();
    assert_eq!(row(&buf, 8, 0), "###~    ");
}

#[test]
fn clear_resets() {
    let mut buf = [0u16; 4];
    let mut w = Writer::new(&mut buf, 2, 2);
    write!(w, "xyz").unwrap();
    w.set_color(Color::Black, Color::Green);
    w.clear();
    assert_eq!(w.position(), (0, 0));
    assert!(buf.iter().all(|&c| c == 0x2020));
}

#[test]
fn formatting() {
    let mut buf = [0u16; 20];
    let mut w = Writer::new(&mut buf, 20, 1);
    write!(w, "{}+{}={}", 2, 2, 2 + 2).unwrap();
    assert_eq!(row(&buf, 20, 0).trim_end(), "2+2=4");
}
