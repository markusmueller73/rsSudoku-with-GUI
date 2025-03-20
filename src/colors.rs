use macroquad::color::Color;

#[derive(Default)]
pub enum ColorTheme {
    #[default]
    Unknown,
    Light,
    Dark,
}

#[derive(Default)]
pub struct Colors {
    theme_id: ColorTheme,
    theme_name: String,
    pub wnd_bkgrd: Color,
    pub btn_bkgrd_lt: Color,
    pub btn_bkgrd_md: Color,
    pub btn_bkgrd_dk: Color,
    pub btn_area: Color,
    pub btn_area_hover: Color,
    pub btn_text: Color,
    pub btn_text_hover: Color,
    pub board_bkgrd: Color,
    pub board_border: Color,
    pub cell_line_thick: Color,
    pub cell_line_thin: Color,
    pub cell_selector: Color,
    pub cell_selector_line: Color,
    pub selection_bkgrd: Color,
    pub selection_border: Color,
    pub selection_line: Color,
    pub selection_selector: Color,
    pub selection_text: Color,
    pub number_default: Color,
    pub number_editable: Color,
    pub number_notvalid: Color,
}

#[allow(dead_code)]
impl Colors {

    pub fn new (theme_name: String) -> Colors {
        let theme_id = Colors::get_theme_by_string(theme_name);
        let result = Colors::fetch_colors(theme_id);
        result
    }

    fn get_theme_by_string(theme_name: String) -> ColorTheme {
        let result: ColorTheme;
        match theme_name.trim().to_uppercase().as_str() {
            "DARK" => result = ColorTheme::Dark,
            _ => result = ColorTheme::Light,
        }
        result
    }

    fn get_theme_name(theme_id: ColorTheme) -> String {
        let result: String;
        match theme_id {
            ColorTheme::Dark => result = "Dark".to_string(),
            ColorTheme::Light => result = "Light".to_string(),
            _ => result = "Unknown".to_string(),
        }
        result
    }

    fn fetch_colors(theme_id: ColorTheme) -> Colors {

        let mut cols: Colors = Colors::default();

        match theme_id {

            ColorTheme::Light => {
                cols.theme_id = ColorTheme::Light;
                cols.theme_name = "Light".to_string();
                cols.wnd_bkgrd = Color::new(0.9, 0.9, 0.9, 1.0 );
                cols.btn_bkgrd_lt = Color::new(0.7, 0.7, 0.7, 1.0 );
                cols.btn_bkgrd_md = Color::new(0.5, 0.5, 0.5, 1.0 );
                cols.btn_bkgrd_dk = Color::new(0.3, 0.3, 0.3, 1.0 );
                cols.btn_area = Color::new(0.0, 0.2, 0.8, 1.0 );
                cols.btn_area_hover = Color::new(0.8, 0.6, 0.2, 1.0 );
                cols.btn_text = Color::new(0.9, 0.9, 0.9, 1.0 );
                cols.btn_text_hover = Color::new(0.1, 0.1, 0.1, 1.0 );
                cols.board_bkgrd = Color::new(0.9, 0.9, 0.7, 1.0 );
                cols.board_border = Color::new(0.6, 0.0, 0.0, 1.0 );
                cols.cell_line_thick = Color::new(0.2, 0.2, 0.2, 1.0 );
                cols.cell_line_thin = Color::new(0.5, 0.5, 0.5, 1.0 );
                cols.cell_selector = Color::new(0.0, 0.8, 0.0, 0.5 );
                cols.cell_selector_line = Color::new(0.0, 0.6, 0.0, 0.5 );
                cols.selection_bkgrd = Color::new(0.4, 0.4, 0.4, 1.0 );
                cols.selection_border = Color::new(0.8, 0.8, 0.0, 1.0 );
                cols.selection_line = Color::new(0.8, 0.8, 0.8, 1.0 );
                cols.selection_selector = Color::new(0.9, 0.7, 0.0, 0.5 );
                cols.selection_text = Color::new(0.3, 0.3, 0.3, 1.0 );
                cols.number_default = Color::new(0.1, 0.1, 0.1, 1.0 );
                cols.number_editable = Color::new(0.0, 0.0, 0.3, 1.0 );
                cols.number_notvalid = Color::new(0.5, 0.0, 0.0, 1.0 );
            }

            ColorTheme::Dark => {
                cols.theme_id = ColorTheme::Dark;
                cols.theme_name = "Dark".to_string( );
                cols.wnd_bkgrd = Color::new(0.1, 0.1, 0.1, 1.0 );
                cols.btn_bkgrd_lt = Color::new(0.4, 0.4, 0.4, 1.0 );
                cols.btn_bkgrd_md = Color::new(0.3, 0.3, 0.3, 1.0 );
                cols.btn_bkgrd_dk = Color::new(0.2, 0.2, 0.2, 1.0 );
                cols.btn_area = Color::new(0.0, 0.1, 0.4, 1.0 );
                cols.btn_area_hover = Color::new(0.5, 0.3, 0.0, 1.0 );
                cols.btn_text = Color::new(0.7, 0.7, 0.7, 1.0 );
                cols.btn_text_hover = Color::new(0.2, 0.2, 0.2, 1.0 );
                cols.board_bkgrd = Color::new(0.3, 0.3, 0.3, 1.0 );
                cols.board_border = Color::new(0.4, 0.0, 0.0, 1.0 );
                cols.cell_line_thick = Color::new(0.1, 0.1, 0.1, 1.0 );
                cols.cell_line_thin = Color::new(0.2, 0.2, 0.2, 1.0 );
                cols.cell_selector = Color::new(0.0, 0.4, 0.0, 0.5 );
                cols.cell_selector_line = Color::new(0.0, 0.2, 0.0, 0.5 );
                cols.selection_bkgrd = Color::new(0.8, 0.8, 0.8, 1.0 );
                cols.selection_border = Color::new(0.4, 0.4, 0.0, 1.0 );
                cols.selection_line = Color::new(0.4, 0.4, 0.4, 1.0 );
                cols.selection_selector = Color::new(0.5, 0.3, 0.0, 0.5 );
                cols.selection_text = Color::new(0.2, 0.2, 0.2, 1.0 );
                cols.number_default = Color::new(0.4, 0.4, 0.4, 1.0 );
                cols.number_editable = Color::new(0.0, 0.0, 0.5, 1.0 );
                cols.number_notvalid = Color::new(0.5, 0.0, 0.0, 1.0 );
            }

            _ => ()

        }

        cols

    }

}
