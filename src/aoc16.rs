fn atoi(data: &[u8], p: &mut usize, stop: u8) -> u16 {
    let mut r: u16 = (data[*p] - b'0') as u16;
    *p += 1;
    while data[*p] != stop {
        //dbg!(data[*p]);
        r *= 10;
        r += (data[*p] - b'0') as u16;
        *p += 1;
    }
    r
}

#[derive(Debug)]
struct Field {
    //name: &'a [u8],
    range1_min: u16,
    range1_max: u16,
    range2_min: u16,
    range2_max: u16,
}

pub(crate) fn run(data: &[u8]) -> String {
    //
    let mut p = 0;
    let len = data.len();

    let mut fields = Vec::new();

    loop {
        //let name_start = p;
        while data[p] != b':' {
            p += 1;
        }
        //let name_end = p;
        p += 2;
        //dbg!(data[p] as char);
        let range1_min = atoi(data, &mut p, b'-');
        p += 1;
        //dbg!(data[p] as char);
        let range1_max = atoi(data, &mut p, b' ');
        p += 4;
        //dbg!(data[p] as char);
        let range2_min = atoi(data, &mut p, b'-');
        p += 1;
        //dbg!(data[p] as char);
        let range2_max = atoi(data, &mut p, b'\n');
        p += 1;
        fields.push(Field {
            //name: &data[name_start..name_end],
            range1_min,
            range1_max,
            range2_min,
            range2_max,
        });
        if data[p] == b'\n' {
            break;
        }
    }
    //dbg!(&fields);

    // my ticket
    p += 14;
    //dbg!(data[p] as char);
    let mut my_ticket = Vec::with_capacity(fields.len());
    for _ in 1..fields.len() {
        //dbg!(data[p] as char);
        my_ticket.push(atoi(data, &mut p, b','));
        p += 1;
    }
    my_ticket.push(atoi(data, &mut p, b'\n'));

    p += 18;
    //dbg!(data[p] as char);
    let mut vals = vec![Vec::new(); fields.len()];
    let mut p1 = 0;
    while p < len {
        let mut ticket = Vec::with_capacity(fields.len());
        for _ in 1..fields.len() {
            //dbg!(data[p] as char);
            ticket.push(atoi(data, &mut p, b','));
            p += 1;
        }
        //dbg!(data[p] as char);
        ticket.push(atoi(data, &mut p, b'\n'));
        p += 1;
        let mut good = true;
        for tf in &ticket {
            if fields
                .iter()
                .find(|f| {
                    (*tf >= f.range1_min && *tf <= f.range1_max)
                        || (*tf >= f.range2_min && *tf <= f.range2_max)
                })
                .is_none()
            {
                //dbg!(&ticket);
                //dbg!(tf);
                p1 += *tf as u32;
                good = false;
                break;
            }
        }
        //dbg!(good);
        if good {
            for (x, v) in ticket.into_iter().enumerate() {
                vals[x].push(v as u16);
            }
        }
    }

    //dbg!(p1);

    let mut vals_min = Vec::with_capacity(fields.len());
    let mut vals_max = Vec::with_capacity(fields.len());
    for val in &vals {
        let mut min = 1000;
        let mut max = 0;
        for x in val {
            if *x < min {
                min = *x;
            } else if *x > max {
                max = *x;
            }
        }
        vals_min.push(min);
        vals_max.push(max);
    }
    //dbg!(&vals[0]);

    // Is field x already assigned?
    let mut assignment: Vec<i8> = vec![-1; fields.len()];
    // Bit y of value x is set if vals[y] could be field x.
    let mut options: Vec<u32> = vec![0; fields.len()];
    // Bit y of value x is set if vals[x] could be field y.
    //let mut optionsr: Vec<u32> = vec![0; fields.len()];
    //dbg!(fields.len());
    let mut to_assign = fields.len() as u8;
    for (x, v) in vals.iter().enumerate() {
        let mut possible = 0;
        let mut possible_id: i8 = -1;
        for i in 0..fields.len() {
            if assignment[i] != -1 {
                continue;
            }
            let field = &fields[i];
            if vals_min[x] >= field.range1_min && vals_max[x] <= field.range2_max {
                // thorough check
                if v.iter()
                    .find(|v| *v > &field.range1_max && *v < &field.range2_min)
                    .is_none()
                {
                    //println!("P val {} field {}", x, i);
                    possible += 1;
                    possible_id = i as i8;
                    options[i] |= 1 << x;
                    //optionsr[x] |= 1 << i;
                }
            }
        }
        //dbg!(possible);
        if possible == 1 {
            assignment[possible_id as usize] = x as i8;
            options[possible_id as usize] = 0;
            //optionsr[x as usize] = 0;
            to_assign -= 1;
            //for v in optionsr.iter_mut() {
            //    *v &= !(1 << possible_id);
            //}
        }
    }
    while to_assign > 0 {
        //dbg!(&assignment);
        //dbg!(&options);
        //dbg!(&optionsr);
        //dbg!(to_assign);
        //let old_todo = to_assign;
        //let mut done = 0;
        // check if there are fields with only one possible val
        for x in 0..assignment.len() {
            // }(x, v) in options.iter_mut().enumerate() {
            let v = options[x];
            if v == 0 {
                //done += 1;
                continue;
            }
            let first = v.trailing_zeros();
            //dbg!(first);
            //dbg!(v ^ 1 << first);
            if v ^ 1 << first == 0 {
                // println!("C1: {} {}", x, first);
                assignment[x] = first as i8;
                //dbg!(&assignment);
                options[x] = 0;
                //optionsr[first as usize] = 0;
                to_assign -= 1;
                //done += 1;
                for y in options.iter_mut() {
                    *y &= !(1 << first);
                }
                //for y in optionsr.iter_mut() {
                //    *y &= !(1 << x);
                //}
                //dbg!(&options);
                //dbg!(&optionsr);
            }
        }
        //if (done + to_assign) as usize != assignment.len() {
        //    return None;
        //}
        //done = 0;
        // check if there are vals with only one possible x
        /*for x in 0..assignment.len() {
            // }(x, v) in optionsr.iter_mut().enumerate() {
            let v = optionsr[x];
            if v == 0 {
                continue;
            }
            let first = v.trailing_zeros();
            if v ^ 1 << first == 0 {
                //
                assignment[first as usize] = x as i8;
                options[first as usize] = 0;
                optionsr[x] = 0;
                to_assign -= 1;
                for y in options.iter_mut() {
                    *y &= !(1 << x);
                }
                for y in optionsr.iter_mut() {
                    *y &= !(1 << first);
                }
            }
        }*/
    }
    //dbg!(to_assign);

    //let assign = solve(to_assign, assignment, options, optionsr).unwrap();
    //dbg!(&assign);

    let mut p2: usize = 1;
    for ix in assignment.iter().take(6) {
        p2 *= my_ticket[*ix as usize] as usize;
    }

    format!("{} {}\n", p1, p2)
}
