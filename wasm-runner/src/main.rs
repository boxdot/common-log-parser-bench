use structopt::StructOpt;
use wasmtime::*;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, StructOpt)]
struct Args {
    wasm: PathBuf,
    input: PathBuf,
}

// fn scope_timing<R>(label: &str, mut f: impl FnMut() -> R) -> R {
//     let start = Instant::now();
//     f();
//     println!("{}: {:?}", label, start.elapsed());
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::from_args();

    let store = Store::default();
    let module = Module::from_file(store.engine(), &args.wasm)?;
    let instance = Instance::new(&store, &module, &[])?;

    let memory = instance
        .get_memory("memory")
        .ok_or("missing `memory` export")?;

    let alloc = instance
        .get_func("alloc")
        .ok_or("missing export `alloc`")?
        .get1::<u32, u32>()?;

    let nom_parse = instance
        .get_func("nom_parse")
        .ok_or("missing `nom_parse` export")?
        .get2::<u32, u32, u32>()?;

    let (ptr, len) = {
        let _timing = ScopedTiming::new("Reading data into WASM linear memory");

        let mut f = File::open(args.input)?;
        let num_bytes = f.metadata()?.len() as usize;

        let guest_ptr_offset = alloc(num_bytes as u32)?;
        let host_ptr = unsafe { memory.data_ptr().offset(guest_ptr_offset as isize) };
        let guest_slice = unsafe { std::slice::from_raw_parts_mut(host_ptr, num_bytes) };
        f.read_exact(guest_slice)?;
        (guest_ptr_offset, num_bytes)
    };

    {
        let _timing = ScopedTiming::new("nom_parse");
        println!("Parsed: {}", nom_parse(ptr, len as u32)?);
    }

    Ok(())
}

struct ScopedTiming {
    label: &'static str,
    started: Instant,
}

impl ScopedTiming {
    fn new(label: &'static str) -> Self {
        Self {
            label,
            started: Instant::now(),
        }
    }
}

impl Drop for ScopedTiming {
    fn drop(&mut self) {
        println!("{}: {:?}", self.label, self.started.elapsed());
    }
}
