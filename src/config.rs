use configparser::ini::Ini;
use lazy_static::lazy_static;

use std::default::Default;
use std::error::Error;
use std::sync::RwLock;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::new());
}

pub struct Config {
    pub sleep_ms_per_frame: u64,
    pub window_width: usize,
    pub window_height: usize,
    pub decorations: bool,
    pub simple_mode: bool,

    pub particle_size: f32,
    pub new_particles: f32,
    pub gravity: f32,
    pub wind: f32,
    pub starting_speed: f32,
    pub max_particles: usize,
}

impl Config {
    fn new() -> Self {
        let mut config = Self::default();
        if let Err(error) = config.reload() {
            eprintln!("{error}");
            #[cfg(debug_assertions)]
            panic!();
        }
        config
    }

    pub fn reload(&mut self) -> Result<(), Box<dyn Error>> {
        let path = "config.ini";
        let mut ini = Ini::new();
        if ini.load(path).is_ok() {
            let default_section = "default";
            self.sleep_ms_per_frame = ini
                .getuint(default_section, "sleep_ms_per_frame")?
                .unwrap_or(60);
            self.window_width = ini
                .getuint(default_section, "window_width")?
                .unwrap_or(480)
                .try_into()
                .unwrap();
            self.window_height = ini
                .getuint(default_section, "window_height")?
                .unwrap_or(360)
                .try_into()
                .unwrap();
            self.decorations = ini.getbool(default_section, "decorations")?.unwrap_or(true);
            self.simple_mode = ini.getbool(default_section, "simple_mode")?.unwrap_or(true);

            self.particle_size = ini
                .getfloat(default_section, "particle_size")?
                .unwrap_or(1.0) as f32;
            self.new_particles = ini
                .getfloat(default_section, "new_particles")?
                .unwrap_or(0.05) as f32;
            self.gravity = ini.getfloat(default_section, "gravity")?.unwrap_or(0.04) as f32;
            self.wind = ini.getfloat(default_section, "wind")?.unwrap_or(0.3) as f32;
            self.starting_speed = ini
                .getfloat(default_section, "starting_speed")?
                .unwrap_or(0.7) as f32;
            self.max_particles = ini
                .getuint(default_section, "max_particles")?
                .unwrap_or(2000)
                .try_into()
                .unwrap();
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sleep_ms_per_frame: 30,
            window_width: 640,
            window_height: 360,
            decorations: false,
            simple_mode: false,

            particle_size: 1.0,
            new_particles: 0.05,
            gravity: 0.04,
            wind: 0.3,
            starting_speed: 0.7,
            max_particles: 2000,
        }
    }
}
