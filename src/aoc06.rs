fn bitcnt(value: &u32) -> u32 {
    let mut a = 0;
    let mut x = *value;
    while x != 0 {
        x &= x - 1;
        a += 1;
    }
    a
}

pub(crate) fn run(b: &[u8]) -> String {
    let len = b.len();
    let mut p = 0;
    let mut ans = 0;
    let mut ans2: u32 = 0xffffffff;

    let mut sum = 0;
    let mut sum2 = 0;

    while p < len {
        if b[p] == '\n' as u8 {
            sum += bitcnt(&ans);
            ans = 0;
            sum2 += bitcnt(&ans2);
            ans2 = 0xffffffff;
        } else {
            let mut l = 0;
            while b[p] != '\n' as u8 {
                let bit = 1 << (b[p] - 'a' as u8);
                ans |= bit;
                l |= bit;
                p += 1;
            }
            ans2 &= l;
        }
        p += 1;
    }
    sum += bitcnt(&ans);
    sum2 += bitcnt(&ans2);

    format!("{} {}\n", sum, sum2)
}
