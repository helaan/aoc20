use std::io::Result as IoResult;
use std::time::Instant;

use aoc_henk::{map_file, FnAoc, PROGS};

fn main() -> IoResult<()> {
    let args = std::env::args();
    if args.len() == 1 {
        println!("Running all programs");
        for (name, prog) in PROGS {
            run_prog(&prog[0], name)?;
        }
    } else {
        for arg in args {
            for (name, prog) in PROGS {
                if name == &&arg {
                    run_prog(&prog[0], name)?;
                }
            }
        }
    }
    Ok(())
}

fn run_prog(prog: &FnAoc, name: &str) -> IoResult<()> {
    let input = map_file(format!("input/{}.in", name))?;
    let expected_output = map_file(format!("input/{}.out", name))?;

    let start_ts = Instant::now();
    let output = prog(&input);
    let end_ts = Instant::now();

    //println!("{}", output);
    assert_eq!(*output.as_bytes(), *expected_output);

    let duration = end_ts.duration_since(start_ts);
    println!("{} took {:?}", name, duration);

    Ok(())
}
