use memmap::Mmap;
use std::fs::File;
use std::io::Result as IoResult;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;
mod aoc05;
mod aoc06;
mod aoc07;
mod aoc08;

pub type FnAoc = fn(&[u8]) -> String;

pub const PROGS: &[(&str, &[FnAoc])] = &[
    ("01", &[aoc01::run]),
    ("02", &[aoc02::run]),
    ("03", &[aoc03::run]),
    ("04", &[aoc04::run]),
    ("05", &[aoc05::run]),
    ("06", &[aoc06::run]),
    ("07", &[aoc07::run]),
    ("08", &[aoc08::run]),
];

pub fn map_file(path: String) -> IoResult<Mmap> {
    let file = File::open(path)?;
    unsafe { Mmap::map(&file) }
}

pub fn execute(id: usize, data: &[u8]) -> Option<String> {
    if id >= PROGS.len() {
        return None;
    }
    let f = PROGS[id - 1].1[0];
    Some(f(data))
}
