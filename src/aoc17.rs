use rustc_hash::FxHashSet;

const NEIGHBORS: [(i8, i8, i8); 26] = [
    (-1, -1, -1),
    (0, -1, -1),
    (1, -1, -1),
    (-1, 0, -1),
    (0, 0, -1),
    (1, 0, -1),
    (-1, 1, -1),
    (0, 1, -1),
    (1, 1, -1),
    (-1, -1, 0),
    (0, -1, 0),
    (1, -1, 0),
    (-1, 0, 0),
    (1, 0, 0),
    (-1, 1, 0),
    (0, 1, 0),
    (1, 1, 0),
    (-1, -1, 1),
    (0, -1, 1),
    (1, -1, 1),
    (-1, 0, 1),
    (0, 0, 1),
    (1, 0, 1),
    (-1, 1, 1),
    (0, 1, 1),
    (1, 1, 1),
];

const NEIGHBORS2: [(i8, i8, i8, i8); 80] = [
    (-1, -1, -1, -1),
    (0, -1, -1, -1),
    (1, -1, -1, -1),
    (-1, 0, -1, -1),
    (0, 0, -1, -1),
    (1, 0, -1, -1),
    (-1, 1, -1, -1),
    (0, 1, -1, -1),
    (1, 1, -1, -1),
    (-1, -1, 0, -1),
    (0, -1, 0, -1),
    (1, -1, 0, -1),
    (-1, 0, 0, -1),
    (0, 0, 0, -1),
    (1, 0, 0, -1),
    (-1, 1, 0, -1),
    (0, 1, 0, -1),
    (1, 1, 0, -1),
    (-1, -1, 1, -1),
    (0, -1, 1, -1),
    (1, -1, 1, -1),
    (-1, 0, 1, -1),
    (0, 0, 1, -1),
    (1, 0, 1, -1),
    (-1, 1, 1, -1),
    (0, 1, 1, -1),
    (1, 1, 1, -1),
    (-1, -1, -1, 0),
    (0, -1, -1, 0),
    (1, -1, -1, 0),
    (-1, 0, -1, 0),
    (0, 0, -1, 0),
    (1, 0, -1, 0),
    (-1, 1, -1, 0),
    (0, 1, -1, 0),
    (1, 1, -1, 0),
    (-1, -1, 0, 0),
    (0, -1, 0, 0),
    (1, -1, 0, 0),
    (-1, 0, 0, 0),
    (1, 0, 0, 0),
    (-1, 1, 0, 0),
    (0, 1, 0, 0),
    (1, 1, 0, 0),
    (-1, -1, 1, 0),
    (0, -1, 1, 0),
    (1, -1, 1, 0),
    (-1, 0, 1, 0),
    (0, 0, 1, 0),
    (1, 0, 1, 0),
    (-1, 1, 1, 0),
    (0, 1, 1, 0),
    (1, 1, 1, 0),
    (-1, -1, -1, 1),
    (0, -1, -1, 1),
    (1, -1, -1, 1),
    (-1, 0, -1, 1),
    (0, 0, -1, 1),
    (1, 0, -1, 1),
    (-1, 1, -1, 1),
    (0, 1, -1, 1),
    (1, 1, -1, 1),
    (-1, -1, 0, 1),
    (0, -1, 0, 1),
    (1, -1, 0, 1),
    (-1, 0, 0, 1),
    (0, 0, 0, 1),
    (1, 0, 0, 1),
    (-1, 1, 0, 1),
    (0, 1, 0, 1),
    (1, 1, 0, 1),
    (-1, -1, 1, 1),
    (0, -1, 1, 1),
    (1, -1, 1, 1),
    (-1, 0, 1, 1),
    (0, 0, 1, 1),
    (1, 0, 1, 1),
    (-1, 1, 1, 1),
    (0, 1, 1, 1),
    (1, 1, 1, 1),
];

