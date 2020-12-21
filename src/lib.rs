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
mod aoc09;
mod aoc10;
mod aoc11;
mod aoc12;
mod aoc13;
mod aoc14;
mod aoc15;
mod aoc16;
mod aoc17;
mod aoc18;
mod aoc19;
mod aoc20;
mod aoc21;

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
    ("09", &[aoc09::run]),
    ("10", &[aoc10::run]),
    ("11", &[aoc11::run]),
    ("12", &[aoc12::run]),
    ("13", &[aoc13::run]),
    ("14", &[aoc14::run]),
    ("15", &[aoc15::run]),
    ("16", &[aoc16::run]),
    ("17", &[aoc17::run]),
    ("18", &[aoc18::run]),
    ("19", &[aoc19::run]),
    ("20", &[aoc20::run]),
    ("21", &[aoc21::run]),
];

pub fn map_file(path: String) -> IoResult<Mmap> {
    let file = File::open(path)?;
    unsafe { Mmap::map(&file) }
}

pub fn execute(id: usize, data: &[u8]) -> Option<String> {
    if id > PROGS.len() {
        return None;
    }
    let f = PROGS[id - 1].1[0];
    Some(f(data))
}
