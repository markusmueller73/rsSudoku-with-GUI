use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use macroquad::rand::{rand, srand};

use crate::constants::*;
use crate::preferences::get_home_dir;

#[derive(Clone, Debug, Default)]
pub struct SudokuBoard {
    board: [[u8; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    editable: [[bool; BOARD_SIZE as usize]; BOARD_SIZE as usize],
    valid: [[bool; BOARD_SIZE as usize]; BOARD_SIZE as usize],
}

#[allow(dead_code)]
impl SudokuBoard {

    fn new() -> SudokuBoard {
        let sb = SudokuBoard::default();
        sb
    }

    pub fn edit(&mut self, board_string: &String) {
        // create numbers from the virtual board
        let new_board = SudokuBoard::create(board_string);
        // iter through arrays
        let mut i: usize = 0;
        for y in 0..BOARD_SIZE as u8 {
            for x in 0..BOARD_SIZE as u8 {
                // get the number from the board string as char
                let value: u8 = new_board.as_bytes()[i] - 48;
                // check for valid number
                if value <= 9 {
                    self.board[x as usize][y as usize] = value;
                }
                // set all editable cells
                if value == 0 {
                    self.editable[x as usize][y as usize] = true;
                } else {
                    self.editable[x as usize][y as usize] = false;
                }
                // reset validity
                self.valid[x as usize][y as usize] = false;
                // counter for the chars in the board string
                i += 1;

            }
        }
        // get the system seconds for ...
        let sys_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        // ... a new seeding
        srand(sys_secs);
        // get a random number and modify the board
        let modifier: u32 = rand() % 8;
        match modifier {
            1 => self.turn_180(),
            2 => self.turn_ccw(),
            3 => self.turn_cw(),
            4 => self.flip_v(),
            5 => self.flip_h(),
            _ => (),
        }
    }

    fn calc_num(sign: u8) -> char {
        let mut s = sign + 48 + 1;
        if s > 57 { s -= 9; }
        s as char
    }

    fn create(code: &String) -> String {

        let mut new = String::new();

        let sys_secs = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        srand(sys_secs);
        let a: u8 = (rand() % 10) as u8;

        for n in 0..code.len() {

            let val: char = code.as_bytes()[n] as char;

            match val {
                'a' => new.push(SudokuBoard::calc_num(a)),
                'b' => new.push(SudokuBoard::calc_num(a + 1)),
                'c' => new.push(SudokuBoard::calc_num(a + 2)),
                'd' => new.push(SudokuBoard::calc_num(a + 3)),
                'e' => new.push(SudokuBoard::calc_num(a + 4)),
                'f' => new.push(SudokuBoard::calc_num(a + 5)),
                'g' => new.push(SudokuBoard::calc_num(a + 6)),
                'h' => new.push(SudokuBoard::calc_num(a + 7)),
                'i' => new.push(SudokuBoard::calc_num(a + 8)),
                'j' => new.push('0'),
                _ => eprintln!("Error during board creation, unknown char: {}", val)

            }
        }

        new

    }

    fn flip_h(&mut self) {
        let tmp = self.clone();
        let bsize: usize = BOARD_SIZE as usize;
        for y in 0..bsize {
            for x in 0..bsize {
                self.board[x][y] = tmp.board[x][bsize - y - 1];
                self.editable[x][y] = tmp.editable[x][bsize - y - 1];
                self.valid[x][y] = tmp.valid[x][bsize - y - 1];
            }
        }
    }

    fn flip_v(&mut self) {
        let tmp = self.clone();
        let bsize: usize = BOARD_SIZE as usize;
        for y in 0..bsize {
            for x in 0..bsize {
                self.board[x][y] = tmp.board[bsize - x - 1][y];
                self.editable[x][y] = tmp.editable[bsize - x - 1][y];
                self.valid[x][y] = tmp.valid[bsize - x - 1][y];
            }
        }
    }

    fn turn_180(&mut self) {
        let tmp = self.clone();
        let bsize: usize = BOARD_SIZE as usize;
        for y in 0..bsize {
            for x in 0..bsize {
                self.board[bsize - x - 1][bsize - y - 1] = tmp.board[x][y];
                self.editable[bsize - x - 1][bsize - y - 1] = tmp.editable[x][y];
                self.valid[bsize - x - 1][bsize - y - 1] = tmp.valid[x][y];
            }
        }
    }

    fn turn_ccw(&mut self) {
        let tmp = self.clone();
        let bsize: usize = BOARD_SIZE as usize;
        for y in 0..bsize {
            for x in 0..bsize {
                self.board[y][bsize - x - 1] = tmp.board[x][y];
                self.editable[y][bsize - x - 1] = tmp.editable[x][y];
                self.valid[y][bsize - x - 1] = tmp.valid[x][y];
            }
        }
    }

    fn turn_cw(&mut self) {
        let tmp = self.clone();
        let bsize: usize = BOARD_SIZE as usize;
        for y in 0..bsize {
            for x in 0..bsize {
                self.board[bsize - y - 1][x] = tmp.board[x][y];
                self.editable[bsize - y - 1][x] = tmp.editable[x][y];
                self.valid[bsize - y - 1][x] = tmp.valid[x][y];
            }
        }
    }

    pub fn reset(&mut self) {
        for y in 0..BOARD_SIZE as u8 {
            for x in 0..BOARD_SIZE as u8 {
                if self.editable[x as usize][y as usize] {
                    self.board[x as usize][y as usize] = 0;
                }
                self.valid[x as usize][y as usize] = false;
            }
        }
    }

    pub fn clear(&mut self) {
        for y in 0..BOARD_SIZE as u8 {
            for x in 0..BOARD_SIZE as u8 {
                self.board[x as usize][y as usize] = 0;
                self.editable[x as usize][y as usize] = false;
                self.valid[x as usize][y as usize] = false;
            }
        }
    }

    pub fn get_field(&self, x: usize, y: usize) -> u8 {
        self.board[x][y]
    }

    pub fn set_field(&mut self, x: usize, y: usize, value: u8) {
        self.board[x][y] = value;
    }

    pub fn set_as_valid(&mut self, x: usize, y: usize) {
        self.valid[x][y] = true;
    }

    pub fn set_as_invalid(&mut self, x: usize, y: usize) {
        self.valid[x][y] = false;
    }

    pub fn was_valid(&self, x: usize, y: usize) -> bool {
        self.valid[x][y]
    }

    pub fn is_editable(&self, x: usize, y: usize) -> bool {
        self.editable[x][y]
    }

    fn is_in_row(&self, y: usize, value: u8) -> bool {
        for x in 0..9 {
            if self.get_field(x, y) == value {
                return true;
            }
        }
        false
    }

    fn is_in_col(&self, x: usize, value: u8) -> bool {
        for y in 0..9 {
            if self.get_field(x, y) == value {
                return true;
            }
        }
        false
    }

    fn is_in_block(&self, x: usize, y: usize, value: u8) -> bool {
        let start_x: usize = x - x % FIELD_SIZE as usize;
        let start_y: usize = y - y % FIELD_SIZE as usize;
        for y in 0..FIELD_SIZE as usize {
            for x in 0..FIELD_SIZE as usize {
                if self.get_field(start_x + x, start_y + y) == value {
                    return true;
                }
            }
        }
        false
    }

    fn next_empty_field(&self) -> Option<(usize, usize)> {

        for col in 0..BOARD_SIZE {

            for row in 0..BOARD_SIZE {

                if self.get_field(row as usize, col as usize) == 0 {
                    return Some((row as usize, col as usize));
                }

            }

        }

        None

    }

    pub fn is_valid_move(&self, x: usize, y: usize, value: u8) -> bool {
        if !self.is_in_row(y, value) && !self.is_in_col(x, value) && !self.is_in_block(x, y, value) {
            return true;
        }
        false
    }

    pub fn solve(&mut self) -> bool {

        if let Some((row, col)) = self.next_empty_field() {

            for value in 1..=9  {

                if self.is_valid_move(row, col, value as u8) {

                    self.set_field(row, col, value as u8);

                    if self.solve() {
                        return true;
                    }

                    self.set_field(row, col, 0);

                }

            }

            return  false;

        }

        true

    }

    pub fn load(&mut self, file_name: String) -> bool {

        let full_path: PathBuf;
        full_path = Path::new(&get_home_dir()).join(SAVE_PATH).join(file_name);

        let file = match File::open(&full_path) {
            Ok(file)    => file,
            Err(err)    => {
                eprintln!("Can't open sudoku [{}]: {}.", full_path.display(), err);
                return false
            }
        };

        let reader = BufReader::new(file);
        let mut game_vec: Vec<String> = Vec::new();

        for line in reader.lines() {

            let l = line.unwrap_or_default();

            if l.is_empty() {
                continue;
            }

            for s in l.split(',') {
                game_vec.push(s.to_string());
            }

        }

        self.edit(&game_vec[0]);
        let _duration = game_vec[1].parse::<u64>();

        true

    }

    pub fn save(&self, duration: u64, default_name: bool) {

        let mut file_name: String;

        if default_name {

            file_name = DEFAULT_FILE.to_string();
            file_name.push('.');
            file_name.push_str(FILE_EXT);

        } else {

            let sys_secs = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let date: (u32,u8,u8) = get_date_from_days((sys_secs / 86_400) as u64);
            let time: (u8,u8,u8) = get_time_from_seconds((sys_secs % 86_400) as u64);

            file_name = String::from(format!("sudoku_{}-{}-{}_{}-{}-{}.{}",
                                                 date.0, date.1, date.2, time.0, time.1, time.2, FILE_EXT));
        }

        let path_name = Path::new(&get_home_dir()).join(SAVE_PATH).join(file_name);
        let file = match File::create(&path_name) {
            Ok(file)    => file,
            Err(err)    => {
                eprintln!("Can't save sudoku: {}", err);
                return
            }
        };

        let mut game_s = String::new();
        for row in 0..BOARD_SIZE as u8 {
            for col in 0..BOARD_SIZE as u8 {
                game_s.push((self.board[col as usize][row as usize] + 48) as char);
            }
        }

        let mut writer = BufWriter::new(file);
        writer.write_fmt(format_args!("{},{}", game_s, duration)).unwrap();
        writer.flush().unwrap();

    }

}

fn get_date_from_days(days: u64) -> (u32,u8,u8) {
    let z: i64 = days as i64 + 719_468;
    let era = if z >= 0 {
        z / 146_097
    } else {
        (z - 146_096) / 146_097
    };
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let mut date: (u32,u8,u8) = (y as u32, m as u8, d as u8);
    if m <= 2 {
        date.0 = (y + m) as u32;
    }
    date
}

pub fn get_time_from_seconds(seconds: u64) -> (u8,u8,u8) {
    let mut s = seconds;
    let h = s / 3_600;
    s -= h * 3_600;
    let m = s / 60;
    s -= m * 60;
    let time: (u8,u8,u8) = (h as u8, m as u8, s as u8);
    time
}


