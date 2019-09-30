extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::*;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod colors;
mod player;

struct App {
    player: player::Player,
    block_size: f64,
}

fn draw_debug(lines: &[&str]) {
    println!("{:?}", lines);
}

impl App {
    fn render(
        &mut self,
        args: &RenderArgs,
        gl: &mut opengl_graphics::GlGraphics,
        glyphs: &mut GlyphCache,
    ) {
        gl.draw(args.viewport(), |c, gl| {
            clear([1.0; 4], gl);

            draw_text(c, gl, self.player.rotation_rad, glyphs);
            draw_grid(c, gl, self.block_size, 1_000.0, 1_000.0);
            draw_walls(c, gl, self.block_size);
            self.player.draw(c, gl, self.block_size);
            let player_debug = self.player.debug(self.block_size);
            draw_debug(player_debug);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // self._rotation += 2.0 * _args.dt; // Rotate 2 radians per second.
    }

    fn handle_input(&mut self, button: &Button) {
        if let Button::Keyboard(key) = button {
            match key {
                Key::A => {
                    self.player.rotation_rad += 0.0628319;
                }
                Key::D => {
                    self.player.rotation_rad -= 0.0628319;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("ray-casting", [1000, 1200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    let mut gl = GlGraphics::new(opengl);
    let mut app = App {
        player: player::Player {
            x: 4.0,
            y: 3.0,
            rotation_rad: -0.5654870999999999,
        },
        block_size: 100.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(b) = e.press_args() {
            app.handle_input(&b);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(r) = e.render_args() {
            app.render(&r, &mut gl, &mut glyphs);
        }
    }
}

fn draw_grid(
    context: graphics::Context,
    graphics: &mut opengl_graphics::GlGraphics,
    block_size: f64,
    board_width: f64,
    board_height: f64,
) {
    for i in 1..10 {
        let offset = i as f64;
        graphics::line(
            colors::BLACK,
            1.0,
            [offset * block_size, 0.0, offset * block_size, board_height],
            context.transform,
            graphics,
        );
        graphics::line(
            colors::BLACK,
            1.0,
            [0.0, offset * block_size, board_width, offset * block_size],
            context.transform,
            graphics,
        );
    }
}

fn draw_walls(c: graphics::Context, gl: &mut opengl_graphics::GlGraphics, block_size: f64) {
    graphics::rectangle(
        colors::RED_ALPHA,
        [block_size, block_size, block_size * 8.0, block_size],
        c.transform,
        gl,
    );
    graphics::rectangle(
        colors::BLUE_ALPHA,
        [block_size, block_size * 2.0, block_size, block_size * 5.0],
        c.transform,
        gl,
    );
    graphics::rectangle(
        colors::GREEN_ALPHA,
        [block_size, block_size * 7.0, block_size * 8.0, block_size],
        c.transform,
        gl,
    );
    graphics::rectangle(
        colors::ORANGE_ALPHA,
        [
            block_size * 7.0,
            block_size * 2.0,
            block_size,
            block_size * 5.0,
        ],
        c.transform,
        gl,
    );
}

fn draw_text(
    context: graphics::Context,
    gl: &mut opengl_graphics::GlGraphics,
    rotation_rad: f64,
    glyphs: &mut GlyphCache,
) {
    let location = context.transform.trans(25.0, 1025.0);
    let text = format!("radians: {}", rotation_rad);
    graphics::text(colors::BLACK, 25, &text, glyphs, location, gl).expect("write text failure");

    let location = context.transform.trans(25.0, 1050.0);
    let text = format!("cos_rad: {}", rotation_rad.cos());
    graphics::text(colors::BLACK, 25, &text, glyphs, location, gl).expect("write text failure");

    let location = context.transform.trans(25.0, 1075.0);
    let text = format!("sin_rad: {}", rotation_rad.sin());
    graphics::text(colors::BLACK, 25, &text, glyphs, location, gl).expect("write text failure");

    let location = context.transform.trans(25.0, 1100.0);
    let text = format!("tan_rad: {}", rotation_rad.tan());
    graphics::text(colors::BLACK, 25, &text, glyphs, location, gl).expect("write text failure");
}
