use bit_set::BitSet;
use rustc_hash::FxHashMap;

#[derive(Debug)]
struct InternMap<'a> {
    map: FxHashMap<&'a [u8], usize>,
    next: usize,
}

impl<'a> InternMap<'a> {
    fn new() -> InternMap<'a> {
        InternMap {
            map: FxHashMap::default(),
            next: 0,
        }
    }

    fn get(&mut self, key: &'a [u8]) -> usize {
        if let Some(value) = self.map.get(key) {
            *value
        } else {
            self.map.insert(key, self.next);
            self.next += 1;
            self.next - 1
        }
    }
    fn len(&self) -> usize {
        self.next
    }
    fn collect(self) -> Vec<(&'a [u8], usize)> {
        self.map.into_iter().collect()
    }
    fn reverse(self) -> FxHashMap<usize, &'a [u8]> {
        let mut r = FxHashMap::default();
        for (k, v) in self.map {
            r.insert(v, k);
        }
        r
    }
}

pub(crate) fn run(data: &[u8]) -> String {
    let mut food_contents = Vec::new();
    let mut food_allergens = Vec::new();
    let mut ingredients = InternMap::new();
    let mut allergies = InternMap::new();

    let mut p = 0;
    while p < data.len() {
        //dbg!(data[p] as char);
        let mut contents = BitSet::new();
        while data[p] != b'(' {
            // dbg!(data[p] as char);
            let start = p;
            p += 1;
            while data[p] != b' ' {
                p += 1;
            }
            //dbg!(&data[start..p]);
            let ino = ingredients.get(&data[start..p]);
            contents.insert(ino);
            p += 1;
        }
        food_contents.push(contents);
        p += 10;
        let mut allergens = Vec::new();
        loop {
            let start = p;
            p += 1;
            while data[p] != b',' && data[p] != b')' {
                p += 1;
            }
            let ano = allergies.get(&data[start..p]);
            allergens.push(ano);
            if data[p] == b')' {
                p += 2;
                break;
            }
            p += 2;
        }
        food_allergens.push(allergens);
    }
    //dbg!(&ingredients);
    //dbg!(&food_contents);
    //dbg!(&food_allergens);

    let mut allergen_candidates: Vec<Option<BitSet>> = vec![None; allergies.len()];

    for (contents, allergens) in food_contents.iter().zip(food_allergens.iter()) {
        for allergen in allergens {
            if let Some(Some(e)) = allergen_candidates.get_mut(*allergen) {
                e.intersect_with(contents);
            } else {
                allergen_candidates[*allergen] = Some(contents.clone());
            }
        }
    }
    //dbg!(&allergen_candidates);

    let mut p1set = BitSet::with_capacity(ingredients.len());
    for c in &allergen_candidates {
        p1set.union_with(&c.as_ref().unwrap());
    }

    //dbg!(&p1set);

    //let p1 = ingredients.len() - p1set.len();
    let mut p1 = 0;
    for l in &food_contents {
        for c in l {
            if !p1set.contains(c) {
                p1 += 1;
            }
        }
    }

    let mut unassigned = allergen_candidates.len();
    let mut used = BitSet::with_capacity(ingredients.len());
    let mut answers = vec![9999; allergies.len()];
    while unassigned > 0 {
        for (id, candidate_list) in &mut allergen_candidates.iter_mut().enumerate() {
            if let Some(list) = candidate_list {
                list.difference_with(&used);
                if list.len() == 1 {
                    let ans = list.iter().next().unwrap();
                    //dbg!(id, ans, &answers);
                    answers[id] = ans;
                    used.insert(ans);
                    unassigned -= 1;
                    *candidate_list = None;
                }
            }
        }
    }
    //dbg!(&answers);
    let mut canon = allergies.collect();
    let ingredient_names = ingredients.reverse();
    canon.sort_by(|(x, _), (y, _)| x.cmp(y));

    let p2 = canon
        .iter()
        .map(|(_, x)| String::from_utf8_lossy(&ingredient_names.get(&answers[*x]).unwrap()))
        .collect::<Vec<_>>()
        .join(",");

    //   dbg!(p1, p2);

    format!("{} {}\n", p1, p2)
}
