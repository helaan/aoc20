use rustc_hash::{FxHashMap, FxHashSet};

fn traverse1<'a>(
    rev: &FxHashMap<&'a [u8], Vec<&'a [u8]>>,
    res: &mut FxHashSet<&'a [u8]>,
    item: &'a [u8],
) {
    if let Some(v) = rev.get(item) {
        for x in v {
            if res.insert(x) {
                traverse1(&rev, res, x);
            }
        }
    }
}

fn traverse2<'a>(tree: &FxHashMap<&[u8], Vec<(u8, &[u8])>>, x: &[u8]) -> usize {
    let mut ans = 0;
    if let Some(v) = tree.get(x) {
        for y in v {
            let z = traverse2(tree, y.1) + 1;
            ans += z * y.0 as usize;
        }
    }
    ans
}

pub(crate) fn run(data: &[u8]) -> String {
    // bags this bag contains
    let mut tree: FxHashMap<&[u8], Vec<(u8, &[u8])>> = FxHashMap::default();
    // bags that contain this bag
    let mut rev: FxHashMap<&[u8], Vec<&[u8]>> = FxHashMap::default();

    let mut p = 0;
    let len = data.len();
    while p < len {
        let name_start = p;
        while data[p] != ' ' as u8 {
            p += 1;
        }
        p += 1;
        while data[p] != ' ' as u8 {
            p += 1;
        } // now on space before bags
        let name = &data[name_start..p];
        //println!("name: '{}'", std::str::from_utf8(name).unwrap());
        p += 14; // skip over "bags contain "
        let mut contents = Vec::new();
        loop {
            //println!("n: '{}'", data[p] as char);
            let n = data[p] - '0' as u8;
            if n >= 9 {
                p += 15;
                break;
            }
            p += 2;
            let cont_start = p;
            while data[p] != ' ' as u8 {
                p += 1;
            }
            p += 1;
            while data[p] != ' ' as u8 {
                p += 1;
            } // now on space before bags
            let cont_name = &data[cont_start..p];
            //println!("cname: '{}'", std::str::from_utf8(cont_name).unwrap());
            contents.push((n, cont_name));
            if n == 1 {
                p += 4;
            } else {
                p += 5;
            }
            //println!("d: '{}'", data[p] as char);
            if data[p] == '.' as u8 {
                p += 2; // not sure why not 1 but this works
                break;
            } else if data[p] == ',' as u8 {
                // more bags
                p += 2;
            } else {
                unreachable!()
            }
        }
        //println!("{:?}", contents);

        for (_, bag) in &contents {
            if let Some(r) = rev.get_mut(bag) {
                r.push(name);
            } else {
                rev.insert(bag, vec![name]);
            }
        }
        tree.insert(name, contents);
    }
    //println!("{:?}", tree);
    let mut res = FxHashSet::default();
    traverse1(&rev, &mut res, b"shiny gold");
    //println!("{}", res.len());
    let p2 = traverse2(&tree, b"shiny gold");

    format!("{} {}\n", res.len(), p2)
}
