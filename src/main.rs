extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::{math::Matrix2d, Transformed, Viewport};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, Key, MouseCursorEvent, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;
use ray::Ray;
use sharp_graphics::SharpGraphics;

mod colors;
mod display_vec;
mod maths;
mod player;
mod point;
mod ray;
mod sharp_graphics;

#[cfg(target_os = "linux")]
static TOP_OFFSET: f64 = 30.0;
#[cfg(target_os = "windows")]
static TOP_OFFSET: f64 = 0.0;

fn main() {
    let _temp_x: Viewport;
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("ray-casting", [1100, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // window.set_capture_cursor(capture_cursor); // doesn't work w/ current window type

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    let gl = GlGraphics::new(opengl);
    let mut _sharp_graphics = SharpGraphics::new(gl, glyphs);
    const TILES_X: usize = 10;
    let mut app = App {
        player: player::Player {
            position: point::Point { x: 4.0, y: 5.0 },
            angle: std::f64::consts::PI / 4.0,
            angle_tick: std::f64::consts::PI / -20.0,
            rays: vec![Ray::new(); 400],
        },
        block_size: 50.0,
        board: load_board(TILES_X, TILES_X),
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
            app.mouse_y = pos[1] - TOP_OFFSET;
        });

        if let Some(b) = e.press_args() {
            app.handle_input(&b);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(r) = e.render_args() {
            app.render(&r, &mut _sharp_graphics);
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
    fn render(&mut self, args: &RenderArgs, graphics: &mut SharpGraphics) {
        graphics.draw(args.viewport(), |context, graphics| {
            graphics.clear([1.0; 4]);

            draw_grid(context.transform, graphics, self.block_size, self.tiles_x);
            self.draw_board(context.transform, graphics, self.block_size);
            self.player.draw(context, graphics, self.block_size);
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
            draw_lines(
                context.transform,
                graphics,
                self.block_size,
                self.tiles_x,
                display_vector,
            );

            // 3d section
            // ceil
            const VIEW_WIDTH: f64 = 400.0;
            const VIEW_HEIGHT: f64 = 300.0;
            const VIEW_HEIGHT_HALF: f64 = VIEW_HEIGHT / 2.0;
            graphics.draw_rectangle(
                colors::GRAY_CEIL,
                [0.0, 0.0, VIEW_WIDTH, VIEW_HEIGHT_HALF],
                context.transform.trans(600.0, 0.0),
            );
            // floor
            graphics.draw_rectangle(
                colors::GRAY_FLOOR,
                [0.0, 0.0, VIEW_WIDTH, 150.0],
                context.transform.trans(600.0, VIEW_HEIGHT_HALF),
            );
            // wall
            self.draw_wall(
                &self.player.rays,
                VIEW_HEIGHT_HALF,
                graphics,
                context.transform,
            );
        });
    }

    fn draw_wall(
        &self,
        rays: &Vec<Ray>,
        view_height_half: f64,
        graphics: &mut SharpGraphics,
        transform: Matrix2d,
    ) {
        let _temp_rays: Vec<&Ray> = rays
            .iter()
            .filter(|r| r.wall_intersection.is_some())
            .collect();
        if _temp_rays.is_empty() {
            let _temp2 = 12;
            return;
        }

        for (i, ray) in rays.iter().enumerate() {
            let wall_height = ray.wall_height;
            let trans_y = view_height_half - wall_height / 2.0;
            let board_index = ray
                .wall_intersection
                .expect("bad intersection")
                .board_index
                .expect("bad index");
            let color = match self.board[board_index] {
                1 => colors::RED_ALPHA,
                2 => colors::BLUE_ALPHA,
                3 => colors::GREEN_ALPHA,
                4 => colors::ORANGE_ALPHA,
                _ => panic!("bad color"),
            };
            let color = color;
            graphics.draw_line(
                color,
                [0.0, 0.0, 0.0, wall_height],
                transform.trans(601.0 + i as f64, trans_y),
            );
        }
    }

    fn draw_board(&self, c: Matrix2d, graphics: &mut SharpGraphics, block_size: f64) {
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
                graphics.draw_rectangle(
                    color,
                    [
                        x as f64 * block_size,
                        y as f64 * block_size,
                        block_size,
                        block_size,
                    ],
                    c,
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
        const MOVE_STEP: f64 = 0.1;
        if let Button::Keyboard(key) = button {
            match key {
                Key::W => {
                    self.player.position.x += self.player.angle.cos() * MOVE_STEP;
                    self.player.position.y += self.player.angle.sin() * MOVE_STEP;
                }
                Key::S => {
                    self.player.position.x -= self.player.angle.cos() * MOVE_STEP;
                    self.player.position.y -= self.player.angle.sin() * MOVE_STEP;
                }
                Key::A => {
                    let perpendicular_angle = self.player.angle - std::f64::consts::FRAC_PI_2;
                    self.player.position.x += perpendicular_angle.cos() * MOVE_STEP;
                    self.player.position.y += perpendicular_angle.sin() * MOVE_STEP;
                }
                Key::D => {
                    let perpendicular_angle = self.player.angle + std::f64::consts::FRAC_PI_2;
                    self.player.position.x += perpendicular_angle.cos() * MOVE_STEP;
                    self.player.position.y += perpendicular_angle.sin() * MOVE_STEP;
                }
                Key::Left => {
                    self.player.angle += self.player.angle_tick;
                }
                Key::Right => {
                    self.player.angle -= self.player.angle_tick;
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
    transform: Matrix2d,
    graphics: &mut SharpGraphics,
    block_size: f64,
    tiles_x: u32,
    lines: Vec<String>,
) {
    let mut line_start = block_size * tiles_x as f64 + 25.0;
    for line in lines.into_iter() {
        line_start += draw_string(transform, graphics, line_start, line);
    }
}

fn draw_string(
    transform: Matrix2d,
    graphics: &mut SharpGraphics,
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

        let location = transform.trans(10.0, line_start + line_height_used);
        let slice = &data[start_index..end_index];
        graphics.draw_text(colors::BLACK, FONT_SIZE, slice, location)
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
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 3, 3, 3, 3, 3, 4, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]
}

fn draw_grid(
    transform: Matrix2d,
    sharp_graphics_x: &mut SharpGraphics,
    block_size: f64,
    tiles_x: u32,
) {
    for i in 1..10 {
        let offset = i as f64;
        sharp_graphics_x.draw_line(
            colors::BLACK,
            [
                offset * block_size,
                0.0,
                offset * block_size,
                tiles_x as f64 * block_size,
            ],
            transform,
        );

        sharp_graphics_x.draw_line(
            colors::BLACK,
            [
                0.0,
                offset * block_size,
                tiles_x as f64 * block_size,
                offset * block_size,
            ],
            transform,
        );
    }
}
