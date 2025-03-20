// SUDOKU in Rust
// (c) 2024 by markus dot mueller dot 73 at hotmail dot de
// a simple sudoku game with a GUI
// the GUI is made with macroquad (in the web: macroquad dot rs)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the “Software”), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
// SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod board;
mod button;
mod colors;
mod constants;
mod preferences;
mod rendering;

use std::time::SystemTime;
use macroquad::{
    main,
    window::Conf,
    prelude::*,
};
use crate::board::*;
use crate::button::*;
use crate::constants::*;
use crate::preferences::*;
use crate::rendering::*;

#[derive(PartialOrd, PartialEq)]
pub enum GameMode {
    MainMenu,
    InGame,
    EndGame,
}

#[main(game_window)]
async fn main() -> Result<(), i32> {

    // load config
    let config = Preferences::load();

    // init
    let (mut select_x,mut select_y): (i32,i32) = (-1,-1);
    let (mut sel_menu_x,mut sel_menu_y): (f32,f32) = (0.0,0.0);
    let (mut mouse_x, mut mouse_y): (f32,f32);
    let mut number_selection: bool = false;
    let mut mouse_left_click: bool;
    let mut mouse_right_click: bool;

    let mut start_time = SystemTime::now();
    let mut game_duration: u64 = 0;

    let board_size = (BOARD_SIZE * CELL_SIZE) as f32;
    let cell_size = CELL_SIZE as f32;

    // init my simple GUI
    let mut game_mode = GameMode::MainMenu;
    let mut buttons = match Buttons::new() {
        Ok(buttons) => buttons,
        Err(err) => {
            eprintln!("{}", err);
            return Err(1)
        }
    };
    gamemode_mainmenu(&mut buttons, board_size, cell_size);

    // init the renderer
    let mut render = match Rendering::init(board_size, cell_size, config.color_theme.clone()) {
        Ok(render) => render,
        Err(err) => {
            eprintln!("{}", err);
            return Err(2)
        }
    };

    // init the sudoku board arrays
    let mut board: SudokuBoard = SudokuBoard::default();

    // start loop
    'game_loop: loop {

        // catch keyboard  input
        // if is_key_released(KeyCode::Escape) {
        //     break 'game_loop;
        // }

        // get mouse coords and clicked buttons
        (mouse_x,mouse_y) = mouse_position();
        mouse_left_click = is_mouse_button_released(MouseButton::Left);
        mouse_right_click = is_mouse_button_released(MouseButton::Right);

        // process left mouse click
        if mouse_left_click {

            // number selection == a cell waas selected and now the player can choose a number for the cell
            if number_selection {

                // get the selected number in the small selection window
                let sel_num = render.selection_get_number(mouse_x, mouse_y, sel_menu_x, sel_menu_y);

                // check for vallid number
                if sel_num >= 1 && sel_num <= BOARD_SIZE as u8 {
                    if board.is_valid_move(select_x as usize, select_y as usize, sel_num) {
                        board.set_as_valid(select_x as usize, select_y as usize);
                    }
                    board.set_field(select_x as usize, select_y as usize, sel_num);
                }
                number_selection = false;

            // the player clicked in a cell, get the cell and show if its editable
            } else {

                select_x = (mouse_x / cell_size) as i32;
                select_y = (mouse_y / cell_size) as i32;

                // check if the click was inside the board
                if select_x >= 0 && select_x < BOARD_SIZE {

                    // if the cell isn't editable, reset the selection
                    if !board.is_editable(select_x as usize, select_y as usize) {
                        select_x = -1;
                        select_y = -1;

                    // else, reset the cell and activate the small selection window
                    } else {
                        board.set_field(select_x as usize, select_y as usize, 0);
                        board.set_as_invalid(select_x as usize, select_y as usize);
                        number_selection = true;
                    }
                }
            }
        }

        // process right mouse click
        if mouse_right_click {
            if number_selection { number_selection = false; }
            select_x = -1;
            select_y = -1;
        }

        // draw sudoku board
        render.board();

        // draw cell marker
        if !number_selection {
            if game_mode != GameMode::MainMenu {
                render.cell_marker(mouse_x, mouse_y);
            }
        }

        // draw selection marker
        if number_selection {
            render.cell_marker_cross(select_x, select_y);
        }

        // draw the board numbers
        render.board_numbers(&board);

        // draw the submenu to select a number
        if number_selection {
            (sel_menu_x,sel_menu_y) = render.selection_board(select_x, select_y);
            render.selection_marker(sel_menu_x, sel_menu_y, mouse_x, mouse_y);
        }

        // draw title in main menu
        if game_mode == GameMode::MainMenu {
            render.sudoku_title();
        } else {
            if game_mode == GameMode::InGame {
                game_duration = SystemTime::now().duration_since(start_time).unwrap().as_secs();
            }
            render.game_timer(game_duration);
        }

        // draw gui
        let sel_button: u32 = buttons.draw(mouse_x, mouse_y, &render);

        // if mouse left button was clicked, check if a button was clicked
        if mouse_left_click {

            match sel_button {

                BTN_NEW_EASY => {
                    board.edit(&BOARD_EASY.to_string());
                    game_mode = GameMode::InGame;
                    start_time = SystemTime::now();
                    gamemode_ingame(&mut buttons, board_size, cell_size);
                }

                BTN_NEW_MOD => {
                    board.edit(&BOARD_MOD.to_string());
                    game_mode = GameMode::InGame;
                    start_time = SystemTime::now();
                    gamemode_ingame(&mut buttons, board_size, cell_size);
                }

                BTN_NEW_HARD => {
                    board.edit(&BOARD_HARD.to_string());
                    game_mode = GameMode::InGame;
                    start_time = SystemTime::now();
                    gamemode_ingame(&mut buttons, board_size, cell_size);
                }

                BTN_LOAD => (), // TODO

                BTN_SAVE => {
                    game_duration = SystemTime::now().duration_since(start_time).unwrap().as_secs();
                    board.save(game_duration, false)
                },

                BTN_RESTART => {
                    start_time = SystemTime::now();
                    board.reset()
                },

                BTN_SOLVE => {
                    _ = board.solve();
                    game_mode = GameMode::EndGame;
                    game_duration = SystemTime::now().duration_since(start_time).unwrap().as_secs();
                    buttons.del_button(BTN_SOLVE);
                    buttons.del_button(BTN_RESTART);
                }

                BTN_BACK => {
                    board.clear();
                    game_mode = GameMode::MainMenu;
                    game_duration = SystemTime::now().duration_since(start_time).unwrap().as_secs();
                    gamemode_mainmenu(&mut buttons, board_size, cell_size);
                }

                BTN_QUIT => break 'game_loop,

                _ => ()

            }
        }

        // swap buffers
        next_frame().await

    }

    if game_mode == GameMode::InGame {
        game_duration = SystemTime::now().duration_since(start_time).unwrap().as_secs();
        board.save(game_duration, true);
    }

    match config.save() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("{}", err);
            return Err(3)
        }
    };

    Ok(())

}

