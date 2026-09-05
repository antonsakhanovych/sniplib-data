use std::collections::VecDeque;

fn bfs(adj: &[Vec<usize>], src: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; adj.len()];
    dist[src] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(src);
    while let Some(node) = queue.pop_front() {
        for &next in &adj[node] {
            if dist[next] == usize::MAX {
                dist[next] = dist[node] + 1;
                queue.push_back(next);
            }
        }
    }
    dist
}
