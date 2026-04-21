use macroquad::prelude::*;
use macroquad::ui::{root_ui, widgets, Skin};

use crate::generator::generate_level;

const ORANGE_BG: Color = color_u8!(245, 99, 28, 255);
const ORANGE_PATH: Color = color_u8!(231, 83, 36, 255);
const ORANGE_SOFT: Color = color_u8!(255, 229, 214, 255);
const CREAM: Color = color_u8!(255, 247, 242, 255);
const GRID: Color = color_u8!(226, 216, 210, 255);
const CHECKPOINT: Color = color_u8!(36, 32, 32, 255);
const BUTTON_HOVER: Color = color_u8!(255, 241, 232, 255);
const BUTTON_CLICK: Color = color_u8!(255, 221, 203, 255);

const SIDE_PADDING: f32 = 20.0;
const TOP_BAR_HEIGHT: f32 = 54.0;
const BOTTOM_PANEL_HEIGHT: f32 = 172.0;
const HOME_BUTTON_HEIGHT: f32 = 62.0;
const BUTTON_FONT_SIZE: f32 = 28.0;
pub const GRID_SIZE: i16 = 5;
const GRID_TOTAL_CELLS: usize = (GRID_SIZE as usize) * (GRID_SIZE as usize);

pub type Cell = (i16, i16);

#[derive(Clone, Copy, Debug)]
pub struct Checkpoint {
    pub index: u8,
    pub cell: Cell,
}

#[derive(Clone, Debug)]
pub struct Level {
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(Clone, Copy)]
struct BoardLayout {
    board_x: f32,
    board_y: f32,
    board_size: f32,
    cell_size: f32,
}

impl BoardLayout {
    fn new() -> Self {
        let available_width = screen_width() - SIDE_PADDING * 2.0;
        let available_height = screen_height() - TOP_BAR_HEIGHT - BOTTOM_PANEL_HEIGHT - 42.0;
        let card_size = available_width.min(available_height);
        let card_x = (screen_width() - card_size) / 2.0;
        let card_y = TOP_BAR_HEIGHT + 20.0;
        let inset = 14.0;
        let board_size = card_size - inset * 2.0;

        Self {
            board_x: card_x + inset,
            board_y: card_y + inset,
            board_size,
            cell_size: board_size / GRID_SIZE as f32,
        }
    }
}

pub struct GameState {
    level: Level,
    path: Vec<Cell>,
    next_checkpoint_index: u8,
    start_time: f64,
}

impl GameState {
    pub fn new(start_time: f64) -> Self {
        let level = generate_level(GRID_TOTAL_CELLS);

        Self {
            level,
            path: Vec::new(),
            next_checkpoint_index: 1,
            start_time,
        }
    }

    pub fn update(&mut self) -> bool {
        let layout = BoardLayout::new();
        let hovered_cell = Self::cell_from_mouse(layout);

        if is_mouse_button_down(MouseButton::Left) && let Some(cell) = hovered_cell {
            return self.apply_move(cell);
        }

        false
    }

    pub fn draw(&self) {
        let layout = BoardLayout::new();

        clear_background(CREAM);
        self.draw_board(layout);
        self.draw_path(layout);
        self.draw_checkpoints(layout);
        self.draw_instruction(layout);
    }

    pub fn elapsed_seconds(&self) -> i32 {
        (get_time() - self.start_time) as i32
    }

    fn apply_move(&mut self, cell: Cell) -> bool {
        match self.path.last().copied() {
            None => {
                if self.level.checkpoints.first().map(|checkpoint| checkpoint.cell) != Some(cell) {
                    return false;
                }

                self.path.push(cell);
                self.next_checkpoint_index = 2;
                self.is_complete()
            }
            Some(last) if cell == last => false,
            Some(last) if !is_neighbor(last, cell) => false,
            Some(_) if self.path.contains(&cell) => {
                self.backtrack_to(cell);
                false
            }
            Some(_) => {
                if let Some(checkpoint_index) = self.checkpoint_at(cell) {
                    if checkpoint_index != self.next_checkpoint_index {
                        return false;
                    }

                    self.path.push(cell);
                    self.next_checkpoint_index += 1;
                    self.is_complete()
                } else {
                    self.path.push(cell);
                    self.is_complete()
                }
            }
        }
    }

