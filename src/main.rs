#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
use speedy2d::{
    color::Color,
    dimen::{UVec2, Vec2},
    window::{
        KeyScancode, VirtualKeyCode, WindowCreationOptions, WindowHandler, WindowHelper,
        WindowPosition, WindowSize,
    },
    Graphics2D, Window,
};

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 480;

const PARTICLE_SIZE: f32 = 10.0;

fn main() {
    let window_size = UVec2::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let window_pixels = WindowSize::PhysicalPixels(window_size);
    let window = Window::new_with_options(
        "FLOATING",
        WindowCreationOptions::new_windowed(window_pixels, Some(WindowPosition::Center))
            .with_decorations(false)
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
}

impl App {
    pub const fn new(window_size: UVec2) -> Self {
        Self {
            viewport: window_size,
            active_particles: Vec::new(),
            dormant_particles: Vec::new(),
            mouse_pos: Vec2::ZERO,
        }
    }

    pub fn add_particle(&mut self, pos: Vec2) {
        self.active_particles.push(Particle::new(pos));
    }

    pub fn update(&mut self) {}

    pub fn draw(&self, graphics: &mut Graphics2D) {
        for particle in &self.dormant_particles {
            graphics.draw_circle(particle.pos, PARTICLE_SIZE, Color::LIGHT_GRAY);
        }

        for particle in &self.active_particles {
            graphics.draw_circle(particle.pos, PARTICLE_SIZE, Color::LIGHT_GRAY);
        }
    }
}

impl WindowHandler for App {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        self.update();

        graphics.clear_screen(Color::from_rgb(0.3, 0.3, 0.5));
        self.draw(graphics);

        std::thread::sleep(std::time::Duration::from_millis(60));
        helper.request_redraw();
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        self.viewport = size_pixels;
    }

    fn on_mouse_button_up(
        &mut self,
        helper: &mut WindowHelper<()>,
        button: speedy2d::window::MouseButton,
    ) {
        self.add_particle(self.mouse_pos);
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vec2) {
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
                key => println!("Key: {key:?}, scancode: {scancode}"),
            }
        }
    }
}

struct Particle {
    pos: Vec2,
}

impl Particle {
    fn new(pos: Vec2) -> Self {
        Self { pos }
    }
}
