fn slope(data: &[u8], dx: usize, dy: usize, width: usize) -> usize {
    let d = dx as usize + dy as usize * (width as usize + 1);
    let mut p = d;
    let mut t = 0;
    let len = data.len();
    let mut x = dx;
    while p < len {
        //println!("{} {} {}", x, p, b[p] as char);
        if data[p] == b'#' {
            t += 1
        }
        p += d;
        x += dx;
        if x >= width {
            x -= width;
            p -= width;
        }
    }
    t
}

pub(crate) fn run(b: &[u8]) -> String {
    let width = {
        let mut i = 0;
        while b[i] != b'\n' {
            i += 1;
        }
        i
    };

    //let height = b.len() / (width as usize + 1);

    let ans1 = slope(&b, 3, 1, width);

    //println!("Part 1: {}", ans1);

    let ans2 = ans1
        * slope(&b, 1, 1, width)
        * slope(&b, 5, 1, width)
        * slope(&b, 7, 1, width)
        * slope(&b, 1, 2, width);

    format!("{} {}\n", ans1, ans2)
}
