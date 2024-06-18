enum Instruction {
    Simple(DataOp),
    Modded(ModOp, DataOp),
}

struct ModOp {
    code: ModCode,
    modifier: usize,
}

enum ModCode {
    LocaleLat,
    LocaleLatLon,
    JumperLat,
    JumperLon,
    ExternIn,
    ExternOut,
}

struct DataOp {
    code: DataCode,
    input: (PhasedBool, PhasedBool),
    load: Load,
}

enum DataCode {
    Nt,
    And,
    Nop,
    Ot,
    Add,
    In,
    Xor,
    Or,
    Nor,
    Nxor,
    Out,
    Sub,
    Not,
    Pg,
    Nand,
    Tt,
}

enum Load {
    Link(isize, isize),
    Off(isize),
}

struct PhasedBool {
    bool: bool,
    phase: Phase,
}

enum Phase {
    Even,
    Odd,
}

struct Data {
    a: Option<bool>,
    b: Option<bool>,
}

fn main() {}
