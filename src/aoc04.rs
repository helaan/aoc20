fn chk_pid(x: &[u8]) -> bool {
    if x.len() != 9 {
        //  println!("pid len");
        return false;
    }
    for p in x.iter() {
        if !(*p >= b'0' && *p <= b'9') {
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
    match x[0] {
        b'a' => x[1] == b'm' && x[2] == b'b',
        b'b' => (x[1] == b'l' && x[2] == b'u') || (x[1] == b'r' && x[2] == b'n'),
        b'g' => x[1] == b'r' && (x[2] == b'y' || x[2] == b'n'),
        b'h' => x[1] == b'z' && x[2] == b'l',
        b'o' => x[1] == b't' && x[2] == b'h',

        _ => false,
    }
}

fn chk_hcl(x: &[u8]) -> bool {
    if x.len() != 7 {
        //   println!("hcl len");
        return false;
    }
    if x[0] != b'#' {
        //  println!("hcl# invalid");
        return false;
    }
    for p in x.iter().skip(1) {
        if !((*p >= b'0' && *p <= b'9') || (*p >= b'a' && *p <= b'f')) {
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
            let v = (x[0] - b'0') * 10 + x[1] - b'0';
            //    println!("v= {}", v);
            v >= 59 && v <= 76 && x[2] == b'i' && x[3] == b'n'
            //   println!("hgtin invalid");
        }
        5 => {
            x[0] == b'1'
                && ((x[1] >= b'5' && x[1] <= b'8' && x[2] >= b'0' && x[2] <= b'9')
                    || (x[1] == b'9' && x[2] >= b'0' && x[2] <= b'3'))
                && x[3] == b'c'
                && x[4] == b'm'
        }
        _ => false,
    }
}

fn chk_eyr(x: &[u8]) -> bool {
    if x.len() != 4 {
        //println!("eyr len");
        return false;
    }
    x[0] == b'2'
        && x[1] == b'0'
        && ((x[2] == b'2' && x[3] >= b'0' && x[3] <= b'9') || (x[2] == b'3' && x[3] == b'0'))
}

fn chk_iyr(x: &[u8]) -> bool {
    if x.len() != 4 {
        //println!("iyr len");
        return false;
    }
    x[0] == b'2'
        && x[1] == b'0'
        && ((x[2] == b'1' && x[3] >= b'0' && x[3] <= b'9') || (x[2] == b'2' && x[3] == b'0'))
}

fn chk_byr(x: &[u8]) -> bool {
    if x.len() != 4 {
        //println!("byr len");
        return false;
    }
    match x[0] {
        b'1' => x[1] == b'9' && x[2] >= b'2' && x[2] <= b'9' && x[3] >= b'0' && x[3] <= b'9',
        b'2' => (x[1] == b'0' && x[2] == b'0' && x[3] >= b'0' && x[3] <= b'2'),
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
        let fieldname = b[p];
        //dbg!(p, b[p] as char);
        if fieldname == b'\n' {
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
        while b[p] != b' ' && b[p] != b'\n' {
            p += 1;
        }

        match fieldname {
            b'p' => {
                // pid
                ok1 |= 1;
                ok2 &= chk_pid(&b[pstart..p]);
            }
            b'e' => {
                if b[fieldname2p] == b'y' {
                    //eyr
                    ok1 |= 16;
                    ok2 &= chk_eyr(&b[pstart..p]);
                } else {
                    //ecl
                    ok1 |= 2;
                    ok2 &= chk_ecl(&b[pstart..p]);
                }
            }
            b'h' => {
                if b[fieldname2p] == b'c' {
                    // hcl
                    ok1 |= 4;
                    ok2 &= chk_hcl(&b[pstart..p]);
                } else {
                    // hgt
                    ok1 |= 8;
                    ok2 &= chk_hgt(&b[pstart..p]);
                }
            }
            b'i' => {
                ok1 |= 32;
                ok2 &= chk_iyr(&b[pstart..p]);
            }
            b'b' => {
                ok1 |= 64;
                ok2 &= chk_byr(&b[pstart..p]);
            }
            b'c' => {} //cid
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
