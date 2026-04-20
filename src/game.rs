use macroquad::prelude::*;

pub type Point = (i16, i16);

const SQUARES: i16 = 5;
const BOARD_CELLS: usize = (SQUARES as usize) * (SQUARES as usize);
const TOP_UI_SPACE: f32 = 72.0;
const BOTTOM_UI_SPACE: f32 = 84.0;
const SIDE_PADDING: f32 = 24.0;
const TIMER_FONT_SIZE: f32 = 32.0;
const BODY_FONT_SIZE: f32 = 30.0;
const TITLE_FONT_SIZE: f32 = 48.0;
const BUTTON_WIDTH: f32 = 280.0;
const BUTTON_HEIGHT: f32 = 64.0;
const BUTTON_FONT_SIZE: f32 = 28.0;

enum GameColor {
    Primary,
    Secondary,
    Base,
    Text,
    ButtonText,
}

impl GameColor {
    fn value(self) -> Color {
        match self {
            Self::Primary => Color::from_hex(0xD24A37),
            Self::Secondary => Color::from_hex(0xEDC5C0),
            Self::Base => WHITE,
            Self::Text => DARKGRAY,
            Self::ButtonText => WHITE,
        }
    }
}

#[derive(Clone, Copy)]
struct BoardLayout {
    offset_x: f32,
    offset_y: f32,
    game_size: f32,
    cell_size: f32,
}

impl BoardLayout {
    fn new() -> Self {
        let available_width = screen_width() - SIDE_PADDING * 2.0;
        let available_height = screen_height() - TOP_UI_SPACE - BOTTOM_UI_SPACE;
        let game_size = available_width.min(available_height);

        Self {
            offset_x: (screen_width() - game_size) / 2.0,
            offset_y: TOP_UI_SPACE,
            game_size,
            cell_size: game_size / SQUARES as f32,
        }
    }
}

pub struct GameState {
    path_cells: Vec<Point>,
    last_drag_cell: Option<Point>,
    start_time: f64,
}

impl GameState {
    pub fn new(start_time: f64) -> Self {
        Self {
            path_cells: Vec::with_capacity(BOARD_CELLS),
            last_drag_cell: None,
            start_time,
        }
    }

    pub fn update(&mut self) -> bool {
        let layout = BoardLayout::new();

        if is_mouse_button_down(MouseButton::Left)
            && let Some(cell) = Self::cell_from_mouse(layout)
        {
            self.extend_path(cell);
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.last_drag_cell = None;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.reset();
        }

        self.is_complete()
    }

    pub fn draw(&self) {
        let layout = BoardLayout::new();

        clear_background(GameColor::Base.value());
        self.draw_board(layout);
        self.draw_path(layout);
        self.draw_timer();

        // button click "undo" -> self.reset();
    }

    pub fn elapsed_seconds(&self) -> i32 {
        (get_time() - self.start_time) as i32
    }

    fn reset(&mut self) {
        self.path_cells.clear();
        self.last_drag_cell = None;
    }

    fn extend_path(&mut self, cell: Point) {
        match self.last_drag_cell {
            None => {
                self.path_cells.push(cell);
                self.last_drag_cell = Some(cell);
            }
            Some(prev_cell) if cell == prev_cell => {}
            Some(prev_cell)
                if Self::is_neighbor(prev_cell, cell) && !self.path_cells.contains(&cell) =>
            {
                self.path_cells.push(cell);
                self.last_drag_cell = Some(cell);
            }
            Some(_) => {}
        }
    }

    fn is_complete(&self) -> bool {
        self.path_cells.len() == BOARD_CELLS
    }

    fn draw_board(&self, layout: BoardLayout) {
        draw_rectangle(
            layout.offset_x,
            layout.offset_y,
            layout.game_size,
            layout.game_size,
            GameColor::Base.value(),
        );

        for &(col, row) in &self.path_cells {
            draw_rectangle(
                layout.offset_x + col as f32 * layout.cell_size,
                layout.offset_y + row as f32 * layout.cell_size,
                layout.cell_size,
                layout.cell_size,
                GameColor::Secondary.value(),
            );
        }

        for index in 1..SQUARES {
            let shift = layout.cell_size * index as f32;

            draw_line(
                layout.offset_x,
                layout.offset_y + shift,
                layout.offset_x + layout.game_size,
                layout.offset_y + shift,
                2.0,
                GameColor::Text.value(),
            );

            draw_line(
                layout.offset_x + shift,
                layout.offset_y,
                layout.offset_x + shift,
                layout.offset_y + layout.game_size,
                2.0,
                GameColor::Text.value(),
            );
        }
    }

