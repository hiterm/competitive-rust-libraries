use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct BTreeMultiSet<T> {
    map: BTreeMap<T, i64>,
}

impl<T> BTreeMultiSet<T>
where
    T: Ord,
{
    pub fn new() -> BTreeMultiSet<T> {
        Self {
            map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, elem: T) {
        let entry = self.map.entry(elem).or_insert(0);
        *entry += 1;
    }

    pub fn remove(&mut self, elem: &T) {
        let entry = self.map.get_mut(&elem).unwrap();
        *entry -= 1;
        if *entry == 0 {
            self.map.remove(&elem);
        }
    }

    pub fn contains(&self, elem: &T) -> bool {
        self.map.contains_key(elem)
    }

    pub fn get_count(&self, elem: &T) -> i64 {
        self.map.get(elem).copied().unwrap_or(0)
    }

    pub fn min(&self) -> Option<&T> {
        self.map.iter().next().map(|(elem, _)| elem)
    }

    pub fn max(&self) -> Option<&T> {
        self.map.iter().rev().next().map(|(elem, _)| elem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_1() {
        let mut set = BTreeMultiSet::new();
        set.insert(1);
        set.insert(1);
        assert_eq!(2, set.get_count(&1));
    }

    #[test]
    fn _1_and_2() {
        let mut set = BTreeMultiSet::new();
        set.insert(1);
        set.insert(2);
        assert!(set.contains(&1));
        assert!(set.contains(&1));
    }

    #[test]
    fn remove() {
        let mut set = BTreeMultiSet::new();
        set.insert(1);
        set.insert(1);
        set.remove(&1);
        assert_eq!(1, set.get_count(&1));
        set.remove(&1);
        assert_eq!(0, set.get_count(&1));
        assert!(!set.contains(&1));
    }
}