    fn checkpoint_at(&self, cell: Cell) -> Option<u8> {
        self.level
            .checkpoints
            .iter()
            .find(|checkpoint| checkpoint.cell == cell)
            .map(|checkpoint| checkpoint.index)
    }

    fn backtrack_to(&mut self, cell: Cell) {
        let Some(position) = self.path.iter().position(|&path_cell| path_cell == cell) else {
            return;
        };

        self.path.truncate(position + 1);
        self.next_checkpoint_index = 1;

        for checkpoint in &self.level.checkpoints {
            if self.path.contains(&checkpoint.cell) {
                self.next_checkpoint_index = checkpoint.index + 1;
            } else {
                break;
            }
        }
    }

    fn is_complete(&self) -> bool {
        self.path.len() == GRID_TOTAL_CELLS
            && self.next_checkpoint_index as usize > self.level.checkpoints.len()
    }

    fn start_checkpoint_cell(&self) -> Cell {
        self.level
            .checkpoints
            .first()
            .expect("Level must have at least one checkpoint")
            .cell
    }

    fn draw_board(&self, layout: BoardLayout) {
        let cc = self.start_checkpoint_cell();

        draw_rectangle(
                layout.board_x + cc.0 as f32 * layout.cell_size,
                layout.board_y + cc.1 as f32 * layout.cell_size,
                layout.cell_size,
                layout.cell_size,
                ORANGE_SOFT,
            );

        for &(col, row) in &self.path {
            draw_rectangle(
                layout.board_x + col as f32 * layout.cell_size,
                layout.board_y + row as f32 * layout.cell_size,
                layout.cell_size,
                layout.cell_size,
                ORANGE_SOFT,
            );
        }

        for index in 0..= GRID_SIZE {
            let shift = layout.cell_size * index as f32;
            let thickness = if index == 0 || index == GRID_SIZE {
                2.5
            } else {
                1.5
            };

            draw_line(
                layout.board_x,
                layout.board_y + shift,
                layout.board_x + layout.board_size,
                layout.board_y + shift,
                thickness,
                GRID,
            );

            draw_line(
                layout.board_x + shift,
                layout.board_y,
                layout.board_x + shift,
                layout.board_y + layout.board_size,
                thickness,
                GRID,
            );
        }
    }

    fn draw_path(&self, layout: BoardLayout) {
        let path_thickness = (layout.cell_size * 0.52).max(10.0);
        let path_node_radius = (layout.cell_size * 0.26).max(10.0);

        for segment in self.path.windows(2) {
            let start = Self::cell_center(segment[0], layout);
            let end = Self::cell_center(segment[1], layout);
            draw_line(start.0, start.1, end.0, end.1, path_thickness, ORANGE_PATH);
        }

        for &cell in &self.path {
            let center = Self::cell_center(cell, layout);
            draw_circle(center.0, center.1, path_node_radius, ORANGE_PATH);
        }
    }

    fn draw_checkpoints(&self, layout: BoardLayout) {
        let checkpoint_radius = (layout.cell_size * 0.23).max(12.0);
        let font_size = (layout.cell_size * 0.30).max(18.0);

        for checkpoint in &self.level.checkpoints {
            let center = Self::cell_center(checkpoint.cell, layout);
            let is_reached = self.path.contains(&checkpoint.cell);

            draw_circle(
                center.0,
                center.1,
                checkpoint_radius,
                if is_reached { ORANGE_PATH } else { CHECKPOINT },
            );

            let label = checkpoint.index.to_string();
            let text_size = measure_text(&label, None, font_size as u16, 1.0);
            draw_text(
                &label,
                center.0 - text_size.width / 2.0,
                center.1 + text_size.height / 3.0,
                font_size,
                WHITE,
            );
        }
    }

    fn draw_instruction(&self, layout: BoardLayout) {
        let text = "Connect the dots and fill all cells";
        let font_size = (screen_width().min(screen_height()) * 0.048).clamp(18.0, 28.0);
        let text_size = measure_text(text, None, font_size as u16, 1.0);
        let x = screen_width() / 2.0 - text_size.width / 2.0;
        let y = (layout.board_y + layout.board_size + 36.0).min(screen_height() - 28.0);

        draw_text(text, x, y, font_size, CHECKPOINT);
    }

