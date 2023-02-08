#![warn(clippy::nursery)] //, clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![windows_subsystem = "windows"]
use speedy2d::{
    color::Color,
    dimen::{UVec2, Vec2},
    window::{
        KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
        WindowPosition, WindowSize,
    },
    Graphics2D, Window,
};

mod config;
use config::SETTINGS;

fn main() {
    let window_size = UVec2::new(
        SETTINGS.read().unwrap().window_width as u32,
        SETTINGS.read().unwrap().window_height as u32,
    );
    let window_pixels = WindowSize::PhysicalPixels(window_size);
    let window = Window::new_with_options(
        "Snowrest",
        WindowCreationOptions::new_windowed(window_pixels, Some(WindowPosition::Center))
            .with_decorations(SETTINGS.read().unwrap().decorations)
            .with_transparent(true),
    )
    .expect("Wasn't able to create a window!");
    window.run_loop(App::new(window_size));
}

struct App {
    viewport: UVec2,
    active_particles: Vec<Particle>,
    dormant_particles: Vec<Particle>,
    mouse_pos: Vec2,
    wind: Vec2,
}

impl App {
    pub const fn new(window_size: UVec2) -> Self {
        Self {
            viewport: window_size,
            active_particles: Vec::new(),
            dormant_particles: Vec::new(),
            mouse_pos: Vec2::ZERO,
            wind: Vec2::ZERO,
        }
    }

    pub fn add_particle(&mut self, pos: Vec2) {
        self.active_particles.push(Particle::new(pos));
    }

    pub fn update(&mut self) {
        self.wind += Vec2::new(
            (fastrand::f32() - 0.5) * 2.0 * SETTINGS.read().unwrap().wind,
            (fastrand::f32() - 0.5) * 2.0 * SETTINGS.read().unwrap().wind,
        );

        if self.wind.magnitude() >= SETTINGS.read().unwrap().wind * 10.0 {
            self.wind =
                self.wind.normalize().unwrap_or(Vec2::ZERO) * 10.0 * SETTINGS.read().unwrap().wind;
        }

        self.active_particles.iter_mut().for_each(|particle| {
            particle.pos += particle.vel + self.wind;

            if particle.pos.x >= self.viewport.x as f32 + 50.0 {
                particle.pos.x = -40.0
            } else if particle.pos.x <= -50.0 {
                particle.pos.x = self.viewport.x as f32 + 40.0
            }

            particle.vel += Vec2::new(0.0, SETTINGS.read().unwrap().gravity);
        });

        for mut p in std::mem::take(&mut self.active_particles) {
            let is_off_the_bottom_of_the_screen = p.pos.y >= self.viewport.y as f32;

            if is_off_the_bottom_of_the_screen {
                p.pos.y = self.viewport.y as f32;
                let x_is_inside_viewport = p.pos.x >= -SETTINGS.read().unwrap().particle_size
                    && p.pos.x <= self.viewport.x as f32 + SETTINGS.read().unwrap().particle_size;
                let y_is_inside_viewport = p.pos.y >= -SETTINGS.read().unwrap().particle_size
                    && p.pos.y <= self.viewport.y as f32 + SETTINGS.read().unwrap().particle_size;
                if x_is_inside_viewport && y_is_inside_viewport {
                    self.dormant_particles.push(p);
                }
            } else if let Some(pos) = self.touching_dormant_particle(&p) {
                p.pos = pos;
                self.dormant_particles.push(p);
            } else {
                self.active_particles.push(p);
            }
        }

        if self.dormant_particles.len() >= SETTINGS.read().unwrap().max_particles {
            let range_end = self.dormant_particles.len().min(
                SETTINGS.read().unwrap().max_particles / 10
                    + SETTINGS.read().unwrap().new_particles as usize,
            );
            self.dormant_particles.splice(0..range_end, []);
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        for particle in &self.dormant_particles {
            graphics.draw_circle(
                particle.pos,
                SETTINGS.read().unwrap().particle_size,
                Color::from_rgba(1.0, 1.0, 1.0, 1.0),
            );
        }

        for particle in &self.active_particles {
            graphics.draw_circle(
                particle.pos,
                SETTINGS.read().unwrap().particle_size,
                Color::from_rgba(1.0, 1.0, 1.0, 1.0),
            );
        }
    }

    fn touching_dormant_particle(&self, particle: &Particle) -> Option<Vec2> {
        for other_particle in self.dormant_particles.iter().rev().take(
            (SETTINGS.read().unwrap().window_width as f32 / SETTINGS.read().unwrap().particle_size)
                as usize
                * 4,
        ) {
            let distance = particle.pos - other_particle.pos;
            if distance.magnitude_squared() <= particle.vel.magnitude_squared() {
                return Some(
                    other_particle.pos
                        + Vec2::new(fastrand::f32() - 0.5, -fastrand::f32())
                            .normalize()
                            .unwrap()
                            * SETTINGS.read().unwrap().particle_size,
                );
            }
        }
        None
    }
}

impl WindowHandler for App {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        for _ in 0..SETTINGS.read().unwrap().new_particles.floor() as usize {
            self.add_particle(Vec2::new(
                fastrand::f32().mul_add(self.viewport.x as f32 + 80.0, -40.0),
                -10.0,
            ));
        }
        if fastrand::f32() < SETTINGS.read().unwrap().new_particles.fract() {
            self.add_particle(Vec2::new(
                fastrand::f32().mul_add(self.viewport.x as f32 + 80.0, -40.0),
                -10.0,
            ));
        }
        self.update();

        graphics.clear_screen(Color::from_rgba(0.0, 0.0, 0.0, 0.0));
        //graphics.clear_screen(Color::from_rgb(0.3, 0.3, 0.5));
        self.draw(graphics);

        std::thread::sleep(std::time::Duration::from_millis(
            SETTINGS.read().unwrap().sleep_ms_per_frame,
        ));
        helper.request_redraw();
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        self.viewport = size_pixels;
        let mut dormant = std::mem::take(&mut self.dormant_particles);
        self.active_particles.append(&mut dormant);
    }

    fn on_mouse_button_up(
        &mut self,
        _helper: &mut WindowHelper<()>,
        _button: speedy2d::window::MouseButton,
    ) {
        self.add_particle(self.mouse_pos);
    }

    fn on_mouse_move(&mut self, _helper: &mut WindowHelper<()>, position: Vec2) {
        self.mouse_pos = position;
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        if let Some(key_code) = virtual_key_code {
            match key_code {
                VirtualKeyCode::Escape => helper.terminate_loop(),
                VirtualKeyCode::Space => self.dormant_particles.clear(),
                key => println!("Key: {key:?}, scancode: {scancode}"),
            }
        }
    }
}

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Particle {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: Vec2::new(
                (fastrand::f32() - 0.5) * SETTINGS.read().unwrap().starting_speed,
                (fastrand::f32() - 0.5) * SETTINGS.read().unwrap().starting_speed,
            ),
        }
    }
}
