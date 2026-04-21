use crate::level::{Cell, Level};

pub enum MoveOutcome {
    Invalid,
    Advanced,
    Backtracked,
    ReachedCheckpoint,
    Completed,
}

pub fn total_cells(level: &Level) -> usize {
    let size = level.size as usize;
    size * size
}

pub fn first_checkpoint(level: &Level) -> Option<Cell> {
    checkpoint_cell(level, 1)
}

pub fn checkpoint_cell(level: &Level, index: u8) -> Option<Cell> {
    level.checkpoints.iter().find(|cp| cp.index == index).map(|cp| cp.cell)
}

pub fn checkpoint_at(level: &Level, cell: Cell) -> Option<u8> {
    level.checkpoints.iter().find(|cp| cp.cell == cell).map(|cp| cp.index)
}

pub fn is_neighbor(a: Cell, b: Cell) -> bool {
    let dx = (a.0 - b.0).abs();
    let dy = (a.1 - b.1).abs();
    dx + dy == 1
}

pub fn apply_move(
    level: &Level,
    path: &mut Vec<Cell>,
    next_checkpoint_index: &mut u8,
    cell: Cell,
) -> MoveOutcome {
    match path.last().copied() {
        None => {
            if first_checkpoint(level) != Some(cell) {
                return MoveOutcome::Invalid;
            }

            path.push(cell);
            *next_checkpoint_index = 2;
            checkpoint_outcome(level, path, *next_checkpoint_index)
        }
        Some(last) if cell == last => MoveOutcome::Invalid,
        Some(last) if !is_neighbor(last, cell) => MoveOutcome::Invalid,
        Some(_) if path.contains(&cell) => {
            backtrack_to(level, path, next_checkpoint_index, cell);
            MoveOutcome::Backtracked
        }
        Some(_) => {
            if let Some(checkpoint_index) = checkpoint_at(level, cell) {
                if checkpoint_index != *next_checkpoint_index {
                    return MoveOutcome::Invalid;
                }

                path.push(cell);
                *next_checkpoint_index += 1;
                checkpoint_outcome(level, path, *next_checkpoint_index)
            } else {
                path.push(cell);
                advance_outcome(level, path, *next_checkpoint_index)
            }
        }
    }
}

pub fn is_complete(level: &Level, path: &[Cell], next_checkpoint_index: u8) -> bool {
    let all_cells_filled = path.len() == total_cells(level);
    let finished_checkpoints = next_checkpoint_index as usize > level.checkpoints.len();
    all_cells_filled && finished_checkpoints
}

fn checkpoint_outcome(level: &Level, path: &[Cell], next_checkpoint_index: u8) -> MoveOutcome {
    if is_complete(level, path, next_checkpoint_index) {
        MoveOutcome::Completed
    } else {
        MoveOutcome::ReachedCheckpoint
    }
}

fn advance_outcome(level: &Level, path: &[Cell], next_checkpoint_index: u8) -> MoveOutcome {
    if is_complete(level, path, next_checkpoint_index) {
        MoveOutcome::Completed
    } else {
        MoveOutcome::Advanced
    }
}

fn backtrack_to(level: &Level, path: &mut Vec<Cell>, next_checkpoint_index: &mut u8, cell: Cell) {
    let Some(position) = path.iter().position(|&path_cell| path_cell == cell) else {
        return;
    };

    path.truncate(position + 1);
    *next_checkpoint_index = recompute_next_checkpoint_index(level, path);
}

fn recompute_next_checkpoint_index(level: &Level, path: &[Cell]) -> u8 {
    let mut next_checkpoint_index = 1;

    for checkpoint in &level.checkpoints {
        if path.contains(&checkpoint.cell) {
            next_checkpoint_index = checkpoint.index + 1;
        } else {
            break;
        }
    }

    next_checkpoint_index
}
