use std::collections::BTreeSet;

trait BTreeSetUtil<T> {
    fn get_max(&self) -> Option<&T>;
    fn get_min(&self) -> Option<&T>;
    fn pop_max(&mut self) -> Option<T>;
    fn pop_min(&mut self) -> Option<T>;
}

impl<T> BTreeSetUtil<T> for BTreeSet<T>
where
    T: Ord + Clone,
{
    fn get_max(&self) -> Option<&T> {
        self.iter().next_back()
    }

    fn get_min(&self) -> Option<&T> {
        self.iter().next()
    }

    fn pop_max(&mut self) -> Option<T> {
        let max = self.iter().next_back().cloned();
        match max {
            None => None,
            Some(max) => self.take(&max),
        }
    }

    fn pop_min(&mut self) -> Option<T> {
        let min = self.iter().next().cloned();
        match min {
            None => None,
            Some(min) => self.take(&min),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn btreeset_test() {
        let mut set = BTreeSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.insert(4);
        set.insert(5);
        assert_eq!(5, *set.get_max().unwrap());
        assert_eq!(1, *set.get_min().unwrap());
        assert_eq!(5, set.pop_max().unwrap());
        assert_eq!(4, set.len());
        assert_eq!(1, set.pop_min().unwrap());
        assert_eq!(3, set.len());
        assert_eq!(4, set.pop_max().unwrap());
        assert_eq!(2, set.len());
    }
}
