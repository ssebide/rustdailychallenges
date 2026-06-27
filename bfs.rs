use std::collections::VecDeque;

fn main() {
    let graph: Vec<Vec<usize>> = vec![
        vec![1, 2], // Node 0 -> 1,2
        vec![0, 3, 4],
        vec![0, 5],
        vec![1, 6],
        vec![1],
        vec![2],
        vec![3],
    ];

    let start = 0;
    let end = 6;

    match bfs(start, end, &graph) {
        Some(path) => println!("Path from {} to {}: {:?} ", start, end, path),
        None => println!("No path found from {} to {}", start, end),
    }
}

fn bfs(s: usize, e: usize, graph: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let prev = solve(s, graph);
    reconstruct_path(s, e, &prev)
}

//perfoems bfs and returns the previous node array
fn solve(s: usize, graph: &Vec<Vec<usize>>) -> Vec<Option<usize>> {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut prev: Vec<Option<usize>> = vec![None; n];
    let mut q: VecDeque<usize> = VecDeque::new();

    //initialize
    q.push_back(s);
    visited[s] = true;

    while let Some(node) = q.pop_front() {
        for &next in &graph[node] {
            if !visited[next] {
                q.push_back(next);
                visited[next] = true;
                prev[next] = Some(node);
            }
        }
    }
    prev
}

//reconstructs path from s to e using the prev array
fn reconstruct_path(s: usize, e: usize, prev: &Vec<Option<usize>>) -> Option<Vec<usize>> {
    let mut path = Vec::new();
    let mut current = e;

    while let Some(node) = Some(current) {
        path.push(current);
        if current == s {
            break;
        }
        match prev[current] {
            Some(parent) => current = parent,
            None => return None,
        }
    }

    //if we didnt reach strt, no path exists
    if path.last() != Some(&s) {
        return None;
    }

    path.reverse();
    Some(path)
}
