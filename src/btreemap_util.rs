use std::{collections::BTreeMap, ops::RangeBounds};

trait BTreeMapUtil<K, V> {
    fn max_entry(&self) -> Option<(&K, &V)>;
    fn min_entry(&self) -> Option<(&K, &V)>;
    fn pop_max_entry(&mut self) -> Option<(K, V)>;
    fn pop_min_entry(&mut self) -> Option<(K, V)>;
    fn range_max_entry<R: RangeBounds<K>>(&self, range: R) -> Option<(&K, &V)>;
    fn range_min_entry<R: RangeBounds<K>>(&self, range: R) -> Option<(&K, &V)>;
    fn pop_range_max_entry<R: RangeBounds<K>>(&mut self, range: R) -> Option<(K, V)>;
    fn pop_range_min_entry<R: RangeBounds<K>>(&mut self, range: R) -> Option<(K, V)>;
}

impl<K, V> BTreeMapUtil<K, V> for BTreeMap<K, V>
where
    K: Ord + Clone,
{
    fn max_entry(&self) -> Option<(&K, &V)> {
        self.iter().next_back()
    }

    fn min_entry(&self) -> Option<(&K, &V)> {
        self.iter().next()
    }

    fn pop_max_entry(&mut self) -> Option<(K, V)> {
        self.keys().next_back().cloned().map(|key| {
            let value = self.remove(&key).unwrap(); // 必ずSomeになるはず
            (key, value)
        })
    }

    fn pop_min_entry(&mut self) -> Option<(K, V)> {
        self.keys().next().cloned().map(|key| {
            let value = self.remove(&key).unwrap();
            (key, value)
        })
    }

    fn range_max_entry<R: RangeBounds<K>>(&self, range: R) -> Option<(&K, &V)> {
        self.range(range).next_back()
    }

    fn range_min_entry<R: RangeBounds<K>>(&self, range: R) -> Option<(&K, &V)> {
        self.range(range).next()
    }

    fn pop_range_max_entry<R: RangeBounds<K>>(&mut self, range: R) -> Option<(K, V)> {
        self.range_max_entry(range)
            .map(|(key, _value)| key.clone())
            .map(|key| {
                let value = self.remove(&key).unwrap();
                (key, value)
            })
    }

    fn pop_range_min_entry<R: RangeBounds<K>>(&mut self, range: R) -> Option<(K, V)> {
        self.range_min_entry(range)
            .map(|(key, _value)| key.clone())
            .map(|key| {
                let value = self.remove(&key).unwrap();
                (key, value)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btreeset_test() {
        let mut map = BTreeMap::new();
        map.insert(1, 1);
        map.insert(2, 2);
        map.insert(3, 3);
        map.insert(4, 4);
        map.insert(5, 5);
        assert_eq!((&5, &5), map.max_entry().unwrap());
        assert_eq!((&1, &1), map.min_entry().unwrap());
        assert_eq!((5, 5), map.pop_max_entry().unwrap());
        assert_eq!(4, map.len());
        assert_eq!((1, 1), map.pop_min_entry().unwrap());
        assert_eq!(3, map.len());
        assert_eq!((4, 4), map.pop_max_entry().unwrap());
        assert_eq!(2, map.len());
    }
}
