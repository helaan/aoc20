use rustc_hash::FxHashMap;

fn valid(pass: &FxHashMap<&[u8], &[u8]>) -> bool {
    pass.contains_key("byr".as_bytes())
        && pass.contains_key("iyr".as_bytes())
        && pass.contains_key("eyr".as_bytes())
        && pass.contains_key("hgt".as_bytes())
        && pass.contains_key("hcl".as_bytes())
        && pass.contains_key("ecl".as_bytes())
        && pass.contains_key("pid".as_bytes())
}

fn valid2(pass: &FxHashMap<&[u8], &[u8]>) -> bool {
    if let Some(x) = pass.get("byr".as_bytes()) {
        if x.len() != 4 {
            //println!("byr len");
            return false;
        }
        if x[0] == '1' as u8 {
            if !(x[1] == '9' as u8
                && x[2] >= '2' as u8
                && x[2] <= '9' as u8
                && x[3] >= '0' as u8
                && x[3] <= '9' as u8)
            {
                //println!("byr invalid");
                return false;
            }
        } else if x[0] == '2' as u8 {
            if !(x[1] == '0' as u8 && x[2] == '0' as u8 && x[3] >= '0' as u8 && x[3] <= '2' as u8) {
                //println!("byr2 invalid");
                return false;
            }
        } else {
            //println!("byr? invalid");
            return false;
        }
    } else {
        return false;
    }
    if let Some(x) = pass.get("iyr".as_bytes()) {
        if x.len() != 4 {
            //println!("iyr len");
            return false;
        }
        if !(x[0] == '2' as u8
            && x[1] == '0' as u8
            && ((x[2] == '1' as u8 && x[3] >= '0' as u8 && x[3] <= '9' as u8)
                || (x[2] == '2' as u8 && x[3] == '0' as u8)))
        {
            // println!("iyr invalid");
            return false;
        }
    } else {
        //println!("iyr? invalid");
        return false;
    }
    if let Some(x) = pass.get("eyr".as_bytes()) {
        if x.len() != 4 {
            //println!("eyr len");
            return false;
        }
        if !(x[0] == '2' as u8
            && x[1] == '0' as u8
            && ((x[2] == '2' as u8 && x[3] >= '0' as u8 && x[3] <= '9' as u8)
                || (x[2] == '3' as u8 && x[3] == '0' as u8)))
        {
            // println!("eyr invalid");
            return false;
        }
    } else {
        //println!("eyr? invalid");
        return false;
    }
    if let Some(x) = pass.get("hgt".as_bytes()) {
        //sus
        if x.len() < 4 {
            //println!("hgt len");
            return false;
        }
        if x[3] == 'c' as u8 && x.len() == 5 {
            // cm
            if !(x[0] == '1' as u8
                && ((x[1] >= '5' as u8
                    && x[1] <= '8' as u8
                    && x[2] >= '0' as u8
                    && x[2] <= '9' as u8)
                    || (x[1] == '9' as u8 && x[2] >= '0' as u8 && x[2] <= '3' as u8))
                && x[4] == 'm' as u8)
            {
                // println!("hgtcm invalid");
                return false;
            }
        } else if x[3] == 'n' as u8 && x.len() == 4 {
            //in
            let v = (x[0] - '0' as u8) * 10 + x[1] - '0' as u8;
            //    println!("v= {}", v);
            if !(v >= 59 && v <= 76 && x[2] == 'i' as u8) {
                //   println!("hgtin invalid");
                return false;
            }
        } else {
            //    println!("hgt lol");
            return false;
        }
    }

    if let Some(x) = pass.get("hcl".as_bytes()) {
        if x.len() != 7 {
            //   println!("hcl len");
            return false;
        }
        if x[0] != '#' as u8 {
            //  println!("hcl# invalid");
            return false;
        }
        for p in 1..=6 {
            if !((x[p] >= '0' as u8 && x[p] <= '9' as u8)
                || (x[p] >= 'a' as u8 && x[p] <= 'f' as u8))
            {
                //    println!("hcl invalid");
                return false;
            }
        }
    }
    if let Some(x) = pass.get("ecl".as_bytes()) {
        if x.len() != 3 {
            return false;
        }
        let v = match x[0] as char {
            'a' => x[1] == 'm' as u8 && x[2] == 'b' as u8,
            'b' => {
                (x[1] == 'l' as u8 && x[2] == 'u' as u8) || (x[1] == 'r' as u8 && x[2] == 'n' as u8)
            }
            'g' => x[1] == 'r' as u8 && (x[2] == 'y' as u8 || x[2] == 'n' as u8),
            'h' => x[1] == 'z' as u8 && x[2] == 'l' as u8,
            'o' => x[1] == 't' as u8 && x[2] == 'h' as u8,

            _ => false,
        };
        if !v {
            //  println!("ecl invalid");
            return false;
        }
    }
    if let Some(x) = pass.get("pid".as_bytes()) {
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
    }

    return true;
}

pub(crate) fn run(b: &[u8]) -> String {
    let mut pass = FxHashMap::default();
    let mut p = 0;
    let len = b.len();
    let mut good = 0;
    let mut good2 = 0;
    while p < len {
        match b[p] as char {
            ' ' => p += 1,
            '\n' => {
                p += 1;
                // validate
                //println!("{:?}", pass);

                if valid(&pass) {
                    good += 1;
                    if valid2(&pass) {
                        good2 += 1;
                    }
                    /*let mut printp = pass
                        .drain()
                        .map(|(k, v)| (String::from_utf8_lossy(&k), String::from_utf8_lossy(&v)))
                        .collect::<Vec<_>>();
                    printp.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
                    println!("{:?}", printp);*/
                }

                pass.clear();
            }
            ('a'..='z') => {
                let k = &b[p..p + 3];
                p += 4;
                let vstart = p;
                while b[p] != ' ' as u8 && b[p] != '\n' as u8 {
                    p += 1;
                }
                pass.insert(k, &b[vstart..p]);
                p += 1;
                //println!("{:?} -> {:?}", k, &b[vstart..p]);
                //p += 1;
            }
            _ => {
                println!("{}", b[p]);
                panic!()
            }
        }
    }
    if valid(&pass) {
        good += 1;
        if valid2(&pass) {
            good2 += 1;
        }
        /*let mut printp = pass
            .drain()
            .map(|(k, v)| (String::from_utf8_lossy(&k), String::from_utf8_lossy(&v)))
            .collect::<Vec<_>>();
        printp.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
        println!("{:?}", printp);*/
    }
    format!("{} {}\n", good, good2)
}

#[cfg(test)]
mod tests {
    use crate::aoc04::valid2;
    #[test]
    fn t() {
        let d = [
            ("byr".as_bytes(), "1920".as_bytes()),
            ("ecl".as_bytes(), "amb".as_bytes()),
            ("eyr".as_bytes(), "2022".as_bytes()),
            ("hcl".as_bytes(), "#09afe2".as_bytes()),
            ("hgt".as_bytes(), "58int".as_bytes()),
            ("iyr".as_bytes(), "2010".as_bytes()),
            ("pid".as_bytes(), "070700006".as_bytes()),
        ]
        .iter()
        .cloned()
        .collect();
        assert_eq!(valid2(&d), true);
    }
}
