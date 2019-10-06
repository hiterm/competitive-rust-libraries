use std::io::Write;
use std::collections::VecDeque;

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
    let graph_mat = vec![
        vec![0, 1, 1, 0],
        vec![1, 0, 0, 1],
        vec![1, 0, 0, 0],
        vec![0, 1, 0, 0]
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
    println!("");

    println!("start dfs");
    dfs(0, &graph_list);
    println!("");

    println!("start dfs recursive");
    let mut visited = vec![false; n];
    dfs_recursive(0, &graph_list, &mut visited);
}

// 幅優先探索
fn bfs(vertex: usize, graph_list: &Vec<Vec<usize>>) {
    let n = graph_list.len();

    let mut visited = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();

    visited[vertex] = true;
    queue.push_back(vertex);

    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        debugln!("visit {}", v);
        for &v2 in &graph_list[v] {
            if visited[v2] == false {
                visited[v2] = true;
                queue.push_back(v2);
            }
        }
    }
}

// 深さ優先探索
fn dfs(vertex: usize, graph_list: &Vec<Vec<usize>>) {
    let n = graph_list.len();

    let mut visited = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();

    visited[vertex] = true;
    stack.push(vertex);

    while !stack.is_empty() {
        let v = stack.pop().unwrap();
        debugln!("visit {}", v);
        for &v2 in &graph_list[v] {
            if visited[v2] == false {
                visited[v2] = true;
                stack.push(v2);
            }
        }
    }
}

// 深さ優先探索（再帰）
fn dfs_recursive(vertex: usize, graph_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[vertex] = true;

    debugln!("visit {}", vertex);

    for &v2 in &graph_list[vertex] {
        if visited[v2] == false {
            dfs_recursive(v2, &graph_list, visited);
        }
    }
}