    fn draw_path(&self, layout: BoardLayout) {
        let path_thickness = (layout.cell_size * 0.5).max(8.0);
        let path_node_radius = (layout.cell_size * 0.25).max(8.0);

        for segment in self.path_cells.windows(2) {
            let start = Self::cell_center(segment[0], layout);
            let end = Self::cell_center(segment[1], layout);

            draw_line(
                start.0,
                start.1,
                end.0,
                end.1,
                path_thickness,
                GameColor::Primary.value(),
            );
        }

        for &cell in &self.path_cells {
            let center = Self::cell_center(cell, layout);
            draw_circle(
                center.0,
                center.1,
                path_node_radius,
                GameColor::Primary.value(),
            );
        }
    }

    fn draw_timer(&self) {
        let timer_text = format!("Time: {}s", self.elapsed_seconds());
        draw_centered_text(
            &timer_text,
            TOP_UI_SPACE / 2.0 + TIMER_FONT_SIZE / 3.0,
            TIMER_FONT_SIZE,
            GameColor::Text.value(),
        );
    }

    fn is_neighbor(a: Point, b: Point) -> bool {
        let dx = (a.0 - b.0).abs();
        let dy = (a.1 - b.1).abs();

        dx + dy == 1
    }

    fn cell_center(cell: Point, layout: BoardLayout) -> (f32, f32) {
        (
            layout.offset_x + cell.0 as f32 * layout.cell_size + layout.cell_size / 2.0,
            layout.offset_y + cell.1 as f32 * layout.cell_size + layout.cell_size / 2.0,
        )
    }

    fn cell_from_mouse(layout: BoardLayout) -> Option<Point> {
        let (mouse_x, mouse_y) = mouse_position();

        if mouse_x < layout.offset_x
            || mouse_x >= layout.offset_x + layout.game_size
            || mouse_y < layout.offset_y
            || mouse_y >= layout.offset_y + layout.game_size
        {
            return None;
        }

        let col = ((mouse_x - layout.offset_x) / layout.cell_size) as i16;
        let row = ((mouse_y - layout.offset_y) / layout.cell_size) as i16;

        Some((col, row))
    }
}

pub fn draw_start_screen() {
    clear_background(GameColor::Base.value());
    draw_centered_text(
        "Zip",
        screen_height() * 0.35,
        TITLE_FONT_SIZE,
        GameColor::Primary.value(),
    );
    draw_centered_text(
        "Connect the dots in order and fill every cell",
        screen_height() * 0.5,
        BODY_FONT_SIZE,
        GameColor::Text.value(),
    );
    draw_button("Start", start_button_rect());
}

pub fn draw_game_over_screen(elapsed_seconds: i32) {
    clear_background(GameColor::Base.value());
    draw_centered_text(
        "Congrats!",
        screen_height() * 0.35,
        TITLE_FONT_SIZE,
        GameColor::Primary.value(),
    );
    draw_centered_text(
        &format!("Time: {}s", elapsed_seconds),
        screen_height() * 0.5,
        BODY_FONT_SIZE,
        GameColor::Text.value(),
    );
    draw_button("Play again", play_again_button_rect());
}

pub fn start_button_clicked() -> bool {
    button_clicked(start_button_rect())
}

pub fn play_again_button_clicked() -> bool {
    button_clicked(play_again_button_rect())
}

fn draw_centered_text(text: &str, y: f32, font_size: f32, color: Color) {
    let size = measure_text(text, None, font_size as u16, 1.0);
    draw_text(
        text,
        screen_width() / 2.0 - size.width / 2.0,
        y,
        font_size,
        color,
    );
}

fn start_button_rect() -> Rect {
    centered_button_rect(screen_height() * 0.64)
}

fn play_again_button_rect() -> Rect {
    centered_button_rect(screen_height() * 0.64)
}

fn centered_button_rect(center_y: f32) -> Rect {
    Rect::new(
        screen_width() / 2.0 - BUTTON_WIDTH / 2.0,
        center_y - BUTTON_HEIGHT / 2.0,
        BUTTON_WIDTH,
        BUTTON_HEIGHT,
    )
}

fn draw_button(label: &str, rect: Rect) {
    let hovered = rect.contains(vec2(mouse_position().0, mouse_position().1));
    let fill_color = if hovered {
        GameColor::Primary.value()
    } else {
        GameColor::Text.value()
    };

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill_color);
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        2.0,
        GameColor::Secondary.value(),
    );

    let text_size = measure_text(label, None, BUTTON_FONT_SIZE as u16, 1.0);
    draw_text(
        label,
        rect.x + rect.w / 2.0 - text_size.width / 2.0,
        rect.y + rect.h / 2.0 + text_size.height / 3.0,
        BUTTON_FONT_SIZE,
        GameColor::ButtonText.value(),
    );
}

fn button_clicked(rect: Rect) -> bool {
    is_mouse_button_pressed(MouseButton::Left)
        && rect.contains(vec2(mouse_position().0, mouse_position().1))
}
