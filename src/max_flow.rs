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
    let (num_vertices, num_edges): (usize, usize) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let s = s.trim_right().to_owned();
        let mut ws = s.split_whitespace();
        let a = ws.next().unwrap().parse().unwrap();
        let b = ws.next().unwrap().parse().unwrap();
        (a, b)
    };

    let mut graph: Graph = {
        let mut v: Graph = vec![vec![]; num_vertices];
        for _ in 0..num_edges {
            let (from, to, cap): (usize, usize, usize) = {
                let mut s = String::new();
                std::io::stdin().read_line(&mut s).unwrap();
                let s = s.trim_right().to_owned();
                let mut ws = s.split_whitespace();
                let a = ws.next().unwrap().parse().unwrap();
                let b = ws.next().unwrap().parse().unwrap();
                let c = ws.next().unwrap().parse().unwrap();
                (a, b, c)
            };
            let l = v[to].len();
            v[from].push(Edge {to: to, cap: cap, rev: l});
            let l = v[from].len();
            v[to].push(Edge {to: from, cap: 0, rev: l - 1});
        }
        v
    };

    let ans = max_flow(0, num_vertices - 1, &mut graph);
    println!("{}", ans);
}

type Graph = Vec<Vec<Edge>>;

#[derive(Debug, Clone)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

fn max_flow(source: usize, sink: usize, graph: &mut Graph) -> usize {
    let n = graph.len();


    let mut flow = 0;
    loop {
        let mut level: Vec<Option<usize>> = vec![None; n];
        bfs(source, &graph, &mut level);
        if level[sink].is_none() {
            return flow;
        }

        let mut iter: Vec<usize> = vec![0; n];

        let mut f = dfs(source, sink, std::usize::MAX, graph, &level, &mut iter);
        while f > 0 {
            flow += f;
            f = dfs(source, sink, std::usize::MAX, graph, &level, &mut iter);
        }
    }
}

fn bfs(source: usize, graph: &Graph, level: &mut Vec<Option<usize>>) {
    use std::collections::VecDeque;

    let mut queue = VecDeque::new();
    level[source] = Some(0);
    queue.push_front(source);
    while let Some(v) = queue.pop_back() {
        for e in &graph[v] {
            let l = level[v].unwrap();
            if e.cap > 0 && level[e.to].is_none() {
                level[e.to] = Some(l + 1);
                queue.push_front(e.to);
            }
        }
    }
}

fn dfs(v: usize, t: usize, f: usize, graph: &mut Graph, level: &Vec<Option<usize>>, iter: &mut Vec<usize>) -> usize {
    if v == t {
        return f;
    }
    for i in iter[v]..graph[v].len() {
        let e = graph[v][i].clone();
        if e.cap > 0 && level[v] < level[e.to] {
            let d = dfs(e.to, t, std::cmp::min(f, e.cap), graph, level, iter);
            if d > 0 {
                graph[v][i].cap -= d;
                graph[e.to][e.rev].cap += d;
                return d;
            }
        }
    }
    0
}

// 頂点集合X, Yからなる二部グラフの最大マッチング
fn bipartite_matching(x: usize, y: usize, edges: &Vec<(usize, usize)>) -> usize {
    // 0~(x - 1)がX、x~(x + y - 1)がY、x + yが始点、x + y + 1が終点
    let mut graph: Graph = vec![vec![]; x + y + 2];
    let source = x + y;
    let sink = x + y + 1;

    // 始点からX
    for vertex in 0..x {
        add_edge(source, vertex, 1, &mut graph);
    }
    // Yから終点
    for vertex in x..(x + y) {
        add_edge(vertex, sink, 1, &mut graph);
    }
    // X, Y間
    for e in edges {
        let (x1, y1) = *e;
        add_edge(x1, y1 + x, 1, &mut graph);
    }

    max_flow(source, sink, &mut graph)
}

fn add_edge(from: usize, to: usize, cap: usize, graph: &mut Graph) {
    let l = graph[to].len();
    graph[from].push(Edge {to: to, cap: cap, rev: l});
    let l = graph[from].len();
    graph[to].push(Edge {to: from, cap: 0, rev: l - 1});
}
