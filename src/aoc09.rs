use rustc_hash::FxHashSet;

const PREAMBLE_SIZE: usize = 25;

pub(crate) fn run(data: &[u8]) -> String {
    //    let mut p = 0;
    //  let len = data.len();

    let mut num = Vec::new();

    let mut x: i64 = 0;
    data.iter().for_each(|b| match *b {
        b'\n' => {
            num.push(x);
            x = 0;
        }
        i => {
            x *= 10;
            x += i as i64 - '0' as i64;
        }
    });

    let mut set = FxHashSet::default();
    for i in num.iter().take(PREAMBLE_SIZE) {
        set.insert(*i);
    }

    let mut p1 = 0;

    for i in PREAMBLE_SIZE..num.len() {
        let target = num[i];
        let mut contain = false;
        for p in num.iter().skip(i - PREAMBLE_SIZE).take(PREAMBLE_SIZE) {
            if set.contains(&(target - *p)) {
                contain = true;
                break;
            }
        }
        if !contain {
            p1 = num[i];
            //println!("{}", p1);
            break;
        }

        set.insert(num[i]);
        set.remove(&(num[i - PREAMBLE_SIZE]));
    }

    let mut x = num[0];
    let mut start = 0;
    let mut end = 0;
    for i in 1..num.len() {
        x += num[i];
        //println!("x: {}, {}, {}", x, num[i], start);
        while x > p1 {
            x -= num[start];
            start += 1;
        }
        if x == p1 {
            end = i;
            break;
        }
    }
    let mut min = num[start];
    let mut max = num[start];
    for d in num.iter().take(end + 1).skip(start + 1) {
        if *d < min {
            min = *d
        }
        if *d > max {
            max = *d
        }
        //println!("{} {} {}", d, min, max);
    }
    //println!("{} {}", min, max);

    //println!("{} {}\n", p1, min + max);
    format!("{} {}\n", p1, min + max)
}
