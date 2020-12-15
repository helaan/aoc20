use bit_set::BitSet;

enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn eval(ops: &[Op], corrupt: usize) -> Option<i32> {
    let mut visited = BitSet::with_capacity(ops.len());
    let mut ip = 0;
    let mut acc: i32 = 0;

    while ip < ops.len() {
        if !visited.insert(ip) {
            return None;
        }

        match ops.get(ip).unwrap() {
            Op::Acc(x) => {
                acc += x;
                ip += 1;
            }
            Op::Nop(x) => {
                if corrupt == ip {
                    ip = (x + ip as i32) as usize
                } else {
                    ip += 1
                }
            }
            Op::Jmp(x) => {
                if corrupt == ip {
                    ip += 1
                } else {
                    ip = (x + ip as i32) as usize
                }
            }
        }
    }

    Some(acc)
}

pub(crate) fn run(data: &[u8]) -> String {
    let mut p = 0;
    let len = data.len();

    let mut ops = Vec::new();

    while p < len {
        let opid = p;
        let sigid = p + 4;
        p += 5;
        let mut o = data[p] as i32 - '0' as i32;
        p += 1;
        while data[p] != b'\n' {
            o *= 10;
            o += data[p] as i32 - '0' as i32;
            p += 1;
        }
        p += 1;
        if data[sigid] == b'-' {
            o *= -1;
        }
        ops.push(match data[opid] {
            b'a' => Op::Acc(o),
            b'j' => Op::Jmp(o),
            b'n' => Op::Nop(o),
            x => unreachable!(x),
        })
    }
    let mut visited = BitSet::with_capacity(ops.len());
    let mut ip = 0;
    let mut acc: i32 = 0;
    loop {
        if !visited.insert(ip) {
            break;
        }
        match ops.get(ip).unwrap() {
            Op::Acc(x) => {
                acc += x;
                ip += 1;
            }
            Op::Nop(_) => ip += 1,
            Op::Jmp(x) => ip = (x + ip as i32) as usize,
        }
    }

    let p2 = (0..ops.len())
        .find_map(|x| match ops[x] {
            Op::Acc(_) => None,
            _ => eval(&ops, x),
        })
        .unwrap();

    format!("{} {}\n", acc, p2)
}
