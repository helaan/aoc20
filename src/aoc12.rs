// North and east are positive

// E S W N
const DIRS: [(i8, i8); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

pub(crate) fn run(data: &[u8]) -> String {
    let mut d1 = 0;
    let mut x1: isize = 0;
    let mut y1: isize = 0;

    let mut x2: isize = 0;
    let mut y2: isize = 0;
    let mut dx2: isize = 1;
    let mut dy2: isize = 10;

    let mut p = 0;
    while p < data.len() {
        //dbg!(x2, y2, dx2, dy2);
        let cmd = data[p];
        //dbg!(cmd as char, acc);
        p += 1;

        if cmd == b'L' {
            match data[p] {
                b'9' => {
                    d1 += 3;
                    p += 3;
                    let ndy = -dx2;
                    dx2 = dy2;
                    dy2 = ndy;
                }
                b'1' => {
                    d1 += 2;
                    p += 4;
                    dx2 *= -1;
                    dy2 *= -1;
                }
                b'2' => {
                    d1 += 1;
                    p += 4;
                    let ndy = dx2;
                    dx2 = -dy2;
                    dy2 = ndy;
                }
                _ => unreachable!(data[p]),
            }
            d1 %= 4;
            continue;
        } else if cmd == b'R' {
            match data[p] {
                b'9' => {
                    d1 += 1;
                    p += 3;
                    let ndy = dx2;
                    dx2 = -dy2;
                    dy2 = ndy;
                }
                b'1' => {
                    d1 += 2;
                    p += 4;
                    dx2 *= -1;
                    dy2 *= -1;
                }
                b'2' => {
                    d1 += 3;
                    p += 4;
                    let ndy = -dx2;
                    dx2 = dy2;
                    dy2 = ndy;
                }
                _ => unreachable!(data[p]),
            }
            d1 %= 4;
            continue;
        }

        let mut acc: u16 = data[p] as u16 - '0' as u16;
        p += 1;
        while data[p] != b'\n' {
            acc *= 10;
            acc += data[p] as u16 - '0' as u16;
            p += 1;
        }
        p += 1;

        match cmd {
            b'N' => {
                x1 += acc as isize;
                dx2 += acc as isize;
            }
            b'S' => {
                x1 -= acc as isize;
                dx2 -= acc as isize;
            }
            b'E' => {
                y1 += acc as isize;
                dy2 += acc as isize;
            }
            b'W' => {
                y1 -= acc as isize;
                dy2 -= acc as isize;
            }

            b'F' => {
                x1 += acc as isize * DIRS[d1 as usize].0 as isize;
                y1 += acc as isize * DIRS[d1 as usize].1 as isize;
                x2 += dx2 * acc as isize;
                y2 += dy2 * acc as isize;
            }

            _ => unreachable!(cmd),
        }
    }
    let p1 = x1.abs() + y1.abs();
    //dbg!(p1, x1, y1);
    let p2 = x2.abs() + y2.abs();
    //dbg!(p2, x2, y2);

    format!("{} {}\n", p1, p2)
}
