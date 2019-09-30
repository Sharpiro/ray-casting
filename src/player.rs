use graphics::*;

use colors;

const DEGREES_90_RADIANS: f64 = 1.5708;

struct Point {
    x: f64,
    y: f64,
}

pub struct Player {
    pub x: f64,
    pub y: f64,
    pub rotation_rad: f64,
}

impl Player {
    pub fn debug(&self, block_size: f64) -> &[&str] {
        &["hello", "world"]
    }
    pub fn draw(
        &self,
        context: graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        block_size: f64,
    ) {
        // draw player
        let line_rot = context
            .transform
            .trans(block_size * self.x, block_size * self.y)
            .rot_rad(self.rotation_rad);
        // line(colors::BLACK, 1.0, [0.0, 0.0, 800.0, 0.0], line_rot, gl);
        rectangle(
            colors::BLACK,
            [0.0, 0.0, 10.0, 10.0],
            line_rot.trans(-5.0, -5.0),
            gl,
        );
        self.draw_intercepts(context, gl, block_size);
    }

    fn draw_intercepts(
        &self,
        context: graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        block_size: f64,
    ) {
        let mut x_intercept = self.get_initial_x_intercept(block_size);
        self.draw_intercept(context, gl, &x_intercept, colors::RED_ALPHA);

        let mut y_intercept = self.get_initial_y_intercept(block_size);
        self.draw_intercept(context, gl, &y_intercept, colors::BLUE_ALPHA);

        let skip_x = false;
        let skip_y = false;
        let mut point_calculations = 0;
        for _ in 0..5 {
            if !skip_x {
                x_intercept = self.get_x_intercept(block_size, &x_intercept);
                self.draw_intercept(context, gl, &x_intercept, colors::RED_ALPHA);
                point_calculations += 1;
            }

            if !skip_y {
                y_intercept = self.get_y_intercept(block_size, &y_intercept);
                self.draw_intercept(context, gl, &y_intercept, colors::BLUE_ALPHA);
                point_calculations += 1;
            }
        }
    }

    fn get_initial_x_intercept(&self, block_size: f64) -> Point {
        let x_value = if self.rotation_rad.sin() > 0.0 {
            (block_size * 4.0) + ((DEGREES_90_RADIANS - self.rotation_rad).tan() * block_size)
        } else {
            (block_size * 4.0) - ((DEGREES_90_RADIANS - self.rotation_rad).tan() * block_size)
        };
        let y_value = if self.rotation_rad.sin() > 0.0 {
            block_size * 4.0
        } else {
            block_size * 2.0
        };
        Point {
            x: x_value,
            y: y_value,
        }
    }

    fn get_x_intercept(&self, block_size: f64, last_point: &Point) -> Point {
        let x_value = if self.rotation_rad.sin() > 0.0 {
            last_point.x + (DEGREES_90_RADIANS - self.rotation_rad).tan() * block_size
        } else {
            last_point.x - (DEGREES_90_RADIANS - self.rotation_rad).tan() * block_size
        };
        let y_value = if self.rotation_rad.sin() > 0.0 {
            last_point.y + block_size
        } else {
            last_point.y - block_size
        };
        Point {
            x: x_value,
            y: y_value,
        }
    }

    fn get_initial_y_intercept(&self, block_size: f64) -> Point {
        let x_value = if self.rotation_rad.cos() > 0.0 {
            block_size * 5.0
        } else {
            block_size * 3.0
        };
        let y_value = if self.rotation_rad.cos() > 0.0 {
            (block_size * 3.0) + (self.rotation_rad.tan() * block_size)
        } else {
            (block_size * 3.0) - (self.rotation_rad.tan() * block_size)
        };
        Point {
            x: x_value,
            y: y_value,
        }
    }

    fn get_y_intercept(&self, block_size: f64, last_point: &Point) -> Point {
        let x_value = if self.rotation_rad.cos() > 0.0 {
            last_point.x + block_size
        } else {
            last_point.x - block_size
        };
        let y_value = if self.rotation_rad.cos() > 0.0 {
            last_point.y + self.rotation_rad.tan() * block_size
        } else {
            last_point.y - self.rotation_rad.tan() * block_size
        };
        Point {
            x: x_value,
            y: y_value,
        }
    }

    fn draw_intercept(
        &self,
        context: graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        point: &Point,
        color: [f32; 4],
    ) {
        let xform = context.transform.trans(point.x, point.y).trans(-5.0, -5.0);
        rectangle(color, [0.0, 0.0, 10.0, 10.0], xform, gl);
    }
}
