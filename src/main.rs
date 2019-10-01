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
    board: Vec<i32>,
    block_size: f64,
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

            draw_grid(c, gl, self.block_size, 1_000.0, 1_000.0);
            // draw_walls(c, gl, self.block_size);
            self.draw_board(c, gl, self.block_size);
            self.player.draw(c, gl, self.block_size);
            let player_display = format!("{}", self.player);
            let mut display_vector = vec![player_display];
            display_vector.push(String::from("other random data"));
            draw_text(c, gl, glyphs, display_vector);
        });
    }

    fn draw_board(
        &self,
        c: graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        block_size: f64,
    ) {
        for (i, &cell) in self.board.iter().enumerate() {
            let color = match cell {
                1 => Some(colors::RED_ALPHA),
                2 => Some(colors::BLUE_ALPHA),
                3 => Some(colors::GREEN_ALPHA),
                4 => Some(colors::ORANGE_ALPHA),
                _ => None,
            };

            if let Some(color) = color {
                let x = (i % 10) as f64;
                let y = (i as f64 / 10.0).floor();
                graphics::rectangle(
                    color,
                    [x * block_size, y * block_size, block_size, block_size],
                    c.transform,
                    gl,
                );
            };
        }
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.player.update(self.block_size);
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

fn draw_text(
    context: graphics::Context,
    gl: &mut opengl_graphics::GlGraphics,
    glyphs: &mut GlyphCache,
    lines: Vec<String>,
) {
    for (i, line) in lines.into_iter().enumerate() {
        let location = context.transform.trans(25.0, 1025.0 + i as f64 * 25.0);
        graphics::text(colors::BLACK, 25, &line, glyphs, location, gl).expect("write text failure");
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
    let tiles_x = 10;
    let tiles_y = 10;
    let mut app = App {
        player: player::Player {
            x: 4.0,
            y: 3.0,
            rotation_rad: -0.5654870999999999,
            x_intercepts: vec![],
            y_intercepts: vec![],
        },
        block_size: 100.0,
        board: load_board(tiles_x, tiles_y),
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
fn load_board(_tiles_x: usize, _tiles_y: usize) -> Vec<i32> {
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
