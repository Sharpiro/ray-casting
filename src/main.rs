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
use ray::Ray;

mod colors;
mod maths;
mod player;
mod point;
mod ray;

fn main() {
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
    // const PLAYER_ROT_INC: f64 = 0.0628319;
    // const PLAYER_ROT_INC: f64 = 0.017453292519943295;
    // const PLAYER_ROT_INC: f64 = 0.03490658503988659;
    // const PLAYER_ROT_INC: f64 = 0.06981317007977318;
    // const PLAYER_ROT_INC: f64 = 0.39269908169872414;
    let mut app = App {
        player: player::Player {
            position: point::Point { x: 5.0, y: 7.0 },
            // angle: std::f64::consts::PI / -4.0,
            angle: -0.78539816339744783,
            angle_tick: 0.39269908169872414,
            // angle: -0.5654870999999999,
            // angle: -49.762,
            // angle: 0.69115,
            // angle: 1.3191234,
            rays: vec![Ray::new(); 1],
        },
        block_size: 50.0,
        board: load_board(tiles_x, tiles_x),
        tiles_x: 10,
        dt: 0.0,
        fps: 0.0,
        mouse_x: 0.0,
        mouse_y: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        e.mouse_cursor(|pos| {
            app.mouse_x = pos[0];
            app.mouse_y = pos[1] - 30.0;
        });

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

struct App {
    player: player::Player,
    board: Vec<u32>,
    block_size: f64,
    tiles_x: u32,
    dt: f64,
    fps: f64,
    mouse_x: f64,
    mouse_y: f64,
}

impl std::fmt::Display for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "App {{dt: {}, fps: {} }}", self.dt, self.fps)
    }
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
            let board_x = (self.mouse_x / self.block_size).floor();
            let board_y = (self.mouse_y / self.block_size).floor();
            let board_debug = String::from(format!("board_x: {}, board_y: {}", board_x, board_y));
            let board_index = String::from(format!(
                "index: {},",
                board_y * self.tiles_x as f64 + board_x,
            ));
            let mouse_debug = String::from(format!(
                "mouse_x: {}, mouse_y: {}",
                self.mouse_x, self.mouse_y
            ));
            let mut display_vector = vec![
                board_debug,
                board_index,
                mouse_debug,
                self.to_string(),
                self.player.to_string(),
            ];
            display_vector.push(format!("sin: {}", self.player.angle.sin()));
            display_vector.push(format!("cos: {}", self.player.angle.cos()));
            display_vector.push(format!("tan: {}", self.player.angle.tan()));
            display_vector.push(format!("x-es: {}", self.player.rays[0].x_intercepts));
            display_vector.push(format!("y-es: {}", self.player.rays[0].y_intercepts));
            draw_lines(c, gl, glyphs, self.block_size, self.tiles_x, display_vector);
            // self.draw_3d_wall(c, gl);
        });
    }

    // fn draw_3d_wall(&self, c: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
    //     graphics::rectangle(
    //         colors::BLACK,
    //         [0.0, 0.0, 200.0, self.player.wall_height],
    //         c.transform.trans(600.0, 0.0),
    //         gl,
    //     );
    // }

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
                let (y, x) = maths::div_mod(i, self.tiles_x);
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

    fn update(&mut self, args: &UpdateArgs) {
        self.dt = args.dt;
        self.fps = 1.0 / self.dt;
        self.player.update(self.block_size, &self.board);
    }

    fn handle_input(&mut self, button: &Button) {
        if let Button::Keyboard(key) = button {
            match key {
                Key::A => {
                    self.player.angle += self.player.angle_tick;
                }
                Key::D => {
                    self.player.angle -= self.player.angle_tick;
                }
                Key::Left => {
                    self.player.position.x -= 1.0;
                }
                Key::Right => {
                    self.player.position.x += 1.0;
                }
                Key::Up => {
                    self.player.position.y -= 1.0;
                }
                Key::Down => {
                    self.player.position.y += 1.0;
                }
                _ => {}
            }
        }
    }
}

fn draw_lines(
    context: graphics::Context,
    gl: &mut opengl_graphics::GlGraphics,
    glyphs: &mut GlyphCache,
    block_size: f64,
    tiles_x: u32,
    lines: Vec<String>,
) {
    let mut line_start = block_size * tiles_x as f64 + 25.0;
    for line in lines.into_iter() {
        line_start += draw_string(context, gl, glyphs, line_start, line);
    }
}

fn draw_string(
    context: graphics::Context,
    gl: &mut opengl_graphics::GlGraphics,
    glyphs: &mut GlyphCache,
    line_start: f64,
    data: String,
) -> f64 {
    const FONT_SIZE: u32 = 15;
    const LINE_LENGTH: usize = 150;
    const LINE_HEIGHT: f64 = 15.0;

    let mut line_height_used = 0.0;
    let mut end_index = 0;
    while end_index != data.len() {
        let start_index = end_index;
        end_index += std::cmp::min(data.len() - end_index, LINE_LENGTH);
        line_height_used += LINE_HEIGHT;

        let location = context.transform.trans(10.0, line_start + line_height_used);
        let slice = &data[start_index..end_index];
        graphics::text(colors::BLACK, FONT_SIZE, slice, glyphs, location, gl)
            .expect("write text failure");
    }

    line_height_used
}

#[rustfmt::skip]
fn load_board(_tiles_x: usize, _tiles_y: usize) -> Vec<u32> {
   vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 2, 1, 1, 1, 1, 1, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 3, 0, 0, 3, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 3, 4, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 3, 3, 3, 3, 3, 4, 0, 0,
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
