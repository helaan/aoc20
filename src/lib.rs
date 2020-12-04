use memmap::Mmap;
use std::fs::File;
use std::io::Result as IoResult;

mod aoc01;
mod aoc02;
mod aoc03;
mod aoc04;

pub type FnAoc = fn(&[u8]) -> String;

pub const PROGS: &[(&str, &[FnAoc])] = &[
    ("01", &[aoc01::run]),
    ("02", &[aoc02::run]),
    ("03", &[aoc03::run]),
    ("04", &[aoc04::run]),
];

pub fn map_file(path: String) -> IoResult<Mmap> {
    let file = File::open(path)?;
    unsafe { Mmap::map(&file) }
}
