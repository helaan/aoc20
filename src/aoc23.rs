//use std::collections::HashSet;

fn shuf(next: &mut [u32], iters: usize, start: u32) {
    let mut cur = start;
    for _ in 0..iters {
        let c1 = next[cur as usize];
        let c2 = next[c1 as usize];
        let c3 = next[c2 as usize];
        next[cur as usize] = next[c3 as usize];
        let mut target = cur - 1;
        if target == 0 {
            target = next.len() as u32 - 1;
        }
        while target == c1 || target == c2 || target == c3 {
            target -= 1;
            if target == 0 {
                target = next.len() as u32 - 1;
            }
        }
        next[c3 as usize] = next[target as usize];
        next[target as usize] = c1;
        //dbg!(cur, c1, c2, c3, target);
        cur = next[cur as usize];
    }
}

pub(crate) fn run(data: &[u8]) -> String {
    let mut start = Vec::with_capacity(9);

    let mut p = 0;

    while data[p] != b'\n' {
        start.push(data[p] - b'0');
        p += 1;
    }

    let mut next = vec![0; start.len() + 1];
    let mut cur = start[0];
    for x in &start {
        next[cur as usize] = *x as u32;
        cur = *x;
    }
    next[cur as usize] = start[0] as u32;
    let mut next2 = next.clone();
    shuf(&mut next, 100, start[0] as u32);

    let mut p1 = vec![0; start.len() - 1];
    cur = 1;
    for x in p1.iter_mut() {
        cur = next[cur as usize] as u8;
        *x = cur + b'0';
    }
    //dbg!(String::from_utf8(p1).unwrap());

    // p2
    let mut v = start.len() as u32 + 1;
    next2.resize_with(1_000_001, || {
        v += 1;
        v
    });
    next2[1_000_000] = start[0] as u32;
    next2[start[start.len() - 1] as usize] = start.len() as u32 + 1;

    /*let mut hs = HashSet::new();
    for x in &next2 {
        if !hs.insert(x.clone()) {
            dbg!(x);
        }
    }*/

    //dbg!(&next2);
    shuf(&mut next2, 10_000_000, start[0] as u32);
    //dbg!(&next2);
    let p2a = next2[1] as u64;
    let p2b = next2[p2a as usize] as u64;
    let p2 = p2a * p2b;
    //dbg!(p2a, p2b, p2);

    format!("{} {}\n", String::from_utf8(p1).unwrap(), p2)
}
