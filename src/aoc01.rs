use bit_set::BitSet;

pub(crate) fn run(data: &[u8]) -> String {
    let mut singles = BitSet::with_capacity(2020);
    let mut acc: usize = 0;
    //let mut p1 = 0;
    let mut p2 = 0;
    data.iter().for_each(|i| {
        let x = *i as usize;
        if x >= '0' as usize && x <= '9' as usize {
            acc *= 10;
            acc += x - '0' as usize;
        } else {
            // if singles.contains(2020 - acc) {
            //println!("Part 1: {}", acc as i32 * (2020 - acc as i32));
            //     p1 = acc as i32 * (2020 - acc as i32);
            //exit(0);
            //  }
            if acc != 0 {
                singles.insert(acc);
            }
            acc = 0;
        }
    });
    let p1 = singles
        .iter()
        .find_map(|x| {
            if singles.contains(2020 - x) {
                Some(x * (2020 - x))
            } else {
                None
            }
        })
        .unwrap();
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
                            if singles.contains(z) {
                                //println!("Part 2: {}", *x as i32 * *y as i32 * z as i32);
                                p2 = x * y * z;
                                break 'p2;
                            } else if x + y > 2020 {
                                //bitvec is sorted
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    format!("{} {}\n", p1, p2)
}
