pub struct Adjacent {
    adj_points: Vec<(usize, usize)>,
}

impl Adjacent {
    pub fn new(point: (usize, usize), size: (usize, usize)) -> Adjacent {
        const DIFFS: [(i64, i64); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        let mut adj_points = vec![];
        let (r, c) = point;
        let (sr, sc) = size;
        for (dr, dc) in DIFFS.iter().copied() {
            let ar = r as i64 + dr;
            let ac = c as i64 + dc;
            if 0 <= ar && ar < sr as i64 && 0 <= ac && ac < sc as i64 {
                adj_points.push((ar as usize, ac as usize));
            }
        }

        Adjacent { adj_points }
    }
}

impl IntoIterator for Adjacent {
    type Item = (usize, usize);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.adj_points.into_iter()
    }
}
