/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/


use std::collections::VecDeque;

struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        // 如果是无向图，需要添加这一行；如果是有向图，则不需要
        // self.adj[dest].push(src);
    }
}

// 将 BFS 函数移出 Graph 实现，使其成为一个独立的函数
fn bfs_with_return(graph: &Graph, start: usize) -> Vec<usize> {
    let mut visit_order = Vec::new();
    let mut visited = vec![false; graph.adj.len()];
    let mut queue = VecDeque::new();

    if start < graph.adj.len() {
        queue.push_back(start);
        visited[start] = true;
    }

    while let Some(node) = queue.pop_front() {
        visit_order.push(node);
        for neighbor in graph.adj[node].iter() {
            if !visited[*neighbor] {
                queue.push_back(*neighbor);
                visited[*neighbor] = true;
            }
        }
    }

    visit_order
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = bfs_with_return(&graph, 0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = bfs_with_return(&graph, 2);
        assert_eq!(visited_order, vec![2]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = bfs_with_return(&graph, 0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let graph = Graph::new(1);

        let visited_order = bfs_with_return(&graph, 0);
        assert_eq!(visited_order, vec![0]);
    }
}