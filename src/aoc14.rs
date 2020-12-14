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
    pub(crate) fn insert(&mut self, addr: u64, val: u64, maskx: u64, i: usize) {
        if i != 36 {
            if maskx & (1 << i) == 0 {
                self.insert(addr, val, maskx, i + 1);
            } else {
                self.insert(addr, val, maskx, i + 1);
                self.insert(addr ^ (1 << i), val, maskx, i + 1);
            }
        } else {
            //println!("SET {:b} {}", addr, val);
            self.map.insert(addr, val);
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

                mem2.insert(addr | mask1, data, maskx, 0);

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
