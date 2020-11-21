use std::collections::BTreeMap;

fn main() {
    let mut set = BTreeMultiSet::new();
    set.insert(1i64);
    set.insert(1i64);
    dbg!(&set);
}

#[derive(Clone, Debug)]
struct BTreeMultiSet<T> {
    map: BTreeMap<T, i64>,
}

impl<T> BTreeMultiSet<T>
where
    T: Ord,
{
    fn new() -> BTreeMultiSet<T> {
        Self {
            map: BTreeMap::new(),
        }
    }

    #[allow(unused)]
    fn insert(&mut self, elem: T) {
        let entry = self.map.entry(elem).or_insert(0);
        *entry += 1;
    }

    #[allow(unused)]
    fn remove(&mut self, elem: &T) {
        let entry = self.map.get_mut(&elem).unwrap();
        *entry -= 1;
        if *entry == 0 {
            self.map.remove(&elem);
        }
    }

    #[allow(unused)]
    fn contains(&self, elem: &T) -> bool {
        self.map.contains_key(elem)
    }

    #[allow(unused)]
    fn min(&self) -> Option<&T> {
        self.map.iter().next().map(|(elem, _)| elem)
    }

    #[allow(unused)]
    fn max(&self) -> Option<&T> {
        self.map.iter().rev().next().map(|(elem, _)| elem)
    }
}
