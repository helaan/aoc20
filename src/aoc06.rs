pub(crate) fn run(b: &[u8]) -> String {
    let len = b.len();
    let mut p = 0;
    let mut ans: u32 = 0;
    let mut ans2: u32 = 0xffffffff;

    let mut sum = 0;
    let mut sum2 = 0;

    let mut l = 0;

    while p < len {
        if b[p] != b'\n' {
            l |= 1 << (b[p] - b'a');
        } else if l != 0 {
            ans |= l;
            ans2 &= l;
            l = 0;
        } else {
            sum += &ans.count_ones();
            sum2 += &ans2.count_ones();
            ans = 0;
            ans2 = 0xffffffff;
        }
        p += 1;
    }
    sum += &ans.count_ones();
    sum2 += &ans2.count_ones();

    format!("{} {}\n", sum, sum2)
}
