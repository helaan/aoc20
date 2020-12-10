pub(crate) fn run(data: &[u8]) -> String {
    let mut adapters = Vec::new();
    let mut wrk: u8 = 0;
    data.iter().for_each(|b| {
        if *b == '\n' as u8 {
            adapters.push(wrk);
            wrk = 0;
        } else {
            wrk *= 10;
            wrk += b - '0' as u8;
        }
    });
    assert_eq!(wrk, 0);

    adapters.sort_unstable();

    let mut d1 = 0;
    let mut d3 = 1;
    let mut cur = 0;
    adapters.iter().for_each(|a| {
        if a - cur == 1 {
            d1 += 1;
        } else if a - cur == 3 {
            d3 += 1;
        }
        cur = *a;
    });
    //println!("1j: {} 3j: {} x {}", d1, d3, d1 * d3);

    let mut a = Vec::with_capacity(cur as usize + 4);
    a.push(1 as u64);
    (1..cur + 4).for_each(|_| a.push(0));
    adapters.iter().for_each(|ad| {
        let min = if *ad >= 3 { *ad as usize - 3 } else { 0 };
        a[*ad as usize] = a[min..*ad as usize].iter().fold(0, |acc, x| acc + x);
    });

    format!("{} {}\n", d1 * d3, a[cur as usize])
}
