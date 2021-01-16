use std::{collections::HashMap, hash::Hash};

#[derive(Clone, Debug)]
pub struct HashMultiSet<T: Hash + Eq> {
    map: HashMap<T, i64>,
}

impl<T> HashMultiSet<T>
where
    T: Hash + Eq,
{
    pub fn new() -> HashMultiSet<T> {
        Self {
            map: HashMap::new(),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_1() {
        let mut set = HashMultiSet::new();
        set.insert(1);
        set.insert(1);
        assert_eq!(2, set.get_count(&1));
    }

    #[test]
    fn _1_and_2() {
        let mut set = HashMultiSet::new();
        set.insert(1);
        set.insert(2);
        assert!(set.contains(&1));
        assert!(set.contains(&1));
    }

    #[test]
    fn remove() {
        let mut set = HashMultiSet::new();
        set.insert(1);
        set.insert(1);
        set.remove(&1);
        assert_eq!(1, set.get_count(&1));
        set.remove(&1);
        assert_eq!(0, set.get_count(&1));
        assert!(!set.contains(&1));
    }
}
