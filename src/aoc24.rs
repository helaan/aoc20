use rustc_hash::FxHashSet;

pub const NEIGHBORS: [(i8, i8); 6] = [(0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1)];

pub(crate) fn run(data: &[u8]) -> String {
    let mut p = 0;
    let mut tiles = FxHashSet::default();

    let mut x = 0;
    let mut y = 0;

    while p < data.len() {
        match data[p] {
            b'\n' => {
                if !tiles.insert((x, y)) {
                    tiles.remove(&(x, y));
                }
                x = 0;
                y = 0;
            }
            b'e' => {
                x += 1;
            }
            b'w' => {
                x -= 1;
            }
            b'n' => {
                y -= 1;
                p += 1;
                if data[p] == b'e' {
                    x += 1;
                }
            }
            b's' => {
                y += 1;
                p += 1;
                if data[p] == b'w' {
                    x -= 1;
                }
            }
            _ => unreachable!(data[p] as char),
        }
        p += 1;
    }

    //   let p1 = tiles.values().filter(|v| **v).count();
    let p1 = tiles.len();

    let mut minx = 0;
    let mut miny = 0;
    let mut maxx = 0;
    let mut maxy = 0;

    for (x, y) in &tiles {
        if *x < minx {
            minx = *x;
        } else if *x > maxx {
            maxx = *x;
        }
        if *y < miny {
            miny = *y;
        } else if *y > maxy {
            maxy = *y;
        }
    }

    for _ in 0..100 {
        let mut new = FxHashSet::default();
        for x in minx - 1..maxx + 2 {
            for y in miny - 1..maxy + 2 {
                let cnt = NEIGHBORS
                    .iter()
                    .filter(|(dx, dy)| tiles.contains(&(x + dx, y + dy)))
                    .count();
                if cnt == 2 || (cnt == 1 && tiles.contains(&(x, y))) {
                    new.insert((x, y));
                    if x < minx {
                        minx = x;
                    } else if x > maxx {
                        maxx = x;
                    }
                    if y < miny {
                        miny = y;
                    } else if y > maxy {
                        maxy = y;
                    }
                }
            }
        }
        tiles = new;
    }
    //dbg!(minx, maxx, miny, maxy);
    let p2 = tiles.len();

    format!("{} {}\n", p1, p2)
}
