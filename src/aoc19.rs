use rustc_hash::FxHashMap;

fn atoi(data: &[u8], p: &mut usize) -> u8 {
    //dbg!(data[*p] as char);
    let mut r: u8 = (data[*p] - b'0') as u8;
    *p += 1;
    while data[*p] >= b'0' && data[*p] <= b'9' {
        //dbg!(data[*p]);
        r *= 10;
        r += (data[*p] - b'0') as u8;
        *p += 1;
    }
    r
}

#[derive(Debug, Clone)]
enum Rule {
    Literal(u8),

    Reference(u8),
    Composition(u8, u8),
    TComposition(u8, u8, u8), //example only
}

#[derive(Debug, Clone)]
enum OptionRule {
    Single(Rule),
    Double(Rule, Rule),
}

fn eval(rules: &FxHashMap<u8, OptionRule>, s: &[u8], i: usize, r: u8) -> Option<usize> {
    match rules.get(&r).unwrap() {
        OptionRule::Single(x) => evalo(rules, s, i, x),
        OptionRule::Double(x, y) => {
            if let Some(j) = evalo(rules, s, i, x) {
                Some(j)
            } else {
                evalo(rules, s, i, y)
            }
        }
    }
}

fn evalo(rules: &FxHashMap<u8, OptionRule>, s: &[u8], i: usize, r: &Rule) -> Option<usize> {
    match r {
        Rule::Literal(c) => {
            if i >= s.len() {
                None
            } else if s[i] == *c {
                Some(i + 1)
            } else {
                None
            }
        }
        Rule::Reference(x) => eval(rules, s, i, *x),
        Rule::Composition(x, y) => {
            if let Some(j) = eval(rules, s, i, *x) {
                eval(rules, s, j, *y)
            } else {
                None
            }
        }
        Rule::TComposition(x, y, z) => {
            if let Some(j) = eval(rules, s, i, *x) {
                if let Some(k) = eval(rules, s, j, *y) {
                    eval(rules, s, k, *z)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

fn evalr(rules: &FxHashMap<u8, OptionRule>, s: &[u8], i: usize, r: u8) -> Vec<usize> {
    if [0, 8, 11].contains(&r) {
        if let Some(rule) = rules.get(&r) {
            match rule {
                OptionRule::Double(x, y) => {
                    let mut r = Vec::new();
                    r.extend(evalor(rules, s, i, x));
                    r.extend(evalor(rules, s, i, y));
                    r
                }
                OptionRule::Single(x) => evalor(rules, s, i, x),
            }
        } else {
            unreachable!()
        }
    } else {
        if let Some(r) = eval(rules, s, i, r) {
            vec![r]
        } else {
            Vec::new()
        }
    }
    //
}

fn evalor(rules: &FxHashMap<u8, OptionRule>, s: &[u8], i: usize, r: &Rule) -> Vec<usize> {
    match r {
        Rule::Literal(c) => {
            if i >= s.len() {
                Vec::new()
            } else if s[i] == *c {
                vec![i + 1]
            } else {
                Vec::new()
            }
        }
        Rule::Reference(x) => evalr(rules, s, i, *x),
        Rule::Composition(x, y) => {
            let opts = evalr(rules, s, i, *x);
            let mut r = Vec::new();
            for opt in opts {
                r.extend(evalr(rules, s, opt, *y));
            }
            r
        }
        Rule::TComposition(x, y, z) => {
            let opts = evalr(rules, s, i, *x);
            let mut r = Vec::new();
            for opt in opts {
                r.extend(evalr(rules, s, opt, *y));
            }
            let mut r2 = Vec::new();
            for opt in r {
                r2.extend(evalr(rules, s, opt, *z));
            }
            r2
        }
    }
}

pub(crate) fn run(data: &[u8]) -> String {
    let mut rules = FxHashMap::default();
    let mut p = 0;
    let mut buf = Vec::with_capacity(5);
    while p < data.len() {
        if data[p] == b'\n' {
            p += 1;
            break;
        }
        let id = atoi(&data, &mut p);
        p += 2;
        let rule = if data[p] == b'"' {
            let r = OptionRule::Single(Rule::Literal(data[p + 1]));
            p += 4;
            r
        } else {
            let mut rule_a = None;
            buf.push(atoi(&data, &mut p));
            while data[p] != b'\n' {
                p += 1;
                if data[p] == b'|' {
                    rule_a = Some(match buf.len() {
                        1 => Rule::Reference(buf[0]),
                        2 => Rule::Composition(buf[0], buf[1]),
                        _ => unreachable!(buf.len()),
                    });
                    buf.clear();
                    p += 1;
                } else {
                    buf.push(atoi(&data, &mut p));
                }
            }
            p += 1;
            //dbg!(&buf);
            let rule_b = match buf.len() {
                1 => Rule::Reference(buf[0]),
                2 => Rule::Composition(buf[0], buf[1]),
                3 => Rule::TComposition(buf[0], buf[1], buf[2]),
                _ => unreachable!(buf.len()),
            };
            buf.clear();
            if let Some(a) = rule_a {
                OptionRule::Double(a, rule_b)
            } else {
                OptionRule::Single(rule_b)
            }
        };
        // dbg!(id, &rule);
        rules.insert(id, rule);
    }
    //dbg!(&rules);
    let mut rules2 = rules.clone();
    rules2.insert(
        8,
        OptionRule::Double(Rule::Reference(42), Rule::Composition(42, 8)),
    );
    rules2.insert(
        11,
        OptionRule::Double(Rule::Composition(42, 31), Rule::TComposition(42, 11, 31)),
    );

    let mut p1 = 0;
    let mut p2 = 0;
    while p < data.len() {
        let pstart = p;
        p += 1;
        while data[p] != b'\n' {
            p += 1;
        }
        //dbg!(&data[pstart..p]);
        if let Some(i) = eval(&rules, &data[pstart..p], 0, 0) {
            //dbg!(i, p - pstart);
            if i == p - pstart {
                p1 += 1;
            }
        }
        let r = evalr(&rules2, &data[pstart..p], 0, 0);
        if r.contains(&(p - pstart)) {
            p2 += 1;
        }
        p += 1;
    }
    //dbg!(p1, p2);

    format!("{} {}\n", p1, p2)
}
