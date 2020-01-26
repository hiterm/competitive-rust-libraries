use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::io::Write;

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn main() {
    let n = 4;
    // graph shape:
    // 0 - 1 - 3
    //   \
    //     2
    let graph_mat = vec![
        vec![0, 1, 1, 0],
        vec![1, 0, 0, 1],
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0],
    ];
    debugln!("{:?}", graph_mat);

    let mut graph_list = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if graph_mat[i][j] == 1 {
                graph_list[i].push(j);
            }
        }
    }
    debugln!("{:?}", graph_list);

    println!("start bfs");
    bfs(0, &graph_list);
    println!();

    println!("start dfs");
    dfs(0, &graph_list);
    println!();

    println!("start dfs recursive");
    dfs_recursive(0, &graph_list);

    println!("start dijkstra");
    let distances = dijkstra(0, &graph_list);
    println!("{:?}", distances);

    // graph shape:
    // 0 -> 1 -> 3
    //   ↘
    //     2
    let mut graph_list2 = vec![vec![]; n];
    graph_list2[0].push(1);
    graph_list2[0].push(2);
    graph_list2[1].push(3);
    println!("Topological Sort");
    match topological_sort(&graph_list2) {
        Some(v) => println!("{:?}", v),
        None => println!("Graph has a closed path.")
    }

    // graph shape:
    // 0 -> 1
    //   ↖  ↓
    //      2
    let mut graph_list3 = vec![vec![]; n];
    graph_list3[0].push(1);
    graph_list3[1].push(2);
    graph_list3[2].push(0);
    println!("Topological Sort");
    match topological_sort(&graph_list3) {
        Some(v) => println!("{:?}", v),
        None => println!("Graph has a closed path.")
    }
}

// 幅優先探索
fn bfs(start: usize, graph_list: &[Vec<usize>]) {
    let n = graph_list.len();

    let mut visited = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while !queue.is_empty() {
        let vertex = queue.pop_front().unwrap();
        debugln!("visit {}", vertex);
        for &next in &graph_list[vertex] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }
}

// 深さ優先探索
fn dfs(start: usize, graph_list: &[Vec<usize>]) {
    let n = graph_list.len();

    let mut visited = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();

    visited[start] = true;
    stack.push(start);

    while !stack.is_empty() {
        let vertex = stack.pop().unwrap();
        debugln!("visit {}", vertex);
        for &next in &graph_list[vertex] {
            if !visited[next] {
                visited[next] = true;
                stack.push(next);
            }
        }
    }
}

// 深さ優先探索（再帰）
fn dfs_recursive(start: usize, graph_list: &[Vec<usize>]) {
    let n = graph_list.len();
    let mut visited = vec![false; n];

    dfs_aux(start, graph_list, &mut visited);
}
// 補助関数
fn dfs_aux(start: usize, graph_list: &[Vec<usize>], visited: &mut Vec<bool>) {
    visited[start] = true;

    debugln!("visit {}", start);

    for &next in &graph_list[start] {
        if !visited[next] {
            dfs_aux(next, &graph_list, visited);
        }
    }
}

// 古いRust向けの標準と同じ機能を持つReverse
#[derive(PartialEq, Eq, Debug)]
struct Reverse<T>(T);

impl<T: Ord> Ord for Reverse<T> {
    fn cmp(&self, other: &Reverse<T>) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl<T: PartialOrd> PartialOrd for Reverse<T> {
    fn partial_cmp(&self, other: &Reverse<T>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0).map(|a| a.reverse())
    }
}

// ダイクストラ法。辺の長さがすべて1の場合。
fn dijkstra(start: usize, graph_list: &[Vec<usize>]) -> Vec<usize> {
    let n = graph_list.len();
    let mut distances = vec![std::usize::MAX >> 2; n];
    distances[start] = 0;

    // BinaryHeapは最大ヒープなので、Reverseで距離最小のものを取り出せるようにする
    // ヒープの中身は Reverse((distance, distination))
    let mut queue: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    for vertex in 0..n {
        queue.push(Reverse((distances[vertex], vertex)));
    }

    while !queue.is_empty() {
        let Reverse((d, u)) = queue.pop().unwrap();
        debugln!("u: {}", u);
        debugln!("distances: {:?}", distances);
        for &v in &graph_list[u] {
            debugln!("v: {}", v);
            let alt = d + 1;
            if distances[v] > alt {
                distances[v] = alt;
                queue.push(Reverse((alt, v)))
            }
        }
    }

    distances
}

// Warshall-Floyd法
#[allow(unused)]
fn warshall_floyd(graph_mat: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = graph_mat.len();

    let mut d = vec![vec![std::usize::MAX >> 2; n]; n];
    for i in 0..n {
        for j in 0..n {
            if graph_mat[i][j] == 1 {
                d[i][j] = 1;
            }
        }
    }
    for i in 0..n {
        d[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][j] > d[i][k] + d[k][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }

    d
}

// グラフの直径
#[allow(unused)]
fn graph_diameter(graph_mat: &[Vec<usize>]) -> usize {
    let d_mat = warshall_floyd(&graph_mat);
    let mut diameter = 0;
    for v in d_mat {
        for d in v {
            if d > diameter {
                diameter = d;
            }
        }
    }

    diameter
}

// 木の直径．グラフが閉路を持たないときのみ使える。
#[allow(unused)]
fn tree_diameter(graph_list: &[Vec<usize>]) -> usize {
    let start = 0;
    let d_v = dijkstra(start, graph_list);

    let mut farthest = start;
    let mut d_max = 0;
    for (v, &d) in d_v.iter().enumerate() {
        if d > d_max {
            d_max = d;
            farthest = v;
        }
    }

    let start = farthest;
    let mut d_max = 0;
    let d_v = dijkstra(start, graph_list);
    for (_, &d) in d_v.iter().enumerate() {
        if d > d_max {
            d_max = d;
        }
    }
    d_max
}

// 2部グラフ判定
#[allow(unused)]
fn is_bipartite_graph(graph_list: &[Vec<usize>]) -> bool {
    let n = graph_list.len();

    let mut stack: Vec<(usize, bool)> = vec![];
    let mut colors = vec![None; n];

    let start = 0;
    stack.push((start, true));

    while !stack.is_empty() {
        let (vertex, color) = stack.pop().unwrap();

        colors[vertex] = Some(color);

        for &next in &graph_list[vertex] {
            match colors[next] {
                Some(next_color) => {
                    if color == next_color {
                        return false;
                    }
                }
                None => stack.push((next, !color)),
            }
        }
    }

    true
}

fn topological_sort(graph_list: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = graph_list.len();

    let mut incoming_num = vec![0; n];
    for edges in graph_list.iter() {
        for &vertex in edges {
            incoming_num[vertex] += 1;
        }
    }

    // 入次数0の頂点集合
    let mut stack: Vec<usize> = vec![];
    for (vertex, &num) in incoming_num.iter().enumerate() {
        if num == 0 {
            stack.push(vertex);
        }
    }

    let mut ret = vec![];
    while let Some(vertex) = stack.pop() {
        ret.push(vertex);
        for &next in &graph_list[vertex] {
            incoming_num[next] -= 1;
            if incoming_num[next] == 0 {
                stack.push(next);
            }
        }
    }

    if incoming_num.iter().filter(|a| **a != 0).count() == 0 {
        Some(ret)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    // TODO
}
