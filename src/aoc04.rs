fn chk_pid(x: &[u8]) -> bool {
    if x.len() != 9 {
        //  println!("pid len");
        return false;
    }
    for p in 0..=8 {
        if !(x[p] >= '0' as u8 && x[p] <= '9' as u8) {
            //    println!("pid invalid");
            return false;
        }
    }
    true
}

fn chk_ecl(x: &[u8]) -> bool {
    if x.len() != 3 {
        return false;
    }
    match x[0] as char {
        'a' => x[1] == 'm' as u8 && x[2] == 'b' as u8,
        'b' => (x[1] == 'l' as u8 && x[2] == 'u' as u8) || (x[1] == 'r' as u8 && x[2] == 'n' as u8),
        'g' => x[1] == 'r' as u8 && (x[2] == 'y' as u8 || x[2] == 'n' as u8),
        'h' => x[1] == 'z' as u8 && x[2] == 'l' as u8,
        'o' => x[1] == 't' as u8 && x[2] == 'h' as u8,

        _ => false,
    }
}

fn chk_hcl(x: &[u8]) -> bool {
    if x.len() != 7 {
        //   println!("hcl len");
        return false;
    }
    if x[0] != '#' as u8 {
        //  println!("hcl# invalid");
        return false;
    }
    for p in 1..=6 {
        if !((x[p] >= '0' as u8 && x[p] <= '9' as u8) || (x[p] >= 'a' as u8 && x[p] <= 'f' as u8)) {
            //    println!("hcl invalid");
            return false;
        }
    }
    true
}

fn chk_hgt(x: &[u8]) -> bool {
    if x.len() < 4 {
        //println!("hgt len");
        return false;
    }
    match x.len() {
        4 => {
            //in
            let v = (x[0] - '0' as u8) * 10 + x[1] - '0' as u8;
            //    println!("v= {}", v);
            v >= 59 && v <= 76 && x[2] == 'i' as u8 && x[3] == 'n' as u8
            //   println!("hgtin invalid");
        }
        5 => {
            x[0] == '1' as u8
                && ((x[1] >= '5' as u8
                    && x[1] <= '8' as u8
                    && x[2] >= '0' as u8
                    && x[2] <= '9' as u8)
                    || (x[1] == '9' as u8 && x[2] >= '0' as u8 && x[2] <= '3' as u8))
                && x[3] == 'c' as u8
                && x[4] == 'm' as u8
        }
        _ => false,
    }
}

fn chk_eyr(x: &[u8]) -> bool {
    if x.len() != 4 {
        //println!("eyr len");
        return false;
    }
    x[0] == '2' as u8
        && x[1] == '0' as u8
        && ((x[2] == '2' as u8 && x[3] >= '0' as u8 && x[3] <= '9' as u8)
            || (x[2] == '3' as u8 && x[3] == '0' as u8))
}

fn chk_iyr(x: &[u8]) -> bool {
    if x.len() != 4 {
        //println!("iyr len");
        return false;
    }
    x[0] == '2' as u8
        && x[1] == '0' as u8
        && ((x[2] == '1' as u8 && x[3] >= '0' as u8 && x[3] <= '9' as u8)
            || (x[2] == '2' as u8 && x[3] == '0' as u8))
}

fn chk_byr(x: &[u8]) -> bool {
    if x.len() != 4 {
        //println!("byr len");
        return false;
    }
    match x[0] as char {
        '1' => {
            x[1] == '9' as u8
                && x[2] >= '2' as u8
                && x[2] <= '9' as u8
                && x[3] >= '0' as u8
                && x[3] <= '9' as u8
        }
        '2' => (x[1] == '0' as u8 && x[2] == '0' as u8 && x[3] >= '0' as u8 && x[3] <= '2' as u8),
        _ => false,
    }
}

pub(crate) fn run(b: &[u8]) -> String {
    let mut p = 0;
    let len = b.len();
    let mut p1 = 0;
    let mut p2 = 0;

    // x - byr = 64 - iyr = 32 - eyr = 16 - hgt = 8 - hcl = 4 - ecl = 2 - pid = 1
    let mut ok1: u8 = 0;
    let mut ok2 = true;

    while p < len {
        let fieldname = b[p] as char;
        //dbg!(p, b[p] as char);
        if fieldname == '\n' {
            if ok1 == 0b0111_1111 {
                p1 += 1;
                if ok2 {
                    p2 += 1;
                }
            }
            ok1 = 0;
            ok2 = true;
            p += 1;
            continue;
        }
        let fieldname2p = p + 1;
        p += 4;
        let pstart = p;
        p += 1; // assume there is at least 1 character of value
        while b[p] != ' ' as u8 && b[p] != '\n' as u8 {
            p += 1;
        }

        match fieldname {
            'p' => {
                // pid
                ok1 |= 1;
                ok2 &= chk_pid(&b[pstart..p]);
            }
            'e' => {
                if b[fieldname2p] == 'y' as u8 {
                    //eyr
                    ok1 |= 16;
                    ok2 &= chk_eyr(&b[pstart..p]);
                } else {
                    //ecl
                    ok1 |= 2;
                    ok2 &= chk_ecl(&b[pstart..p]);
                }
            }
            'h' => {
                if b[fieldname2p] == 'c' as u8 {
                    // hcl
                    ok1 |= 4;
                    ok2 &= chk_hcl(&b[pstart..p]);
                } else {
                    // hgt
                    ok1 |= 8;
                    ok2 &= chk_hgt(&b[pstart..p]);
                }
            }
            'i' => {
                ok1 |= 32;
                ok2 &= chk_iyr(&b[pstart..p]);
            }
            'b' => {
                ok1 |= 64;
                ok2 &= chk_byr(&b[pstart..p]);
            }
            'c' => {} //cid
            _ => unreachable!(fieldname),
        }
        p += 1;
    }
    if ok1 == 0b0111_1111 {
        p1 += 1;
        if ok2 {
            p2 += 1;
        }
    }

    format!("{} {}\n", p1, p2)
}
