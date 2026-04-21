# Zip Game

Zip is a small puzzle game built with `Rust`🦀 and [Macroquad](https://github.com/not-fl3/macroquad).

The goal is to connect the numbered dots in order and cover the entire board without breaking the path.

It is designed to stay fast, minimal, and readable both on desktop and in the browser.

## Demo

https://github.com/user-attachments/assets/b59022fe-d4d6-4579-aeca-3045c479dfcf

## Notes

The project is intentionally small, and the code follows the same idea:

- `main.rs` handles screen flow
- `game.rs` contains the gameplay logic
- `generator.rs` builds playable boards

The split is simple on purpose and keeps the project easy to extend.

## How `generator.rs` Works

`generator.rs` is responsible for board generation.

It first builds a path that visits every cell exactly once, then places numbered checkpoints along that route.

- It attempts to generate a full valid route through the grid
- It distributes checkpoints along that route
- If the search fails, it falls back to a simple snake pattern

## DFS and Backtracking

The generator is built around DFS with backtracking.

The search pushes one route forward as far as possible. When it reaches a dead end, it rewinds the last move and tries a different branch. The generator keeps two core structures:

- `path` stores the current route being built
- `visited` prevents the search from reusing cells

This keeps generation deterministic at the rule level while still producing different boards from run to run.