pub(crate) fn run(data: &[u8]) -> String {
    let mut dim = FxHashSet::default();
    let mut dim2 = FxHashSet::default();

    let mut min = (0, 0, 0);
    let mut max = (0, 0, 0);
    let min2 = (0, 0, 0, 0);
    let mut max2 = (0, 0, 0, 0);
    {
        let mut prev_x = 0;
        let mut x = 0;
        let mut y = 0;
        for c in data {
            match c {
                b'\n' => {
                    y += 1;
                    prev_x = x;
                    x = 0
                }
                b'.' => x += 1,
                b'#' => {
                    dim.insert((x, y, 0));
                    dim2.insert((x, y, 0, 0));
                    x += 1;
                }
                _ => unreachable!(*c as char),
            }
        }
        max.0 = prev_x;
        max.1 = y;
        max2.0 = prev_x;
        max2.1 = y;
    }

    for i in 1..=6 {
        //dbg!(dim.len());
        let mut new_dim = FxHashSet::default();
        for x in min.0 - 1..=max.0 + 1 {
            for y in min.1 - 1..=max.1 + 1 {
                for z in min.2 - 1..=max.2 + 1 {
                    let mut active = 0;
                    for n in &NEIGHBORS {
                        if dim.contains(&(x + n.0, y + n.1, z + n.2)) {
                            active += 1;
                        }
                    }
                    //dbg!((x, y, z), active);
                    if active == 3 || (active == 2 && dim.contains(&(x, y, z))) {
                        new_dim.insert((x, y, z));

                        if x < min.0 {
                            min.0 = x;
                        } else if x > max.0 {
                            max.0 = x
                        }
                        if y < min.1 {
                            min.1 = y;
                        } else if y > max.1 {
                            max.1 = y;
                        }
                        if z < min.2 {
                            min.2 = z;
                        } else if z > max.2 {
                            max.2 = z;
                        }
                    }
                }
            }
        }
        /*dbg!(min, max);
        for z in min.2..=max.2 {
            println!("z={}", z);
            for y in min.1..=max.1 {
                for x in min.0..=max.0 {
                    if new_dim.contains(&(x, y, z)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
        dbg!(dim.len(), new_dim.len());*/
        dim = new_dim;
    }

    let p1 = dim.len();
    //dbg!(p1);

    let mut dim = dim2;
    let mut min = min2;
    let mut max = max2;

    for i in 1..=6 {
        //dbg!(dim.len());
        let mut new_dim = FxHashSet::default();
        for x in min.0 - 1..=max.0 + 1 {
            for y in min.1 - 1..=max.1 + 1 {
                for z in min.2 - 1..=max.2 + 1 {
                    for w in min.3 - 1..=max.3 + 1 {
                        let mut active = 0;
                        for n in &NEIGHBORS2 {
                            if dim.contains(&(x + n.0, y + n.1, z + n.2, w + n.3)) {
                                active += 1;
                            }
                        }
                        //dbg!((x, y, z), active);
                        if active == 3 || (active == 2 && dim.contains(&(x, y, z, w))) {
                            new_dim.insert((x, y, z, w));

                            if x < min.0 {
                                min.0 = x;
                            } else if x > max.0 {
                                max.0 = x
                            }
                            if y < min.1 {
                                min.1 = y;
                            } else if y > max.1 {
                                max.1 = y;
                            }
                            if z < min.2 {
                                min.2 = z;
                            } else if z > max.2 {
                                max.2 = z;
                            }
                            if w < min.3 {
                                min.3 = w;
                            } else if w > max.3 {
                                max.3 = w;
                            }
                        }
                    }
                }
            }
        }
        /*dbg!(min, max);
        for z in min.2..=max.2 {
            println!("z={}", z);
            for y in min.1..=max.1 {
                for x in min.0..=max.0 {
                    if new_dim.contains(&(x, y, z)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }
        dbg!(dim.len(), new_dim.len());*/
        dim = new_dim;
    }
    let p2 = dim.len();
    //dbg!(p2);

    format!("{} {}\n", p1, p2)
}