fn game_window() -> Conf {
    Conf {
        window_title: GAME_TITLE.to_string(),
        window_width: BOARD_SIZE * CELL_SIZE + GUI_SIZE,
        window_height: BOARD_SIZE * CELL_SIZE,
        window_resizable: false,
        fullscreen: false,
        sample_count: 4,
        ..Default::default()
    }
}

fn gamemode_mainmenu(btns: &mut Buttons, board_size: f32, cell_size: f32) {
    let x = board_size + cell_size * 0.5;
    let bw = cell_size * 3.0;
    let bh = cell_size * 0.75;
    btns.reset();
    btns.new_button(BTN_NEW_EASY,x, cell_size * 0.5, bw, bh, "New easy board");
    btns.new_button(BTN_NEW_MOD,x, cell_size * 2.0, bw, bh, "New moderate board");
    btns.new_button(BTN_NEW_HARD,x, cell_size * 3.5, bw, bh, "New hard board");
    //btns.new_button(BTN_LOAD,x, cell_size * 5.5, bw, bh, "Load board");
    btns.new_button(BTN_QUIT,x, cell_size * 7.5, bw, bh, "Quit");
}

fn gamemode_ingame(btns: &mut Buttons, board_size: f32, cell_size: f32) {
    let x = board_size + cell_size * 0.5;
    let bw = cell_size * 3.0;
    let bh = cell_size * 0.75;
    btns.reset();
    btns.new_button(BTN_SOLVE, x, cell_size * 0.5, bw, bh, "Solve board");
    btns.new_button(BTN_RESTART,x, cell_size * 2.0, bw, bh, "Restart board");
    //btns.new_button(BTN_SAVE,x, cell_size * 3.5, bw, bh, "Save board");
    btns.new_button(BTN_BACK,x, cell_size * 6.0, bw, bh, "Back to Main");
    btns.new_button(BTN_QUIT,x, cell_size * 7.5, bw, bh, "Quit");
}