    fn cell_center(cell: Cell, layout: BoardLayout) -> (f32, f32) {
        (
            layout.board_x + cell.0 as f32 * layout.cell_size + layout.cell_size / 2.0,
            layout.board_y + cell.1 as f32 * layout.cell_size + layout.cell_size / 2.0,
        )
    }

    fn cell_from_mouse(layout: BoardLayout) -> Option<Cell> {
        let (mouse_x, mouse_y) = mouse_position();

        if mouse_x < layout.board_x
            || mouse_x >= layout.board_x + layout.board_size
            || mouse_y < layout.board_y
            || mouse_y >= layout.board_y + layout.board_size
        {
            return None;
        }

        let col = ((mouse_x - layout.board_x) / layout.cell_size) as i16;
        let row = ((mouse_y - layout.board_y) / layout.cell_size) as i16;

        if col < 0 || col >= GRID_SIZE || row < 0 || row >= GRID_SIZE {
            return None;
        }

        Some((col, row))
    }
}

pub fn draw_start_screen() -> bool {
    clear_background(ORANGE_BG);

    let logo_y = screen_height() * 0.22;
    let metrics = splash_screen_metrics();

    draw_centered_text("Zip", logo_y + screen_height() * 0.16, metrics.title_font_size, WHITE);

    draw_button("Go", screen_height() * 0.72, metrics)
}

pub fn draw_game_over_screen(elapsed_seconds: i32) -> bool {
    clear_background(ORANGE_BG);

    let metrics = splash_screen_metrics();
    let title_y = screen_height() * 0.28;
    let time_y = screen_height() * 0.47;
    let button_y = screen_height() * 0.7;

    draw_centered_text("Good Job!", title_y, metrics.subtitle_font_size, WHITE);
    draw_centered_text(
        &format!("0:{:02}", elapsed_seconds),
        time_y,
        metrics.timer_font_size,
        WHITE,
    );

    draw_button("New Game", button_y, metrics)
}

fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let size = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, screen_width() / 2.0 - size.width / 2.0, y, font_size, color);
}

#[derive(Clone, Copy)]
struct SplashScreenMetrics {
    title_font_size: f32,
    subtitle_font_size: f32,
    timer_font_size: f32,
    button_height: f32,
    button_font_size: f32,
    button_max_width: f32,
}

fn splash_screen_metrics() -> SplashScreenMetrics {
    let short_side = screen_width().min(screen_height());
    let scale = (short_side / 390.0).clamp(0.9, 1.5);

    SplashScreenMetrics {
        title_font_size: 54.0 * scale,
        subtitle_font_size: 44.0 * scale,
        timer_font_size: 80.0 * scale,
        button_height: HOME_BUTTON_HEIGHT * scale,
        button_font_size: BUTTON_FONT_SIZE * scale,
        button_max_width: 320.0 * scale,
    }
}

fn draw_button(label: &str, y: f32, metrics: SplashScreenMetrics) -> bool {
    let size = vec2(
        (screen_width() - SIDE_PADDING * 2.0).min(metrics.button_max_width),
        metrics.button_height,
    );
    let position = vec2((screen_width() - size.x) / 2.0, y);
    let skin = primary_button_skin(metrics.button_font_size);

    {
        let ui = &mut *root_ui();
        ui.push_skin(&skin);
    }

    let clicked = {
        let ui = &mut *root_ui();
        widgets::Button::new(label).position(position).size(size).ui(ui)
    };

    {
        let ui = &mut *root_ui();
        ui.pop_skin();
    }

    clicked
}

fn primary_button_skin(button_font_size: f32) -> Skin {
    let ui = &mut *root_ui();
    let button_style = ui
        .style_builder()
        .font_size(button_font_size as u16)
        .text_color(ORANGE_BG)
        .text_color_hovered(ORANGE_BG)
        .text_color_clicked(ORANGE_BG)
        .color(WHITE)
        .color_hovered(BUTTON_HOVER)
        .color_clicked(BUTTON_CLICK)
        .color_inactive(WHITE)
        .build();

    Skin {
        button_style,
        ..ui.default_skin().clone()
    }
}

fn is_neighbor(a: Cell, b: Cell) -> bool {
    let dx = (a.0 - b.0).abs();
    let dy = (a.1 - b.1).abs();
    dx + dy == 1
}
