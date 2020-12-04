pub(crate) fn run(b: &[u8]) -> String {
    let mut good = 0;
    let mut good2 = 0;
    let mut cnt = 0;
    let mut cnt2 = 0;
    let mut min = 0;
    let mut max = 0;
    let mut chr = '!' as u8;
    let mut st: i8 = -4;
    b.iter().for_each(|v| {
        let b = *v;
        //print!("{}", b as char);
        match st {
            -4 => {
                if b == '-' as u8 {
                    st = -3;
                } else if b == '\n' as u8 {
                    return;
                } else {
                    min *= 10;
                    min += b - '0' as u8;
                }
            }
            -3 => {
                if b == ' ' as u8 {
                    st = -2;
                } else {
                    max *= 10;
                    max += b - '0' as u8;
                }
            }
            -2 => {
                chr = b;
                st = -1;
            }
            -1 => st += 1,
            0 => st += 1,
            n => {
                if b == '\n' as u8 {
                    st = -4;
                    if cnt >= min && cnt <= max {
                        good += 1;
                    }
                    if cnt2 == 1 {
                        good2 += 1;
                        //println!(" valid");
                    }
                    min = 0;
                    max = 0;
                    cnt = 0;
                    cnt2 = 0;
                } else {
                    if b == chr {
                        //println!("{} {}", n, b);
                        cnt += 1;
                        let p = (n) as u8;
                        if p == min || p == max {
                            cnt2 += 1;
                        }
                    }
                    st += 1;
                }
            }
        }
    });
    format!("{} {}\n", good, good2)
}
