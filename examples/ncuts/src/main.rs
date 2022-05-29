use cannoli::{Cannoli, create_cannoli};

#[macro_use]
use serde;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Clone)]
struct Request {
    observe_addrs: Vec<u64>,
    test: u64,
}

#[derive(Serialize, Deserialize)]
struct Response {
    test: u64,
    read_addrs: Vec<u64>,
}

struct Tracer {
    ppid: u64,
    request: Request,
    need_read: usize,
    need_write: usize,
    trace_mem: bool,
    read_addrs: Vec<u64>,
}

enum Traced {
    PC(u64),
    Read(u64, u64, u64),
    Write(u64, u64, u64),
}

const DEBUG: bool = false;

impl Cannoli for Tracer {
    type Trace = Traced;
    type Context = Request;

    fn init(_: u64, ppid: u64) -> (Self, Self::Context) {
        let file    = std::fs::File::open(format!("/tmp/cannoli/{ppid}-1")).unwrap();
        let reader  = std::io::BufReader::new(file);
        let request: Request = serde_json::from_reader(reader).unwrap();

        (Self {
            ppid,
            request: request.clone(),
            need_read: 1000,
            need_write: 0,
            trace_mem: false,
            read_addrs: Vec::new(),
        }, request)
    }

    fn exec(_ctx: &Self::Context, pc: u64) -> Option<Self::Trace> {
        // Some(Traced::PC(pc))
        None
    }

    fn read(ctxt: &Self::Context, pc: u64, addr: u64, val: u64) -> Option<Self::Trace> {
        if ctxt.observe_addrs.binary_search(&addr).is_ok() || DEBUG {
            Some(Traced::Read(pc, addr, val))
        } else {
            None
        }

    }

    fn write(_ctxt: &Self::Context, pc: u64, addr: u64, val: u64) -> Option<Self::Trace> {
        // Some(Traced::Write(pc, addr, val))
        None
    }

    fn trace(&mut self, _ctxt: &Self::Context, trace: &[Self::Trace]) {
        for t in trace {
            match t {
                Traced::Read(pc, addr, val) => {
                    let mut trace = false;
                    if self.need_read > 0 {
                        self.need_read -= 1;
                        trace = true;
                    }
                    if trace || self.trace_mem {
                        if DEBUG {
                            if true {// *addr > 0x106b000 && *addr < 0x106f000 {
                                println!("r {pc:08x} {addr:08x} {val:08x}");
                            }
                        } else {
                            self.read_addrs.push(*addr);
                       }
                    }
                }
                Traced::Write(pc, addr, val) => {
                    let mut trace = false;
                    if self.need_write > 0 {
                        self.need_write -= 1;
                        trace = true;
                    }
                    if trace || self.trace_mem {
                        println!("w {pc:08x} {addr:08x} {val:08x}");
                    }
                }
                Traced::PC(pc) => {
                    if *pc == 0x0040074C {
                        self.need_read = 2;
                        println!("  {pc:08x}");
                    }
                    if *pc == 0x004006F0 {
                        self.trace_mem = true;
                        println!("  {pc:08x} [1");
                    }
                    if *pc == 0x00400718 {
                        println!("  {pc:08x}   [2");
                    }
                    if *pc == 0x00400744 {
                        self.trace_mem = false;
                    }
                }
            }
        }
        // self.count += trace.len();
    }
}

impl Drop for Tracer {
    fn drop(&mut self) {
        let response = Response{
            test: self.request.test,
            read_addrs: self.read_addrs.clone(),
        };
        serde_json::to_writer(
            std::fs::File::create(format!("/tmp/cannoli/{}-2", self.ppid)).unwrap(),
            &response,
        ).unwrap();
    }
}

fn main() {
    create_cannoli::<Tracer>(2).unwrap();
}
