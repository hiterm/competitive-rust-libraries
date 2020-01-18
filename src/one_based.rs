#[derive(Debug, Copy, Clone)]
struct OneBased(usize);

impl OneBased {
    fn new(n: usize) -> OneBased {
        if n == 0 {
            panic!("arg n should be positive number");
        }
        OneBased(n)
    }

    fn unwrap(self) -> usize {
        let OneBased(n) = self;
        n
    }

    fn index(self) -> usize {
        let OneBased(n) = self;
        n - 1
    }
}

impl Add for OneBased {
    type Output = OneBased;

    fn add(self, other: OneBased) -> OneBased {
        OneBased::new(self.0 + other.0)
    }
}
