use super::*;

fn run(program: &[u8]) -> Cpu {
    let mut cpu = Cpu::new();
    cpu.load(0, program);
    cpu.run();
    cpu
}

#[test]
fn constants() {
    let cpu = run(&[0x60, 0x07, 0x70, 0x03, 0x00, 0x00]);
    assert_eq!(cpu.registers[0], 10);
    assert_eq!(cpu.steps, 3);
    let cpu = run(&[0x61, 0xFF, 0x71, 0x02, 0x80, 0x10, 0x00, 0x00]);
    assert_eq!(cpu.registers[0], 1, "7xkk wraps and 8xy0 copies");
    assert_eq!(cpu.registers[0xF], 0, "7xkk leaves VF alone");
}

#[test]
fn add_sub_flags() {
    let cpu = run(&[0x60, 200, 0x61, 100, 0x80, 0x14, 0x00, 0x00]);
    assert_eq!((cpu.registers[0], cpu.registers[0xF]), (44, 1));
    let cpu = run(&[0x60, 10, 0x61, 3, 0x80, 0x15, 0x00, 0x00]);
    assert_eq!((cpu.registers[0], cpu.registers[0xF]), (7, 1), "no borrow");
    let cpu = run(&[0x60, 3, 0x61, 10, 0x80, 0x15, 0x00, 0x00]);
    assert_eq!((cpu.registers[0], cpu.registers[0xF]), (249, 0), "borrow");
}

#[test]
fn jumps_and_skips() {
    // jump over an instruction that would set V0 to 99
    let cpu = run(&[0x12, 0x04, 0x60, 99, 0x00, 0x00]);
    assert_eq!(cpu.registers[0], 0);
    // 3xkk skips when equal, 4xkk does not
    let cpu = run(&[0x60, 5, 0x30, 5, 0x61, 1, 0x40, 5, 0x62, 1, 0x00, 0x00]);
    assert_eq!((cpu.registers[1], cpu.registers[2]), (0, 1));
}

#[test]
fn calls() {
    let mut cpu = Cpu::new();
    cpu.load(0, &[0x60, 5, 0x61, 10, 0x21, 0x00, 0x21, 0x00, 0x00, 0x00]);
    cpu.load(0x100, &[0x80, 0x14, 0x80, 0x14, 0x00, 0xEE]);
    cpu.run();
    assert_eq!(cpu.registers[0], 45);
}

#[test]
#[should_panic(expected = "unknown opcode f00d")]
fn unknown() {
    run(&[0xF0, 0x0D]);
}

#[test]
#[should_panic]
fn stack_overflow() {
    // a function that calls itself forever
    run(&[0x20, 0x00]);
}

#[test]
fn multiply() {
    for (a, b) in [(6, 7), (0, 9), (9, 0), (1, 1), (15, 17), (20, 20)] {
        let cpu = run(&multiply_program(a, b));
        assert_eq!(cpu.registers[0], a.wrapping_mul(b), "{a} * {b}");
    }
}
