use std::sync::atomic::{AtomicUsize, Ordering};
use cannoli::{Cannoli, create_cannoli};
use std::io::{BufWriter, Write};
use std::io;

struct Symbolizer {
    count: usize,
}

const BASE: u64 = 0x4000000000;
const PROG_END: u64 = BASE + 0x4ac0;
const DEC_START: u64 = BASE + 0x2415;
const DEC_RET: u64 = BASE + 0x2427;

#[derive(Clone, Copy)]
enum Trace {
    Store(u32, u64),
    Instr(u64),
}

impl Cannoli for Symbolizer {
    type Trace = Trace;

    type Context = AtomicUsize;

    fn init(_: u64, _: u64) -> (Self, Self::Context) {
        (Self { count: 0 }, AtomicUsize::new(0))
    }

    fn write(_ctxt: &Self::Context, _pc: u64, addr: u64, val: u64)
            -> Option<Self::Trace> {
        if addr > BASE && addr < PROG_END {
            Some(Trace::Store(addr as u32, val))
        } else {
            None
        }
    }

    fn exec(ctx: &Self::Context, pc: u64) -> Option<Self::Trace> {
        // ctx.fetch_add(1, Ordering::SeqCst);
        if pc > BASE && pc < PROG_END && (pc == DEC_RET || pc == DEC_START) {
            Some(Trace::Instr(pc))
        } else {
            None
        }
    }

    fn trace(&mut self, ctx: &Self::Context, trace: &[Self::Trace]) {
        self.count = ctx.load(Ordering::SeqCst);
        // let stdout = io::stdout().lock();
        // let mut bw = BufWriter::new(stdout);
        let mut decrypting = false;
        for e in trace {
            match e {
                Trace::Store(addr, val) => {
                    if decrypting {
                        // writeln!(&mut bw,"s{:x}={:x}", addr, val).unwrap()
                    }
                }
                Trace::Instr(pc) => {
                    if *pc == DEC_START {
                        decrypting = true;
                    } else if *pc == DEC_RET {
                        decrypting = false;
                    }
                    // writeln!(&mut bw,"x{:x}", pc-BASE).unwrap();
                    println!("x{:x}", pc-BASE);
                }
            }
        }
        // bw.flush().unwrap();
    }
}

impl Drop for Symbolizer {
    fn drop(&mut self) {
        println!("inscount: {}", self.count);
    }
}

fn main() {
    create_cannoli::<Symbolizer>(2).unwrap();
}
