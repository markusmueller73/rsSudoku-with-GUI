use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::path::Path;
use crate::constants::{CELL_SIZE, CONFIG_FILE, GAME_TITLE, SAVE_PATH};

#[derive(Debug,Default)]
pub struct Preferences {
    save_path: String,
    file_name: String,
    pub color_theme: String,
    pub cell_size: f32,
}

impl Preferences {

    pub fn load() -> Preferences {

        let mut prefs = Preferences {
            save_path: std::env::temp_dir().to_str().unwrap().to_string(),
            file_name: CONFIG_FILE.to_string(),
            color_theme: "Light".to_string(),
            cell_size: CELL_SIZE as f32,
        };

        let mut save_path: String = get_home_dir();
        if cfg!(target_os = "windows") {
            save_path.push('\\');
        } else {
            save_path.push('/');
        }
        save_path.push_str(SAVE_PATH);

        if !Path::new(&save_path).exists() {
            match fs::create_dir(&save_path) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Can't create game dir [{}]: {}", save_path, err);
                    return prefs
                }
            }
        }
        prefs.save_path = save_path;

        let cfg_path = Path::new(&prefs.save_path).join(&prefs.file_name);

        let file = match File::open(&cfg_path) {
            Ok(file)    => file,
            Err(err)    => {
                eprintln!("Can't read config file: {}", err);
                return prefs
            }
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {

            let l = line.unwrap_or_default();

            if l.is_empty() || l.starts_with('#') || l.starts_with(';') {
                continue;
            }

            if l.find('=').is_some() {

                let v: Vec<&str> = l.split('=').collect();

                match v[0].to_uppercase().as_str() {
                    "CELL_SIZE" => prefs.cell_size = v[1].parse::<f32>().unwrap_or(80.0),
                    "COLOR_THEME" => prefs.color_theme = v[1].to_string(),
                    _ => (),
                }

            }

        }

        prefs
    }

    pub fn save(&self) -> Result<(), String> {

        let cfg_path = Path::new(&self.save_path).join(&self.file_name);
        let file = match File::create(&cfg_path) {
            Ok(file)    => file,
            Err(err)    => return Err(format!("Can't create config file [{}]: {}.", cfg_path.display(), err))
        };

        let mut writer = BufWriter::new(file);

        writer.write_fmt(format_args!("# {} config file, edit or modify this file on your own risk\n\n", GAME_TITLE)).unwrap();
        //writer.write_fmt(format_args!("Save_Path={}\n", self.save_path)).unwrap();
        //writer.write_fmt(format_args!("File_Name={}\n", self.file_name)).unwrap();
        writer.write_fmt(format_args!("Cell_Size={}\n", self.cell_size)).unwrap();
        writer.write_fmt(format_args!("Color_Theme={}\n", self.color_theme)).unwrap();

        writer.flush().unwrap();

        Ok(())

    }

}


pub fn get_home_dir() -> String {

    let mut home_dir = String::new();

    if cfg!(target_os = "linux") {

        match std::env::var("HOME") {
            Ok(d) => {
                home_dir = d;
                home_dir.push('/');
                home_dir.push_str(".config")
            },
            Err(err) => {
                eprintln!("Error reading environment variable %HOME%: {}", err);
                home_dir = std::env::temp_dir().to_str().unwrap().to_string();
            }
        };

    }else if cfg!(target_os = "macos") {

        match std::env::var("HOME") {
            Ok(d) => {
                home_dir = d;
                home_dir.push('/');
                home_dir.push_str("Applications")
            },
            Err(err) => {
                eprintln!("Error reading environment variable %HOME%: {}", err);
                home_dir = std::env::temp_dir().to_str().unwrap().to_string();
            }
        };

    } else if cfg!(target_os = "windows") {

        match std::env::var("LOCALAPPDATA") {
            Ok(d) => {
                home_dir = d;
                home_dir.push('\\')
            },
            Err(err) => {
                eprintln!("Error reading environment variable %LOCALAPPDATA%: {}", err);
                home_dir = std::env::temp_dir().to_str().unwrap().to_string();
            }
        };

    }

    home_dir

}