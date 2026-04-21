use std::time::{SystemTime, UNIX_EPOCH};

use crate::level::{Cell, Checkpoint, Level};

pub fn generate_level(size: i16) -> Level {
    let total_cells = (size as usize) * (size as usize);
    let checkpoint_count = checkpoint_count(size, total_cells);
    let mut rng = Rng::seeded();

    for _ in 0..96 {
        if let Some(path) = generate_path(size, &mut rng) {
            return Level {
                size,
                checkpoints: checkpoints_from_path(&path, checkpoint_count),
            };
        }
    }

    Level {
        size,
        checkpoints: checkpoints_from_path(&snake_path(size), checkpoint_count),
    }
}

fn checkpoint_count(size: i16, total_cells: usize) -> usize {
    let suggested = (size as usize) + 3;
    suggested.clamp(5, total_cells)
}

fn generate_path(size: i16, rng: &mut Rng) -> Option<Vec<Cell>> {
    let total_cells = (size as usize) * (size as usize);
    let start = (rng.range_i16(0, size), rng.range_i16(0, size));
    let mut path = Vec::with_capacity(total_cells);
    let mut visited = vec![false; total_cells];

    path.push(start);
    visited[cell_index(size, start)] = true;

    if dfs(size, &mut path, &mut visited, rng) {
        Some(path)
    } else {
        None
    }
}

fn dfs(size: i16, path: &mut Vec<Cell>, visited: &mut [bool], rng: &mut Rng) -> bool {
    if path.len() == visited.len() {
        return true;
    }

    let current = *path.last().expect("path must have a current cell");
    let mut candidates = neighbors(size, current);
    shuffle(&mut candidates, rng);
    candidates.sort_by_key(|&cell| onward_degree(size, cell, visited));

    for next in candidates {
        let index = cell_index(size, next);
        if visited[index] {
            continue;
        }

        visited[index] = true;
        path.push(next);

        if dfs(size, path, visited, rng) {
            return true;
        }

        path.pop();
        visited[index] = false;
    }

    false
}

fn onward_degree(size: i16, cell: Cell, visited: &[bool]) -> usize {
    neighbors(size, cell)
        .into_iter()
        .filter(|&neighbor| !visited[cell_index(size, neighbor)])
        .count()
}

fn neighbors(size: i16, cell: Cell) -> Vec<Cell> {
    let mut result = Vec::with_capacity(4);
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (dx, dy) in directions {
        let next = (cell.0 + dx, cell.1 + dy);
        if next.0 >= 0 && next.0 < size && next.1 >= 0 && next.1 < size {
            result.push(next);
        }
    }

    result
}

fn checkpoints_from_path(path: &[Cell], checkpoint_count: usize) -> Vec<Checkpoint> {
    let last_index = path.len() - 1;
    let mut checkpoints = Vec::with_capacity(checkpoint_count);

    for position in 0..checkpoint_count {
        let path_index = if position + 1 == checkpoint_count {
            last_index
        } else {
            position * last_index / (checkpoint_count - 1)
        };

        checkpoints.push(Checkpoint {
            index: (position + 1) as u8,
            cell: path[path_index],
        });
    }

    checkpoints
}

fn snake_path(size: i16) -> Vec<Cell> {
    let mut path = Vec::with_capacity((size as usize) * (size as usize));

    for row in 0..size {
        if row % 2 == 0 {
            for col in 0..size {
                path.push((col, row));
            }
        } else {
            for col in (0..size).rev() {
                path.push((col, row));
            }
        }
    }

    path
}

fn cell_index(size: i16, cell: Cell) -> usize {
    (cell.1 as usize) * (size as usize) + cell.0 as usize
}

fn shuffle<T>(items: &mut [T], rng: &mut Rng) {
    if items.len() < 2 {
        return;
    }

    for index in (1..items.len()).rev() {
        let swap_index = rng.range_usize(0, index + 1);
        items.swap(index, swap_index);
    }
}

struct Rng {
    state: u64,
}

impl Rng {
    fn seeded() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_nanos() as u64)
            .unwrap_or(0x9E37_79B9_7F4A_7C15);

        Self {
            state: seed ^ 0xA076_1D64_78BD_642F,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 7;
        self.state ^= self.state >> 9;
        self.state ^= self.state << 8;
        self.state
    }

    fn range_usize(&mut self, start: usize, end: usize) -> usize {
        start + (self.next_u64() as usize % (end - start))
    }

    fn range_i16(&mut self, start: i16, end: i16) -> i16 {
        start + (self.next_u64() % (end - start) as u64) as i16
    }
}
