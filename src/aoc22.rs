use std::{
    collections::{hash_map::DefaultHasher, HashSet, VecDeque},
    hash::Hash,
    hash::Hasher,
};

fn atoi(data: &[u8], p: &mut usize, stop: u8) -> u8 {
    let mut r: u8 = data[*p] - b'0';
    *p += 1;
    while data[*p] != stop {
        //dbg!(data[*p]);
        r *= 10;
        r += data[*p] - b'0';
        *p += 1;
    }
    r
}

// true if p1 wins
fn rec(d1: &mut VecDeque<u8>, d2: &mut VecDeque<u8>) -> bool {
    let mut seen = HashSet::new();

    while !(d1.is_empty() || d2.is_empty()) {
        //dbg!(&d1, &d2);
        let mut hasher = DefaultHasher::new();
        (&d1, &d2).hash(&mut hasher);
        let hash = hasher.finish();
        //dbg!(&hash);
        if !seen.insert(hash) {
            //dbg!(true);
            return true;
        }
        let c1 = d1.pop_front().unwrap();
        let c2 = d2.pop_front().unwrap();
        let winner = if d1.len() >= c1 as usize && d2.len() >= c2 as usize {
            //dbg!("recurse");
            //dbg!(d1.len() + d2.len());
            let mut dd1 = d1.clone();
            while dd1.len() > c1 as usize {
                dd1.pop_back();
            }
            let mut dd2 = d2.clone();
            while dd2.len() > c2 as usize {
                dd2.pop_back();
            }
            rec(&mut dd1, &mut dd2)
        } else {
            c1 > c2
        };
        if winner {
            d1.push_back(c1);
            d1.push_back(c2);
        } else {
            d2.push_back(c2);
            d2.push_back(c1);
        }
    }
    //dbg!(d2.is_empty());
    d2.is_empty()
}

fn score(mut d: VecDeque<u8>) -> u32 {
    let mut r: u32 = 0;
    let mut smul = 1;
    while let Some(x) = d.pop_back() {
        r += x as u32 * smul;
        smul += 1;
    }
    r
}

pub(crate) fn run(data: &[u8]) -> String {
    let mut p = 10;

    let mut d1 = VecDeque::new();
    let mut d2 = VecDeque::new();

    // dbg!(data[p] as char);
    while data[p] != b'\n' {
        d1.push_back(atoi(&data, &mut p, b'\n'));
        p += 1;
    }
    p += 11;
    // dbg!(data[p] as char);
    while p < data.len() {
        d2.push_back(atoi(&data, &mut p, b'\n'));
        p += 1;
    }

    let mut dd1 = d1.clone();
    let mut dd2 = d2.clone();

    while !(d1.is_empty() || d2.is_empty()) {
        let c1 = d1.pop_front().unwrap();
        let c2 = d2.pop_front().unwrap();
        if c1 > c2 {
            d1.push_back(c1);
            d1.push_back(c2);
        } else {
            d2.push_back(c2);
            d2.push_back(c1);
        }
    }
    let p1 = score(d1) + score(d2);
    let p2 = if rec(&mut dd1, &mut dd2) {
        score(dd1)
    } else {
        score(dd2)
    };

    format!("{} {}\n", p1, p2)
}
