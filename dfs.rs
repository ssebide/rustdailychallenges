fn main() {
    let graph: Vec<Vec<usize>> = vec![
        vec![1, 2], // Node 0 -> 1,2
        vec![3],
        vec![4],
        vec![],
        vec![],
    ];

    let n = graph.len();
    let mut visited = vec![false; n];

    //start dfs from node 0
    let start_node = 0;
    dfs(start_node, &graph, &mut visited);

    //print visited nodes
    println!("visited nodes:");
    for (i, &v) in visited.iter().enumerate() {
        if v {
            print!("{}", i);
        }
    }
    println!();
}

fn dfs(at: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
    if visited[at] {
        return;
    }

    visited[at] = true;

    let neighbours = &graph[at];
    for &next in neighbours {
        dfs(next, graph, visited);
    }
}
