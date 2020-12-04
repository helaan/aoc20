use rustc_hash::FxHashSet;

pub(crate) fn run(data: &[u8]) -> String {
    let mut singles = FxHashSet::default();
    let mut acc: i16 = 0;
    let mut p1 = 0;
    let mut p2 = 0;
    data.iter().for_each(|i| {
        let x: i16 = *i as i16;
        if x >= '0' as i16 && x <= '9' as i16 {
            acc *= 10;
            acc += x - '0' as i16;
        } else {
            if singles.contains(&(2020 - acc)) {
                //println!("Part 1: {}", acc as i32 * (2020 - acc as i32));
                p1 = acc as i32 * (2020 - acc as i32);
                //exit(0);
            }
            if acc != 0 {
                singles.insert(acc);
            }
            acc = 0;
        }
    });
    let mut xi = singles.iter();
    'p2: loop {
        match xi.next() {
            None => break,
            Some(x) => {
                let mut yi = xi.clone();
                loop {
                    match yi.next() {
                        None => break,
                        Some(y) => {
                            let z = 2020 - x - y;
                            if singles.contains(&z) {
                                //println!("Part 2: {}", *x as i32 * *y as i32 * z as i32);
                                p2 = *x as i32 * *y as i32 * z as i32;
                                break 'p2;
                            }
                        }
                    }
                }
            }
        }
    }
    format!("{} {}\n", p1, p2)
}
