use modular_bitfield::prelude::*;

#[derive(BitfieldSpecifier, Debug)]
#[bits = 2]
pub enum Status {
    Green = 0, Yellow = 1, Red = 2
    // 0x11 (= 3) is undefined here for Status!
}

#[bitfield]
#[derive(Debug)]
pub struct DataPackage {
    status: Status,
    contents: B4,
    is_alive: bool,
    is_received: bool,
}

fn main() {
    let package = DataPackage::from_bytes([0b01011011]);
    assert_eq!(
        format!("{:?}", package),
        "DataPackage { status: InvalidBitPattern { invalid_bytes: 3 }, contents: 6, is_alive: true, is_received: false }",
    );
    assert_eq!(
        format!("{:#X?}", package),
        "DataPackage {\n    \
            status: InvalidBitPattern {\n        \
                invalid_bytes: 0x3,\n    \
            },\n    \
            contents: 0x6,\n    \
            is_alive: true,\n    \
            is_received: false,\n\
        }",
    );
}
