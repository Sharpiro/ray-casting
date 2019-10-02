use graphics::*;

use colors;

const DEGREES_90_RADIANS: f64 = 1.5708;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub rotation_rad: f64,
    pub x_intercepts: Vec<Point>,
    pub y_intercepts: Vec<Point>,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Player {{ count: {}, rot: {}, xes: {:?} }}",
            self.x_intercepts.len() + self.y_intercepts.len(),
            &self.rotation_rad.to_string()[0..7],
            self.x_intercepts
        )
    }
}

impl Player {
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
        self.draw_intercepts(context, gl);
    }

    pub fn update(&mut self, block_size: f64, board: &Vec<i32>) {
        let (x_intercepts, y_intercepts) = self.get_intercepts(block_size, board);
        self.x_intercepts = x_intercepts;
        self.y_intercepts = y_intercepts;
    }

    fn get_intercepts(&self, block_size: f64, board: &Vec<i32>) -> (Vec<Point>, Vec<Point>) {
        let mut x_intercept = self.get_initial_x_intercept(block_size);
        let mut y_intercept = self.get_initial_y_intercept(block_size);
        let mut x_intercepts = vec![x_intercept];
        // let mut y_intercepts = vec![y_intercept];
        let y_intercepts = vec![];

        let mut skip_x = false;
        // let mut skip_y = false;
        for _ in 0..3 {
            if !skip_x {
                x_intercept = self.get_x_intercept(block_size, x_intercept);
                // let x = (x_intercept.x / block_size).floor() + 1.0; // ceil?
                let x = (x_intercept.x / block_size).ceil();
                let y = x_intercept.y / block_size;
                let index = ((x + y * 10.0) - 1.0) as usize;
                if index >= 100 {
                    continue;
                }
                if board[index] != 0 {
                    skip_x = true;
                }
                x_intercepts.push(x_intercept);
            }

            // if !skip_y {
            //     y_intercept = self.get_y_intercept(block_size, y_intercept);
            //     // let x = (x_intercept.x / block_size).floor() + 1.0;
            //     let x = y_intercept.x / block_size;
            //     // let y = x_intercept.y / block_size;
            //     let y = (y_intercept.y / block_size).floor() + 1.0;
            //     let index = ((x + y * 10.0) - 1.0) as usize;
            //     if index >= 100 {
            //         continue;
            //     }
            //     if board[index] != 0 {
            //         skip_y = true;
            //     }
            //     y_intercepts.push(y_intercept);
            // }
        }
        (x_intercepts, y_intercepts)
    }

    fn draw_intercepts(&self, context: graphics::Context, gl: &mut opengl_graphics::GlGraphics) {
        for &x_intercept in &self.x_intercepts {
            self.draw_intercept(context, gl, x_intercept, colors::RED_ALPHA);
        }

        for &y_intercept in &self.y_intercepts {
            self.draw_intercept(context, gl, y_intercept, colors::BLUE_ALPHA);
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

    fn get_x_intercept(&self, block_size: f64, last_point: Point) -> Point {
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

    fn get_y_intercept(&self, block_size: f64, last_point: Point) -> Point {
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
        point: Point,
        color: [f32; 4],
    ) {
        let xform = context.transform.trans(point.x, point.y).trans(-5.0, -5.0);
        rectangle(color, [0.0, 0.0, 10.0, 10.0], xform, gl);
    }
}
