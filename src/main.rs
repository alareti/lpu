struct Dag {
    instructions: Vec<Instr>,
    nodes: Vec<Node>,
    phase: Phase,
}

struct Node {
    ia: PhasedBool,
    ib: PhasedBool,
}

struct PhasedBool {
    bool: bool,
    phase: Phase,
}

enum Phase {
    Even,
    Odd,
}

enum Instr {
    Modded(ModOp, DataOp),
    Dubbed(DataOp, DataOp),
}

struct ModOp {
    code: ModOpCode,
    r#mod: u16,
}

enum ModOpCode {
    LocLon,
    LocLatLon,
    JumpLon,
    JumpLat,
    PortIn,
    PortOut,
}

struct DataOp {
    code: DataOpCode,
    la: u8,
    lb: u8,
}

struct Data {
    a: Option<bool>,
    b: Option<bool>,
}

impl DataOp {
    fn eval(&self, i: Data) -> Data {
        let ia = i.a;
        let ib = i.b;

        // Assert preconditions of all the codes
        match self.code {
            DataOpCode::Nop => {}
            DataOpCode::Ot | DataOpCode::Not => {
                assert!((ia.is_some() && ib.is_none()) || (ia.is_none() && ib.is_some()))
            }
            DataOpCode::Nt | DataOpCode::Tt => assert_eq!((ia, ib), (None, None)),
            DataOpCode::And
            | DataOpCode::Sub
            | DataOpCode::Pg
            | DataOpCode::Xor
            | DataOpCode::Or
            | DataOpCode::Nor
            | DataOpCode::Nxor
            | DataOpCode::Dp
            | DataOpCode::Add
            | DataOpCode::Nand
            | DataOpCode::Fs => assert_ne!((ia, ib), (None, None)),
        }

        // Evaluate output of code
        match self.code {
            DataOpCode::Nt => Data {
                a: Some(false),
                b: Some(false),
            },
            DataOpCode::And => {
                let o = ia.unwrap() & ib.unwrap();
                Data {
                    a: Some(o),
                    b: Some(o),
                }
            }
            DataOpCode::Nop => Data { a: None, b: None },
            DataOpCode::Ot => {
                let o;
                if let Some(ia) = ia {
                    o = ia;
                } else {
                    o = ib.unwrap();
                }

                Data {
                    a: Some(o),
                    b: Some(o),
                }
            }
            DataOpCode::Sub => {
                let oa = ia.unwrap() ^ ib.unwrap();
                let ob = !(ia.unwrap() | !ib.unwrap());

                Data {
                    a: Some(oa),
                    b: Some(ob),
                }
            }
            DataOpCode::Pg => {
                let o = match ib.unwrap() {
                    false => None,
                    true => Some(ia.unwrap()),
                };

                Data { a: o, b: o }
            }
            DataOpCode::Xor => {
                let o = ia.unwrap() ^ ib.unwrap();
                Data {
                    a: Some(o),
                    b: Some(o),
                }
            }
            DataOpCode::Or => {
                let o = ia.unwrap() | ib.unwrap();
                Data {
                    a: Some(o),
                    b: Some(o),
                }
            }
            DataOpCode::Nor => {
                let o = !(ia.unwrap() | ib.unwrap());
                Data {
                    a: Some(o),
                    b: Some(o),
                }
            }
            DataOpCode::Nxor => {
                let o = !(ia.unwrap() ^ ib.unwrap());
                Data {
                    a: Some(o),
                    b: Some(o),
                }
            }
            DataOpCode::Dp => {
                let oa = ia.unwrap();
                let ob = ib.unwrap();
                Data {
                    a: Some(oa),
                    b: Some(ob),
                }
            }
            DataOpCode::Add => {
                let oa = ia.unwrap() ^ ib.unwrap();
                let ob = ia.unwrap() & ib.unwrap();
                Data {
                    a: Some(oa),
                    b: Some(ob),
                }
            }
            DataOpCode::Not => {
                let o;
                if let Some(ia) = ia {
                    o = !ia;
                } else {
                    o = !ib.unwrap();
                }

                Data {
                    a: Some(o),
                    b: Some(o),
                }
            }
            DataOpCode::Fs => Data { a: None, b: None },
            DataOpCode::Nand => {
                let o = !(ia.unwrap() & ib.unwrap());
                Data {
                    a: Some(o),
                    b: Some(o),
                }
            }
            DataOpCode::Tt => Data {
                a: Some(true),
                b: Some(true),
            },
        }
    }
}

enum DataOpCode {
    Nt,
    And,
    Nop,
    Ot,
    Sub,
    Pg,
    Xor,
    Or,
    Nor,
    Nxor,
    Dp,
    Add,
    Not,
    Fs,
    Nand,
    Tt,
}

fn main() {}
