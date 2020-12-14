use rustc_hash::FxHashMap;

fn atoi(data: &[u8], p: &mut usize, stop: u8) -> u64 {
    let mut r: u64 = (data[*p] - '0' as u8) as u64;
    *p += 1;
    while data[*p] != stop {
        //dbg!(data[*p]);
        r *= 10;
        r += (data[*p] - '0' as u8) as u64;
        *p += 1;
    }
    r
}

#[derive(Default)]
pub(crate) struct FloatMem {
    pub(crate) map: FxHashMap<u64, u64>,
}

impl FloatMem {
    #[inline]
    pub(crate) fn insert(&mut self, mut addr: u64, val: u64, maskx: u64) {
        //dbg!(addr);
        //dbg!(format!("{:0b}", addr));
        let mut s = Vec::new();
        for i in 0..36 {
            if maskx & 1 << i != 0 {
                s.push(1 << i);
                addr &= !(1 << i);
            }
        }
        //dbg!(format!("{:0b}", addr));
        //dbg!(&s);

        loop {
            //println!("ins {}", addr);
            self.map.insert(addr, val);
            let mut p = s.len() - 1;
            while addr & s[p] != 0 {
                if p == 0 {
                    return;
                }
                p -= 1;
            }
            addr |= s[p];
            p += 1;
            while p < s.len() {
                addr &= !(s[p]);
                p += 1;
            }
        }
    }
}

pub(crate) fn run(data: &[u8]) -> String {
    let mut p = 1;
    let len = data.len();

    let mut mask0 = u64::MAX;
    let mut mask1 = 0;
    let mut maskx = 0;

    let mut mem = FxHashMap::default();
    mem.reserve(2048);
    let mut mem2 = FloatMem::default();
    mem2.map.reserve(524288);

    while p < len {
        //dbg!(data[p] as char);
        match data[p] as char {
            'a' => {
                p += 6;
                //dbg!(data[p] as char);
                mask0 = u64::MAX;
                mask1 = 0;
                maskx = 0;
                for i in (0..36).rev() {
                    //dbg!(data[p] as char);
                    match data[p] as char {
                        '0' => mask0 ^= 1 << i,
                        '1' => mask1 |= 1 << i,
                        'X' => maskx |= 1 << i,
                        _ => unreachable!(data[p]),
                    }
                    p += 1;
                }
                p += 2;
            }
            'e' => {
                p += 3;
                let addr = atoi(data, &mut p, ']' as u8);
                p += 4;
                let mut data = atoi(data, &mut p, '\n' as u8);
                p += 2;

                // println!("{:x} {:x} {:x} {:x}", addr, addr | mask1, data, maskx);

                mem2.insert(addr | mask1, data, maskx);

                data &= mask0;
                data |= mask1;
                mem.insert(addr, data);
            }
            _ => unreachable!(data[p] as char),
        }
    }

    let p1 = mem.values().fold(0, |x, y| x + y);
    let p2 = mem2.map.values().fold(0, |x, y| x + y);
    //dbg!(mem.len());
    //dbg!(mem2.map.len());

    format!("{} {}\n", p1, p2)
}
