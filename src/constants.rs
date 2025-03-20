pub const GAME_TITLE: &str = "Sudoku";

// dimensions of the game
pub const CELL_SIZE: i32 = 80;
pub const BOARD_SIZE: i32 = 9;
pub const FIELD_SIZE: i32 = 3;
pub const GUI_SIZE: i32 = CELL_SIZE * 4;


// config and save files
#[cfg(target_os = "windows")]
pub const CONFIG_FILE: &str = "game.ini";
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub const CONFIG_FILE: &str = "game.conf";

pub const DEFAULT_FILE: &str = "_last";
pub const FILE_EXT: &str = "rsdk";
pub const SAVE_PATH: &str = "rs_sudoku";

// the board string
// we have only one string for one difficulty, but we can replace any char for a number =
// 9 x 9 = 81 different boards, plus we modify these boards with 6 modifiers (flipping, turning)
// 81 different boards x 6 modifiers = 486 boards for each difficulty
pub const BOARD_EASY: &str = "fecdjbgjiijhejcjbaadbjijjcfjjdcbjijhjjjfedjabbjjjhgjjejajhgjbfdjhijdfjecdbjjjejij";
pub const BOARD_MOD: &str = "jaejghjbjghjfjbjacjbjejaihjefdabjjjjbjjghidjjjjjjjjacbjjjhijjefigjjfebjjfjcjjdjgi";
pub const BOARD_HARD: &str = "jfjhjgjjdbjjjfjjighijjadjjjdejjhjjbjgjijjcjejjjjdjfjhijjhecafjjjdbjjjecjjjafjjijj";

// UI buttons
pub const BTN_NEW_EASY: u32 = 1;
pub const BTN_NEW_MOD: u32 = 2;
pub const BTN_NEW_HARD: u32 = 3;
pub const BTN_LOAD: u32 = 4;
pub const BTN_SAVE: u32 = 5;
pub const BTN_RESTART: u32 = 6;
pub const BTN_SOLVE: u32 = 7;
pub const BTN_BACK: u32 = 8;
pub const BTN_QUIT: u32 = 9;
