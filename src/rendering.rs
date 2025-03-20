use macroquad::{
    color::Color,
    math::Vec2,
    prelude::*,
    texture::Texture2D,
};
use crate::colors::Colors;
use crate::board::{get_time_from_seconds, SudokuBoard};

pub struct Rendering {
    b_size: f32,
    c_size: f32,
    col: Colors,
    col_title: Color,
    title_dir: f32,
    title_texture: Texture2D,
    number_font: Font,
}

impl Rendering {

    pub fn init(board_size: f32, cell_size: f32, color_theme: String) -> Result<Rendering,String> {

        let t = Texture2D::from_file_with_format(include_bytes!("../resources/sudoku_title.png"), Some(ImageFormat::Png));

        let f = match load_ttf_font_from_bytes(include_bytes!("../resources/rimouski_sb.ttf")) {
            Ok(f) => f,
            Err(e) => return Err(format!("Can't load font: {}", e)),
        };

        Ok( Rendering {
            b_size: board_size,
            c_size: cell_size,
            col: Colors::new(color_theme),
            col_title: Color::new(0.0, 0.8, 0.0, 0.75),
            title_dir: -0.001,
            title_texture: t,
            number_font: f,
        })

    }

    pub fn sudoku_title(&mut self) {

        self.col_title.g += self.title_dir;
        if self.col_title.g <= 0.6 {
            self.title_dir = 0.001
        } else if self.col_title.g >= 0.8 {
            self.title_dir = -0.001
        }

        let size = Vec2{x: self.b_size, y: self.c_size * 2.5};

        draw_texture_ex(
            &self.title_texture,
            0.0,
            (self.b_size - self.c_size * 2.5) / 2.0,
            self.col_title,
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            }
        );

    }

    pub fn board(&self) {

        clear_background(self.col.board_bkgrd);

        for i in 0..9 {
            let pos = i as f32 * self.c_size;
            draw_line(pos, 0.0, pos, self.b_size, 1.0, self.col.cell_line_thin);
            draw_line(0.0, pos, self.b_size, pos, 1.0, self.col.cell_line_thin);
        }

        for i in 1..=2 {
            let pos = 3.0 * i as f32 * self.c_size - 1.0;
            draw_line(pos, 0.0, pos, self.b_size, 3.0, self.col.cell_line_thick);
            draw_line(0.0, pos, self.b_size, pos, 3.0, self.col.cell_line_thick);
        }

        draw_rectangle_lines(0.0, 0.0, self.b_size, self.b_size, 3.0, self.col.board_border);

    }

    pub fn board_numbers(&self, board: &SudokuBoard) {

        let font_size: u16 = self.c_size as u16;

        // iter through board array
        for y in 0..9 {
            for x in 0..9 {

                // get number of the field
                let n = board.get_field(x, y);

                // get text dimensions
                let td = measure_text(n.to_string().as_str(), Some(&self.number_font), font_size, 1.0);
                let offset_x: f32 = (self.c_size - td.width) * 0.5;
                let offset_y: f32 = (self.c_size - td.height) * 0.5 + td.offset_y;

                if n >= 1 && n <= 9 {

                    let mut txt_col = self.col.number_default;
                    if board.is_editable(x, y) {
                        if board.was_valid(x, y) {
                            txt_col = self.col.number_editable;
                        } else {
                            txt_col = self.col.number_notvalid;
                        }
                    }

                    // draw number if is set
                    draw_text_ex(
                        &n.to_string(),
                        x as f32 * self.c_size + offset_x,
                        y as f32 * self.c_size + offset_y,
                        TextParams {
                            font_size,
                            font: Some(&self.number_font),
                            color: txt_col,
                            ..Default::default()
                        }
                    );

                }
            }
        }
    }

    pub fn cell_marker(&self, mouse_x: f32, mouse_y: f32) {
        if mouse_x <= self.b_size {
            let x: i32 = (mouse_x / self.c_size) as i32;
            let y: i32 = (mouse_y / self.c_size) as i32;
            draw_rectangle(x as f32 * self.c_size, y as f32 * self.c_size, self.c_size, self.c_size, self.col.cell_selector);
        }
    }

    pub fn cell_marker_cross(&self, selected_x: i32, selected_y: i32) {

        if selected_x >= 0 && selected_y >= 0 {

            // draw the line and row selection
            for i in 0..9 {
                if i != selected_x {
                    draw_rectangle(i as f32 * self.c_size, selected_y as f32 * self.c_size, self.c_size, self.c_size, self.col.cell_selector_line);
                }
                if i != selected_y {
                    draw_rectangle(selected_x as f32 * self.c_size, i as f32 * self.c_size, self.c_size, self.c_size, self.col.cell_selector_line);
                }
            }

            // draw the selected cell
            draw_rectangle(selected_x as f32 * self.c_size, selected_y as f32 * self.c_size, self.c_size, self.c_size,  self.col.cell_selector);

        }
    }

    pub fn selection_board(&self, selected_x: i32, selected_y: i32) -> (f32,f32) {

        // get text dimensions
        let font_size: u16 = (self.c_size / 10.0 * 8.0) as u16;

        let mut result_x: f32 = 0.0;
        let mut result_y: f32 = 0.0;

        // show only if a cell is selected
        if selected_x >= 0 && selected_y >= 0 {

            let sub_x: f32;
            if selected_x > 5 {
                sub_x = (selected_x - 3) as f32 * self.c_size;
            } else {
                sub_x = (selected_x + 1) as f32 * self.c_size;
            }

            let sub_y: f32;
            if selected_y > 5 {
                sub_y = (selected_y - 3) as f32 * self.c_size;
            } else {
                sub_y = (selected_y + 1) as f32 * self.c_size;
            }

            // draw the submenu background to select a number
            draw_rectangle(sub_x, sub_y, self.c_size * 3.0, self.c_size * 3.0, self.col.selection_bkgrd);
            result_x = sub_x;
            result_y = sub_y;

            // draw the gridlines
            for i in 0..3 {
                let pos_x = sub_x + i as f32 * self.c_size;
                let pos_y = sub_y + i as f32 * self.c_size;
                draw_line(pos_x, 0.0, pos_x, self.b_size, 1.0, self.col.selection_line);
                draw_line(0.0, pos_y, self.b_size, pos_y, 1.0, self.col.selection_line);
            }

            // draw the border
            draw_rectangle_lines(sub_x, sub_y, self.c_size * 3.0, self.c_size * 3.0, 2.0, self.col.selection_border);

            // draw numbers
            let mut n: u8 = 1;

            for y in 0..3 {

                let td = measure_text(n.to_string().as_str(), Some(&self.number_font), font_size, 1.0);
                let offset_x: f32 = (self.c_size - td.width) * 0.5;
                let offset_y: f32 = (self.c_size - td.height) * 0.5 + td.offset_y;

                let pos_y = sub_y + y as f32 * self.c_size + offset_y;

                for x in 0..3 {

                    let pos_x = sub_x + x as f32 * self.c_size + offset_x;

                    draw_text_ex(
                        &n.to_string(),
                        pos_x,
                        pos_y,
                        TextParams {
                            font_size,
                            font: Some(&self.number_font),
                            color: self.col.number_default,
                            ..Default::default()
                        }
                    );

                    n += 1;

                }
            }
        }

        (result_x,result_y)

    }

    pub fn selection_marker(&self, selected_x: f32, selected_y: f32, mouse_x: f32, mouse_y: f32) {

        let sel_len = self.c_size * 3.0;

        if mouse_x >= selected_x && mouse_x <= selected_x + sel_len && mouse_y >= selected_y && mouse_y <= selected_y + sel_len {

            let x: i32 = (mouse_x / self.c_size) as i32;
            let y: i32 = (mouse_y / self.c_size) as i32;
            draw_rectangle(x as f32 * self.c_size, y as f32 * self.c_size, self.c_size, self.c_size, self.col.selection_selector);

        }
    }

    pub fn  selection_get_number(&self, mouse_x: f32, mouse_y: f32, menu_x: f32, menu_y: f32) -> u8 {
        let cs3 = self.c_size * 3.0;
        if mouse_x >= menu_x && mouse_x <= menu_x + cs3 && mouse_y >= menu_y && mouse_y <= mouse_y + cs3 {
            let x = ((mouse_x - menu_x) / self.c_size) as i32;
            let y = ((mouse_y - menu_y) / self.c_size) as i32;
            let num: u8 = (y * 3 + x + 1) as u8;
            return num;
        }
        0
    }

    pub fn game_timer(&self, duration: u64) {

        let x = self.b_size + self.c_size * 0.5;
        let w = self.c_size * 3.0;
        let h = self.c_size * 0.75;
        let y = (self.b_size - h * 0.5) * 0.5;

        draw_rectangle(x, y, w, h,  self.col.btn_bkgrd_dk);
        draw_rectangle(x + 1.0, y + 1.0, w - 2.0, h - 2.0,  self.col.btn_bkgrd_md);
        draw_rectangle(x + 1.0, y + 2.0, w - 4.0, h - 4.0,  self.col.btn_bkgrd_lt);
        draw_rectangle(x + 1.0, y + 3.0, w - 6.0, h - 6.0,  self.col.btn_area);

        let font_size = (self.c_size * 0.5) as u16;
        let td = measure_text("0:00:00", Some(&self.number_font), font_size, 1.0);
        let offset_x: f32 = (w - td.width) * 0.5;
        let offset_y: f32 = (h - td.height) * 0.5 + td.offset_y;

        let (hour,min,sec) = get_time_from_seconds(duration);
        let timer = format!("{:01}:{:02}:{:02}", hour, min, sec);

        draw_text_ex(
            &timer,
            x + offset_x,
            y + offset_y,
            TextParams {
                font_size,
                font: Some(&self.number_font),
                color: self.col.btn_text,
                ..Default::default()
            }
        );
    }

    pub fn button(&self, x: f32, y: f32, w: f32, h: f32, txt_x: f32, txt_y: f32, txt_size: u16, txt: &String, selected: bool) {

        let text_color: Color;
        let btn_color: Color;
        // check if mouse is over or not
        if selected {
            text_color = self.col.btn_text_hover;
            btn_color = self.col.btn_area_hover;
        } else {
            text_color = self.col.btn_text;
            btn_color = self.col.btn_area;
        }

        // draw button background
        let h2 = h * 0.5;
        draw_circle(x, y + h2, h2, self.col.btn_bkgrd_dk);
        draw_circle(x, y + h2, h2 - 1.0, self.col.btn_bkgrd_md);
        draw_circle(x, y + h2, h2 - 2.0, self.col.btn_bkgrd_lt);
        draw_circle(x + w, y + h2, h2,  self.col.btn_bkgrd_dk);
        draw_circle(x + w, y + h2, h2 - 1.0,  self.col.btn_bkgrd_md);
        draw_circle(x + w, y + h2, h2 - 2.0,  self.col.btn_bkgrd_lt);
        draw_rectangle(x, y, w, h,  self.col.btn_bkgrd_dk);
        draw_rectangle(x, y + 1.0, w, h - 2.0,  self.col.btn_bkgrd_md);
        draw_rectangle(x, y + 2.0, w, h - 4.0,  self.col.btn_bkgrd_lt);

        // than draw button surface
        draw_circle(x, y + h2, h2 - 3.0, btn_color);
        draw_circle(x + w - 1.0, y + h2, h2 - 3.0, btn_color);
        draw_rectangle(x + 3.0, y + 3.0, w - 6.0, h - 6.0, btn_color);

        draw_text_ex(
            txt,
            txt_x,
            txt_y,
            TextParams {
                font_size: txt_size,
                font: Some(&self.number_font),
                color: text_color,
                ..Default::default()
            }
        );

    }

}