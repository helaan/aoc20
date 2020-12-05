use bit_set::BitSet;

#[inline]
fn seat_nr(b: &[u8]) -> usize {
    let mut r = 0;
    for i in 0..7 {
        r *= 2;
        //print!("{}", b[i] as char);
        if b[i] == 'B' as u8 {
            r += 1;
            //        } else if b[i] != 'F' as u8 {
            //          panic!("bfbfbf");
        }
    }
    for i in 7..10 {
        r *= 2;
        //print!("{}", b[i] as char);
        if b[i] == 'R' as u8 {
            r += 1;
            //        } else if b[i] != 'L' as u8 {
            //          panic!("lrlrl");
        }
    }
    //println!(" => {} {} {}", r, c, r * 8 + c);
    r
}

pub(crate) fn run(b: &[u8]) -> String {
    let mut p = 0;
    let len = b.len();
    let mut max = 0;
    let mut min = 1024;
    let mut seats = BitSet::with_capacity(1024);
    while p < len {
        let x = seat_nr(&b[p..p + 10]);
        if x > max {
            max = x
        }
        if x < min {
            min = x
        }
        seats.insert(x);
        p += 11;
    }
    let missing = (min..max).find(|x| !seats.contains(*x)).unwrap();
    format!("{} {}\n", max, missing)
}
