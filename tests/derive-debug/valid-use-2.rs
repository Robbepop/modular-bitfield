use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier, Debug)]
pub enum Status {
    Green, Yellow, Red, None
}

#[bitfield]
#[derive(Debug)]
pub struct DataPackage {
    status: Status,
    contents: B60,
    is_alive: bool,
    is_received: bool,
}

fn main() {
    let package = DataPackage::new()
        .with_status(Status::Green)
        .with_contents(0xC0DE_CAFE)
        .with_is_alive(true)
        .with_is_received(false);
    assert_eq!(
        format!("{:?}", package),
        "DataPackage { status: Green, contents: 3235826430, is_alive: true, is_received: false }",
    );
    assert_eq!(
        format!("{:#X?}", package),
        "DataPackage {\n    status: Green,\n    contents: 0xC0DECAFE,\n    is_alive: true,\n    is_received: false,\n}",
    );
}
