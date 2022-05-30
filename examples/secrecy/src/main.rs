use cannoli::{Cannoli, create_cannoli};

struct Symbolizer {
}

const BASE: u64 = 0x4000000000;
const DEC_START: u64 = BASE + 0x2415;
const DEC_RET: u64 = BASE + 0x2427;

#[derive(Clone, Copy)]
enum Trace {
    Instr(u64),
}

impl Cannoli for Symbolizer {
    type Trace = Trace;

    type Context = ();

    fn init(_: u64, _: u64) -> (Self, Self::Context) {
        (Self {}, ())
    }

    fn exec(_ctx: &Self::Context, pc: u64) -> Option<Self::Trace> {
        if pc == DEC_RET || pc == DEC_START {
            Some(Trace::Instr(pc))
        } else {
            None
        }
    }

    fn trace(&mut self, _ctx: &Self::Context, trace: &[Self::Trace]) {
        for e in trace {
            match e {
                Trace::Instr(pc) => {
                    println!("0x{:x}", pc-BASE);
                }
            }
        }
    }
}

fn main() {
    create_cannoli::<Symbolizer>(2).unwrap();
}
