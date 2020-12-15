#[inline]
fn seat_nr(b: &[u8]) -> usize {
    let mut r = 0;
    for c in b {
        r <<= 1;
        // B and R have bit 3 unset, while F and L have it set
        // tnx Michiel
        if c & 0b00000100 == 0 {
            r |= 1;
        }
    }
    r
}

pub(crate) fn run(data: &[u8]) -> String {
    let mut p = 0;
    let len = data.len();
    let mut max = 0;
    let mut min = 1024;
    //let mut seats = BitSet::with_capacity(1024);
    let mut seats = [0 as u32; 32];
    while p < len {
        let x = seat_nr(&data[p..p + 10]);
        if x > max {
            max = x
        }
        if x < min {
            min = x
        }
        //seats.insert(x);
        let v = 1 << (x % 32);
        let k = x / 32;
        seats[k] |= v;
        p += 11;
    }
    //let missing = (min..max).find(|x| !seats.contains(*x)).unwrap();
    let missing: usize = ((min / 32 + 1)..32)
        .find_map(|i| {
            if seats[i] != std::u32::MAX {
                let mut v = seats[i];
                let mut x = 0;
                let mut result;
                loop {
                    if v % 2 == 0 {
                        result = 32 * i + x;
                        break;
                    }
                    x += 1;
                    v >>= 1;
                }
                if result > max {
                    // look at the block with the minimum
                    let mut v = seats[(min / 32)] >> (min % 32);
                    let mut x = 0;
                    loop {
                        if v % 2 == 0 {
                            result = min + x;
                            break;
                        }
                        x += 1;
                        v >>= 1;
                    }
                }
                Some(result)
            } else {
                None
            }
        })
        .unwrap();
    format!("{} {}\n", max, missing)
}
