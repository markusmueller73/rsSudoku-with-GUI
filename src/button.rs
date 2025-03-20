use macroquad::{
    text::Font,
    prelude::*
};
use crate::rendering::*;

#[derive(Clone, Default)]
struct Button {
    id: u32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    txt: String,
    txt_x: f32,
    txt_y: f32,
    txt_size: u16,
}

pub struct Buttons {
    btns: Vec<Button>,
    font: Font,
}

impl Buttons {

    pub fn new() -> Result<Buttons, String> {
        let f = match load_ttf_font_from_bytes(include_bytes!("../resources/rimouski_sb.ttf")) {
            Ok(f) => f,
            Err(e) => return Err(format!("Can't load font: {}", e)),
        };
        Ok(Buttons {
            btns: Vec::new(),
            font: f,
        })
    }

    pub fn new_button(&mut self, id: u32, x_pos: f32, y_pos: f32, width: f32, height: f32, btn_text: &str) -> u32 {
        let new_id: u32;
        if id == 0 {
            new_id = self.get_highest_id();
        } else {
            new_id = id
        }
        let font_size: u16 = (height * 0.33) as u16;
        let td = measure_text(btn_text, Some(&self.font), font_size, 1.0);
        let offset_x: f32 = (width - td.width) * 0.5;
        let offset_y: f32 = (height - td.height) * 0.5 + td.offset_y;
        let new_btn = Button {
            id: new_id,
            x: x_pos,
            y: y_pos,
            w: width,
            h: height,
            txt: btn_text.to_string(),
            txt_x: x_pos + offset_x,
            txt_y: y_pos + offset_y,
            txt_size: font_size
        };
        self.btns.push(new_btn);
        new_id
    }

    pub fn del_button(&mut self, btn_id: u32) {
        let mut i: usize = 0;
        for btn in self.btns.iter() {
            if btn.id == btn_id {
                self.btns.remove(i);
                break;
            }
            i += 1;
        }
    }

    pub fn reset(&mut self) {
        self.btns.clear();
    }

    pub fn draw(&self, mouse_x: f32, mouse_y: f32, render: &Rendering) -> u32 {

        let mut result: u32 = 0;

        for btn in self.btns.iter() {

            // check if mouse is over or not
            let mut mouse_over: bool = false;
            if mouse_x >= btn.x && mouse_x <= btn.x + btn.w && mouse_y >= btn.y && mouse_y <= btn.y + btn.h {
                mouse_over = true;
                result = btn.id;
            }

            render.button(btn.x, btn.y, btn.w, btn.h, btn.txt_x, btn.txt_y, btn.txt_size, &btn.txt, mouse_over);

        }

        result

    }

    fn get_highest_id(&self) -> u32 {
        let mut result: u32 = 0;
        for btn in self.btns.iter() {
            if btn.id >= result { result = btn.id; }
        }
        result += 1;
        result
    }

}