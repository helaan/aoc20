use rustc_hash::FxHashMap;

const BORDER: u32 = 1000000;

pub(crate) fn run(data: &[u8]) -> String {
    let mut last_said_low = vec![0 as u32; BORDER as usize];
    let mut last_said_hi = FxHashMap::default();

    let mut turn = 1;
    let mut last: u32 = 0;

    for c in data {
        if *c == ',' as u8 {
            last_said_low[last as usize] = turn;
            turn += 1;
            last = 0;
        } else if *c == '\n' as u8 {
            break;
        } else {
            last *= 10;
            last += *c as u32 - '0' as u32;
        }
    }

    while turn < 2020 {
        let old = last_said_low[last as usize];
        last_said_low[last as usize] = turn;
        if old != 0 {
            last = turn - old;
        } else {
            last = 0;
        }
        turn += 1;
        //println!("{},{}", turn, last);
    }
    let p1 = last;

    while turn < 30000000 {
        let old = if last < BORDER {
            let old = last_said_low[last as usize];
            last_said_low[last as usize] = turn;
            old
        } else {
            //
            last_said_hi.insert(last, turn).map_or(0, |c| c)
        };
        if old != 0 {
            last = turn - old;
        } else {
            last = 0;
        }
        turn += 1;
        //println!("{},{}", turn, last);
    }

    /*let mut zero = 0;
    let mut zerowip = 0;
    let mut highest_used = 0;
    for (x, v) in last_said.iter().enumerate() {
        if *v == 0 {
            zerowip += 1;
        } else {
            highest_used = x;
            println!("{}\t{}", x, zerowip);
            zero += zerowip;
            zerowip = 0;
        }
    }
    dbg!(zero);
    dbg!(zerowip);
    dbg!(highest_used);*/

    format!("{} {}\n", p1, last)
}
