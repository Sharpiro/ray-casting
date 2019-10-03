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
    board: Vec<u32>,
    block_size: f64,
    tiles_x: u32,
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

            draw_grid(c, gl, self.block_size, self.tiles_x);
            self.draw_board(c, gl, self.block_size);
            self.player.draw(c, gl, self.block_size);
            let mut display_vector = vec![self.player.to_string()];
            display_vector.push(format!("sin: {}", self.player.rotation_rad.sin()));
            display_vector.push(format!("cos: {}", self.player.rotation_rad.cos()));
            display_vector.push(format!("tan: {}", self.player.rotation_rad.tan()));
            draw_text(c, gl, glyphs, self.block_size, self.tiles_x, display_vector);
        });
    }

    fn draw_board(
        &self,
        c: graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        block_size: f64,
    ) {
        for (i, &cell) in (0..).zip(self.board.iter()) {
            let color = match cell {
                1 => Some(colors::RED_ALPHA),
                2 => Some(colors::BLUE_ALPHA),
                3 => Some(colors::GREEN_ALPHA),
                4 => Some(colors::ORANGE_ALPHA),
                _ => None,
            };

            if let Some(color) = color {
                let (y, x) = App::div_mod(i, self.tiles_x);
                graphics::rectangle(
                    color,
                    [
                        x as f64 * block_size,
                        y as f64 * block_size,
                        block_size,
                        block_size,
                    ],
                    c.transform,
                    gl,
                );
            };
        }
    }

    fn div_mod(left: u32, right: u32) -> (u32, u32) {
        let quotient = left / right;
        let modulus = left - quotient * right;
        (quotient, modulus)
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.player.update(self.block_size, &self.board);
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
                Key::Left => {
                    self.player.x -= 1.0;
                }
                Key::Right => {
                    self.player.x += 1.0;
                }
                Key::Up => {
                    self.player.y -= 1.0;
                }
                Key::Down => {
                    self.player.y += 1.0;
                }
                _ => {}
            }
        }
    }
}

fn draw_text(
    context: graphics::Context,
    gl: &mut opengl_graphics::GlGraphics,
    glyphs: &mut GlyphCache,
    block_size: f64,
    tiles_x: u32,
    lines: Vec<String>,
) {
    const FONT_SIZE: u32 = 13;
    let board_end = block_size * tiles_x as f64;
    let text_start = board_end + 25.0;
    for (i, line) in lines.into_iter().enumerate() {
        let location = context.transform.trans(10.0, i as f64 * 15.0 + text_start);
        graphics::text(colors::BLACK, FONT_SIZE, &line, glyphs, location, gl)
            .expect("write text failure");
    }
}

fn main() {
    // let _temp: graphics::math::Matrix2d = [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0]];
    // return;
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("ray-casting", [1000, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    let mut gl = GlGraphics::new(opengl);
    let tiles_x = 10;
    let mut app = App {
        player: player::Player {
            x: 4.0,
            y: 3.0,
            // rotation_rad: -0.5654870999999999,
            // rotation_rad: 0.0,
            rotation_rad: 0.69115,
            // rotation_rad: 1.3191234,
            x_intercepts: vec![],
            y_intercepts: vec![],
            count: 0,
        },
        block_size: 50.0,
        board: load_board(tiles_x, tiles_x),
        tiles_x: 10,
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

#[rustfmt::skip]
fn load_board(_tiles_x: usize, _tiles_y: usize) -> Vec<u32> {
   vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 1, 1, 1, 1, 1, 1, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 3, 3, 3, 3, 3, 3, 3, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]
}

fn draw_grid(
    context: graphics::Context,
    graphics: &mut opengl_graphics::GlGraphics,
    block_size: f64,
    tiles_x: u32,
) {
    for i in 1..10 {
        let offset = i as f64;
        graphics::line(
            colors::BLACK,
            1.0,
            [
                offset * block_size,
                0.0,
                offset * block_size,
                tiles_x as f64 * block_size,
            ],
            context.transform,
            graphics,
        );
        graphics::line(
            colors::BLACK,
            1.0,
            [
                0.0,
                offset * block_size,
                tiles_x as f64 * block_size,
                offset * block_size,
            ],
            context.transform,
            graphics,
        );
    }
}
