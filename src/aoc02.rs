pub(crate) fn run(b: &[u8]) -> String {
    let len = b.len();
    let mut p = 0;
    let mut good = 0;
    let mut good2 = 0;
    while p < len {
        let mut min = b[p] - '0' as u8;
        p += 1; // possible second number or dash
        while b[p] != '-' as u8 {
            min *= 10;
            min += b[p] - '0' as u8;
            p += 1;
        }
        p += 1; //move over the dash
        let mut max = b[p] - '0' as u8;
        p += 1; // possible second number or space
        while b[p] != ' ' as u8 {
            max *= 10;
            max += b[p] - '0' as u8;
            p += 1;
        }
        p += 1; //move over the space
        let c = b[p];
        p += 2; // move over char and : to stand on space
                // part 2
        if (b[p + min as usize] == c) ^ (b[p + max as usize] == c) {
            good2 += 1
        }
        p += 1; //move over the space
        let mut cnt = 0;
        while b[p] != '\n' as u8 {
            if b[p] == c {
                cnt += 1;
            }
            p += 1
        }
        p += 1;
        if cnt >= min && cnt <= max {
            good += 1;
        }
    }

    format!("{} {}\n", good, good2)
}
