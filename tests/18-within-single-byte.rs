use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier, Debug, PartialEq, Copy, Clone)]
pub enum Mode {
    A = 0b00,
    B = 0b01,
    C = 0b10,
    D = 0b11,
}

#[bitfield]
#[derive(Debug)]
pub struct StatFlag {
    x: bool,
    y: bool,
    z: B4,
    #[bits = 2]
    mode: Mode,
}

fn main() {
    let mut flag = StatFlag::new();

    assert_eq!(flag.get_x(), false);
    assert_eq!(flag.get_y(), false);
    assert_eq!(flag.get_z(), 0);
    assert_eq!(flag.get_mode(), Mode::A);

    let new_mode = Mode::B;

    flag.set_mode(new_mode);
    assert_eq!(flag.get_mode(), new_mode);

    flag.set_x(true);
    assert_eq!(flag.get_x(), true);
    assert_eq!(flag.get_mode(), new_mode);

    flag.set_y(true);
    assert_eq!(flag.get_y(), true);
    assert_eq!(flag.get_mode(), new_mode);

    flag.set_z(0b11);
    assert_eq!(flag.get_z(), 0b11);
    assert_eq!(flag.get_mode(), new_mode);
}
