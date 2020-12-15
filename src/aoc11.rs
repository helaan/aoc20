use bit_set::BitSet;

pub(crate) fn run(data: &[u8]) -> String {
    let width = (0..data.len()).find(|x| data[*x] == b'\n').unwrap();
    let wwidth = width + 1;
    //dbg!(width);
    let height = data.len() / wwidth;
    //dbg!(height);

    let sjoemel = wwidth + 1;

    let dirs: Vec<i16> = vec![
        -1 as i16 - wwidth as i16,
        -(wwidth as i16),
        1 - wwidth as i16,
        -1,
        1,
        -1 + wwidth as i16,
        wwidth as i16,
        1 + wwidth as i16,
    ];

    let p1 = {
        let mut state_a = BitSet::with_capacity(wwidth * (height + 2) + 2);
        let mut state_b = BitSet::with_capacity(wwidth * (height + 2) + 2);

        for y in 0..height {
            let ybase = y * (wwidth);
            for x in 0..width {
                if data[ybase + x] == b'L' {
                    state_a.insert(ybase + x + sjoemel);
                }
            }
        }
        /*if data[0] == 'L' as u8 {
            state_b.insert(0 + sjoemel);
        }
        if data[width - 1] == 'L' as u8 {
            state_b.insert(sjoemel + width - 1);
        }
        if data[(height - 1) * wwidth] == 'L' as u8 {
            state_b.insert(height - 1 + sjoemel);
        }
        if data[data.len() - 1] == 'L' as u8 {
            state_b.insert(data.len() - 1 + sjoemel);
        }*/

        let mut state_old = &mut state_a;
        let mut state_new = &mut state_b;
        let mut changes = true;

        while changes {
            // debug
            /*for y in 0..height + 2 {
                let ybase = wwidth * y;
                for x in 0..width + 2 {
                    print!(
                        "{}",
                        if state_old.contains(ybase + x) {
                            '#'
                        } else {
                            ' '
                        }
                    );
                }
                println!("|");
            }*/
            changes = false;
            // top
            for y in 0..height {
                let ybase = wwidth * y + sjoemel;
                let yrealbase = wwidth * y;
                for x in 0..width {
                    if state_old.contains(ybase + x) {
                        // survive
                        let mut cnt = 0;
                        //dbg!(ybase + x);
                        for pos in &[
                            ybase + x - 1 - wwidth,
                            ybase + x - wwidth,
                            ybase + x + 1 - wwidth,
                            ybase + x - 1,
                            ybase + x + 1,
                            ybase + x - 1 + wwidth,
                            ybase + x + wwidth,
                            ybase + x + 1 + wwidth,
                        ] {
                            if state_old.contains(*pos) {
                                cnt += 1;
                            }
                        }
                        if cnt >= 4 {
                            state_new.remove(ybase + x);
                            changes = true;
                        } else {
                            state_new.insert(ybase + x);
                        }
                    } else if data[yrealbase + x] == b'L'
                        && !state_old.contains(ybase + x - 1 - wwidth)
                        && !state_old.contains(ybase + x - wwidth)
                        && !state_old.contains(ybase + x + 1 - wwidth)
                        && !state_old.contains(ybase + x - 1)
                        && !state_old.contains(ybase + x + 1)
                        && !state_old.contains(ybase + x - 1 + wwidth)
                        && !state_old.contains(ybase + x + wwidth)
                        && !state_old.contains(ybase + x + 1 + wwidth)
                    {
                        state_new.insert(ybase + x);
                        changes = true;
                    } else {
                        state_new.remove(ybase + x);
                    }
                }
            }
            /*let tmp = state_old;
            state_old = state_new;
            state_new = tmp;*/
            std::mem::swap(&mut state_old, &mut state_new);
        }

        let mut occupied = 0;
        state_old.iter().for_each(|_| occupied += 1);
        //println!("{}", occupied);
        occupied
    };
    let p2 = {
        let mut chairs = vec![];
        let mut chairix = vec![99999; data.len()];
        let mut n = 0;

        for y in 0..height {
            let ybase = y * wwidth;
            for x in 0..width {
                if data[ybase + x] == b'L' {
                    chairix[ybase + x] = n;
                    n += 1;
                }
            }
        }

        for y in 0..height {
            let ybase = y * (wwidth);
            for x in 0..width {
                if data[ybase + x] == b'L' {
                    chairs.push((
                        chairix[ybase + x],
                        dirs.iter()
                            .filter_map(|d| {
                                let mut p: isize = (ybase + x) as isize + *d as isize;
                                loop {
                                    if p < 0
                                        || p as usize >= data.len()
                                        || p as usize % wwidth == width
                                    {
                                        return None;
                                    }
                                    if data[p as usize] == b'L' {
                                        return Some(chairix[p as usize]);
                                    }
                                    p += *d as isize;
                                }
                            })
                            .collect::<Vec<_>>(),
                    ));
                }
            }
        }

        let mut state_a = BitSet::with_capacity(n);
        let mut state_b = BitSet::with_capacity(n);

        let mut state_old = &mut state_a;
        let mut state_new = &mut state_b;
        let mut changes = true;
        while changes {
            changes = false;
            for (id, neigh) in &chairs {
                if state_old.contains(*id) {
                    // survive
                    let mut cnt = 0;
                    for pos in neigh {
                        if state_old.contains(*pos) {
                            cnt += 1;
                        }
                    }
                    if cnt >= 5 {
                        state_new.remove(*id);
                        changes = true;
                    } else {
                        state_new.insert(*id);
                    }
                } else {
                    // create
                    if neigh.iter().find(|p| state_old.contains(**p)).is_none() {
                        state_new.insert(*id);
                        changes = true;
                    } else {
                        state_new.remove(*id);
                    }
                }
            }
            //let tmp = state_old;
            //state_old = state_new;
            //state_new = tmp;
            std::mem::swap(&mut state_old, &mut state_new);
        }
        let mut occupied = 0;
        state_old.iter().for_each(|_| occupied += 1);
        //println!("{}", occupied);
        occupied
    };
    format!("{} {}\n", p1, p2)
}
