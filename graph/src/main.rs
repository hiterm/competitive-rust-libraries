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
fn bfs(start: usize, graph_list: &Vec<Vec<usize>>) {
    let n = graph_list.len();

    let mut visited = vec![false; n];
    let mut queue: VecDeque<usize> = VecDeque::new();

    visited[start] = true;
    queue.push_back(start);

    while !queue.is_empty() {
        let vertex = queue.pop_front().unwrap();
        debugln!("visit {}", vertex);
        for &next in &graph_list[vertex] {
            if visited[next] == false {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }
}

// 深さ優先探索
fn dfs(start: usize, graph_list: &Vec<Vec<usize>>) {
    let n = graph_list.len();

    let mut visited = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();

    visited[start] = true;
    stack.push(start);

    while !stack.is_empty() {
        let vertex = stack.pop().unwrap();
        debugln!("visit {}", vertex);
        for &next in &graph_list[vertex] {
            if visited[next] == false {
                visited[next] = true;
                stack.push(next);
            }
        }
    }
}

// 深さ優先探索（再帰）
fn dfs_recursive(start: usize, graph_list: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    visited[start] = true;

    debugln!("visit {}", start);

    for &next in &graph_list[start] {
        if visited[next] == false {
            dfs_recursive(next, &graph_list, visited);
        }
    }
}
