use modular_bitfield::prelude::*;

#[bitfield(filled = false)] // Requires just 4 bits!
#[derive(BitfieldSpecifier)]
#[derive(Debug)]
pub struct Header {
    status: B2,
    is_alive: bool,
    is_received: bool,
}

fn main() {
    let header = Header::new()
        .with_status(1)
        .with_is_alive(true)
        .with_is_received(false);
    assert_eq!(
        format!("{:?}", header),
        "Header { status: 1, is_alive: true, is_received: false }",
    );
    assert_eq!(
        format!("{:#X?}", header),
        "Header {\n    status: 0x1,\n    is_alive: true,\n    is_received: false,\n}",
    );
}
