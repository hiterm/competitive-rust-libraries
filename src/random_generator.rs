//! ランダムテストケースを生成してデバッグを行うためのモジュール

use std::{collections::HashSet, iter::FromIterator};

use rand::{prelude::IteratorRandom, Rng};

/// 0からnまでの頂点を持つ木。
/// 0が根。p[i - 1] (i >= 1) には頂点iの親が入る。
pub fn random_tree<R: Rng>(n: usize, rng: &mut R) -> Vec<usize> {
    let mut done = HashSet::new();
    done.insert(0);
    let mut rest: HashSet<usize> = HashSet::from_iter(1..=n);
    let mut p = vec![0; n];
    while let Some(next) = rest.iter().copied().choose(rng) {
        let parent = done.iter().copied().choose(rng).unwrap();
        p[next - 1] = parent;

        rest.remove(&next);
        done.insert(next);
    }

    p
}
