extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use board::Board;
use glutin_window::GlutinWindow as Window;
use graphics::{math::Matrix2d, Transformed};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{
    Button, Key, MouseCursorEvent, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
use piston::window::WindowSettings;
use point::{BoardPoint, ScreenPoint};
use ray::Ray;
use sharp_graphics::SharpGraphics;

mod board;
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

    let mut graphics = SharpGraphics::new(GlGraphics::new(opengl), glyphs);
    const TILES_X: usize = 10;
    const TILES_Y: usize = 20;
    const BLOCK_SIZE: f64 = 50.0;
    let mut app = App {
        board: board::Board::new(load_board(), TILES_X, TILES_Y, BLOCK_SIZE),
        player: player::Player {
            position: BoardPoint { x: 4.0, y: 4.0 },
            angle: 0.0,
            angle_tick: std::f64::consts::PI / -20.0,
            rays: vec![Ray::new(); 1],
            move_step: 0.1,
        },
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
            app.render(&r, &mut graphics);
        }
    }
}

struct App {
    player: player::Player,
    board: board::Board,
    mouse_x: f64,
    mouse_y: f64,
    dt: f64,
    fps: f64,
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

            self.board.draw(context.transform, graphics);
            self.player.draw(context.transform, graphics, &self.board);
            let mouse_screen_point = ScreenPoint {
                x: self.mouse_x,
                y: self.mouse_y,
            };
            let mouse_board_point = self.board.point_from(mouse_screen_point);
            let board_debug = format!(
                "board_x: {}, board_y: {}",
                mouse_board_point.x, mouse_board_point.y
            );
            let board_index = format!(
                "index: {},",
                self.board.get_index(BoardPoint {
                    x: mouse_board_point.x,
                    y: mouse_board_point.y,
                })
            );
            let mouse_debug = format!("mouse_x: {}, mouse_y: {}", self.mouse_x, self.mouse_y);
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
            draw_lines(context.transform, graphics, &self.board, display_vector);

            // 3d section
            // 3d ceil
            const VIEW_WIDTH: f64 = 400.0;
            const VIEW_HEIGHT: f64 = 300.0;
            const VIEW_HEIGHT_HALF: f64 = VIEW_HEIGHT / 2.0;
            graphics.draw_rectangle(
                colors::GRAY_CEIL,
                [0.0, 0.0, VIEW_WIDTH, VIEW_HEIGHT_HALF],
                context.transform.trans(600.0, 0.0),
            );
            // 3d floor
            graphics.draw_rectangle(
                colors::GRAY_FLOOR,
                [0.0, 0.0, VIEW_WIDTH, 150.0],
                context.transform.trans(600.0, VIEW_HEIGHT_HALF),
            );
            // 3d wall
            self.draw_3d_wall(
                &self.player.rays,
                VIEW_HEIGHT_HALF,
                graphics,
                context.transform,
            );
        });
    }

    fn draw_3d_wall(
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
            let color = match self.board.tiles[board_index] {
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

    fn update(&mut self, args: &UpdateArgs) {
        self.dt = args.dt;
        self.fps = 1.0 / self.dt;
        self.player.update(&self.board);
    }

    fn handle_input(&mut self, button: &Button) {
        if let Button::Keyboard(key) = button {
            match key {
                Key::W => {
                    self.player.move_forward(&self.board);
                }
                Key::S => {
                    self.player.move_backward(&self.board);
                }
                Key::A => {
                    self.player.strafe_left(&self.board);
                }
                Key::D => {
                    self.player.strafe_right(&self.board);
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
    board: &Board,
    lines: Vec<String>,
) {
    let mut line_start = board.block_size * board.tiles_x as f64 + 25.0;
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
fn load_board() -> Vec<u32> {
   vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 2, 1, 1, 1, 1, 1, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 4, 0, 0,
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
