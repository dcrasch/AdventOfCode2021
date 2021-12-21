use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::BufRead;

fn get_edge(
    grid: &Vec<Vec<u8>>,
    row: usize,
    col: usize,
    height: usize,
    width: usize,
    grid_height: usize,
    grid_width: usize,
) -> Edge {
    let row_section = row % grid_height;
    let col_section = col % grid_width;
    let row_p = row / grid_height;
    let col_p = col / grid_width;

    let m = col_p + row_p;
    let mut x = grid[row_section][col_section] + m as u8;
    if x > 9 {
        x = (x - 1) % 9 + 1;
    }
    Edge {
        node: row * width + col,
        cost: x as usize,
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn main() {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut grid: Vec<Vec<u8>> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        let row: Vec<u8> = line.chars().map(|x| x as u8 - b'0').collect();
        grid.push(row);
    }

    let grid_width = grid[0].len();
    let grid_height = grid.len();
    let width = grid_width * 5;
    let height = grid_height * 5;

    let mut graph: Vec<Vec<Edge>> = Vec::new();
    for row in 0..height {
        for col in 0..width {
            let mut edges: Vec<Edge> = Vec::new();
            if row > 0 {
                edges.push(get_edge(
                    &grid,
                    row - 1,
                    col,
                    height,
                    width,
                    grid_height,
                    grid_width,
                ));
            }
            if row < height - 1 {
                edges.push(get_edge(
                    &grid,
                    row + 1,
                    col,
                    height,
                    width,
                    grid_height,
                    grid_width,
                ));
            }
            if col > 0 {
                edges.push(get_edge(
                    &grid,
                    row,
                    col - 1,
                    height,
                    width,
                    grid_height,
                    grid_width,
                ));
            }
            if col < width - 1 {
                edges.push(get_edge(
                    &grid,
                    row,
                    col + 1,
                    height,
                    width,
                    grid_height,
                    grid_width,
                ));
            }
            graph.push(edges);
        }
    }
    if let Some(risk) = shortest_path(&graph, 0, height * width - 1) {
        println!("{}", risk);
    }
}
