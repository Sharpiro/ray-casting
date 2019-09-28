// cspell:disable
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use graphics::*;

pub struct App {
    gl: GlGraphics,
    _rotation: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        const BLOCK_SIZE: f64 = 100.0;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.90];

        let rotation = self._rotation;
        self.gl.draw(args.viewport(), |c, gl| {
            clear([1.0; 4], gl);

            draw_grid(c, gl, BLOCK_SIZE, 1_000.0, 1_000.0);
            draw_walls(c, gl, BLOCK_SIZE);

            let line_transform = c
                .transform
                .trans(BLOCK_SIZE * 4.0, BLOCK_SIZE * 3.0)
                .rot_rad(rotation);
            let box_transform = c
                .transform
                .trans(BLOCK_SIZE * 4.0, BLOCK_SIZE * 3.0)
                .rot_rad(rotation)
                .trans(-5.0, -5.0);
            line(RED, 1.0, [0.0, 0.0, 10_000.0, 0.0], line_transform, gl);
            rectangle(RED, [0.0, 0.0, 10.0, 10.0], box_transform, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self._rotation += 2.0 * _args.dt;
    }

    fn handle_event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::A => {
                    self._rotation += 0.0628319;
                }
                Key::D => {
                    self._rotation -= 0.0628319;
                }
                _ => {}
            }
        }
    }
}

fn draw_grid(
    context: graphics::Context,
    graphics: &mut opengl_graphics::GlGraphics,
    block_size: f64,
    board_width: f64,
    borad_height: f64
) {
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.5];

    for i in 1..10 {
        let offset = i as f64;
        graphics::line(
            BLACK,
            1.0,
            [offset * block_size, 0.0, offset * block_size, borad_height],
            context.transform,
            graphics,
        );
        graphics::line(
            BLACK,
            1.0,
            [0.0, offset * block_size, board_width, offset * block_size],
            context.transform,
            graphics,
        );
    }
}

fn draw_walls(c: graphics::Context, gl: &mut opengl_graphics::GlGraphics, block_size: f64) {
    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.90];
    const RED: [f32; 4] = [1.0, 0.0, 0.0, 0.90];
    const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 0.90];
    const ORANGE: [f32; 4] = [1.0, 0.6470588, 0.0, 0.90];

    graphics::rectangle(
        RED,
        [block_size, block_size, block_size * 8.0, block_size],
        c.transform,
        gl,
    );
    graphics::rectangle(
        BLUE,
        [block_size, block_size * 2.0, block_size, block_size * 5.0],
        c.transform,
        gl,
    );
    graphics::rectangle(
        GREEN,
        [block_size, block_size * 7.0, block_size * 8.0, block_size],
        c.transform,
        gl,
    );
    graphics::rectangle(
        ORANGE,
        [
            block_size * 8.0,
            block_size * 2.0,
            block_size,
            block_size * 5.0,
        ],
        c.transform,
        gl,
    );
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("spinning-square", [1000, 1000])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        _rotation: 0.5026552,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        app.handle_event(&e);
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
