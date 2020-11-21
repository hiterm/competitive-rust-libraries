use std::collections::BTreeSet;

fn main() {
    let mut set = BTreeSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);
    println!("{}", set.get_max().unwrap());
    println!("{}", set.get_min().unwrap());
    println!("{:?}", &set);
    println!("{}", set.pop_max().unwrap());
    println!("{:?}", &set);
    println!("{}", set.pop_min().unwrap());
    println!("{:?}", &set);
    println!("{}", set.pop_max().unwrap());
    println!("{:?}", &set);
}

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
