struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut parent = vec![];
        for i in 0..n {
            parent.push(i);
        }
        UnionFind {
            parent: parent,
            rank: vec![0; n],
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let p_i = self.parent[i];
            self.parent[i] = self.find(p_i);
            self.parent[i]
        }
    }

    fn unite(&mut self, i: usize, j: usize) {
        let i = self.find(i);
        let j = self.find(j);
        if i == j {
            return;
        }

        if self.rank[i] < self.rank[j] {
            self.parent[i] = j;
        } else {
            self.parent[j] = i;
            if self.rank[i] == self.rank[j] {
                self.rank[i] += 1;
            }
        }
    }

    fn same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }
}
