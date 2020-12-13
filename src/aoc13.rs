pub(crate) fn run(data: &[u8]) -> String {
    let mut p = 1;
    let len = data.len();

    let mut ts: usize = (data[0] - '0' as u8) as usize;

    while p < len && data[p] != '\n' as u8 {
        ts *= 10;
        ts += (data[p] - '0' as u8) as usize;
        p += 1;
    }
    p += 1;

    let mut ids = Vec::with_capacity(16);
    let mut offset = 0;

    while p < len {
        if data[p] == 'x' as u8 {
            p += 2;
            offset += 1;
        } else {
            //dbg!(data[p] as char);
            let mut acc: u16 = (data[p] - '0' as u8) as u16;
            p += 1;
            while data[p] != ',' as u8 && data[p] != '\n' as u8 {
                //dbg!(acc, data[p] as char);
                acc *= 10;
                acc += (data[p] - '0' as u8) as u16;
                p += 1;
            }
            p += 1;
            ids.push((acc, offset % acc));
            offset += 1;
        }
    }
    //dbg!(&ids);

    let mut p1id = ids[0].0;
    let mut wait = ids[0].0 as usize - ts % ids[0].0 as usize;
    for id in &ids[1..] {
        let w = id.0 as usize - ts % id.0 as usize;
        if w < wait {
            p1id = id.0;
            wait = w;
        }
    }
    //dbg!(p1id, wait);
    //dbg!(p1id as usize * wait);

    let mut offset: u64 = (ids[0].0 - ids[0].1) as u64;
    let mut d: u64 = ids[0].0 as u64;
    //dbg!(offset, d);
    for id in &ids[1..] {
        //dbg!(id.0);
        let w = (id.0 - id.1) as u64;
        while offset % id.0 as u64 != w as u64 {
            offset += d;
            //dbg!(offset);
        }
        d *= id.0 as u64;
    }

    //dbg!(d, offset);

    format!("{} {}\n", p1id as usize * wait, offset)
}
