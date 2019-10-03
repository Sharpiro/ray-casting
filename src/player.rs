use graphics::*;

use colors;

const DEGREES_90_RADIANS: f64 = 1.5708;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y,)
    }
}

#[derive(Debug)]
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub rotation_rad: f64,
    pub x_intercepts: Vec<Point>,
    pub y_intercepts: Vec<Point>,
    pub count: u32,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rot_str = &self.rotation_rad.to_string();
        let rot_size = std::cmp::min(rot_str.len(), 7);
        write!(
            f,
            "Player {{ count: {}, rot: {}, xes: {:?}, ys: {:?} }}",
            self.x_intercepts.len() + self.y_intercepts.len(),
            &rot_str[..rot_size],
            self.x_intercepts,
            self.y_intercepts
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
        self.draw_intercepts(context.transform, gl);
    }

    pub fn update(&mut self, block_size: f64, board: &Vec<u32>) {
        let (sin, cos) = self.rotation_rad.sin_cos();
        self.x_intercepts = self.get_x_intercepts(block_size, board, sin);
        self.y_intercepts = self.get_y_intercepts(block_size, board, cos);
    }

    fn get_board_index_x(block_size: f64, x_intercept: Point, sin: f64) -> Option<usize> {
        let x_tile = (x_intercept.x / block_size).floor() as usize;
        let mut y_tile = (x_intercept.y / block_size) as usize;
        if sin < 0.0 {
            if y_tile == 0 {
                return None;
            }
            y_tile -= 1;
        }
        if x_tile >= 10 || y_tile >= 10 {
            return None;
        }
        let index = x_tile + y_tile * 10;
        Some(index)
    }

    fn get_board_index_y(block_size: f64, y_intercept: Point, cos: f64) -> Option<usize> {
        let mut x_tile = (y_intercept.x / block_size) as usize;
        let y_tile = (y_intercept.y / block_size).floor() as usize;
        if cos < 0.0 {
            if x_tile == 0 {
                return None;
            }
            x_tile -= 1;
        }
        if x_tile >= 10 || y_tile >= 10 {
            return None;
        }
        let index = x_tile + y_tile * 10;
        Some(index)
    }

    fn get_x_intercepts(&self, block_size: f64, board: &Vec<u32>, sin: f64) -> Vec<Point> {
        let mut x_intercept = self.get_initial_x_intercept(block_size);
        let mut x_intercepts = vec![];

        if let Some(board_index) = Player::get_board_index_x(block_size, x_intercept, sin) {
            x_intercepts.push(x_intercept);
            if board[board_index] != 0 {
                return x_intercepts;
            }
        }

        for _ in 0..10 {
            x_intercept = self.get_x_intercept(block_size, x_intercept);
            if let Some(board_index) = Player::get_board_index_x(block_size, x_intercept, sin) {
                x_intercepts.push(x_intercept);
                if board[board_index] != 0 {
                    break;
                }
            } else {
                x_intercepts.push(Point {
                    x: 5000.0,
                    y: 5000.0,
                });
            }
        }

        x_intercepts
    }

    fn get_y_intercepts(&self, block_size: f64, board: &Vec<u32>, cos: f64) -> Vec<Point> {
        let mut y_intercept = self.get_initial_y_intercept(block_size);
        let mut y_intercepts = vec![];

        if let Some(board_index) = Player::get_board_index_y(block_size, y_intercept, cos) {
            y_intercepts.push(y_intercept);
            if board[board_index] != 0 {
                return y_intercepts;
            }
        }

        for _ in 0..10 {
            y_intercept = self.get_y_intercept(block_size, y_intercept);
            if let Some(board_index) = Player::get_board_index_y(block_size, y_intercept, cos) {
                y_intercepts.push(y_intercept);
                if board[board_index] != 0 {
                    break;
                }
            } else {
                y_intercepts.push(Point {
                    x: 5000.0,
                    y: 5000.0,
                });
            }
        }

        y_intercepts
    }

    fn draw_intercepts(
        &self,
        transform: graphics::math::Matrix2d,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        for &x_intercept in &self.x_intercepts {
            self.draw_intercept(transform, gl, x_intercept, colors::RED_ALPHA);
        }

        for &y_intercept in &self.y_intercepts {
            self.draw_intercept(transform, gl, y_intercept, colors::BLUE_ALPHA);
        }
    }

    fn get_initial_x_intercept(&self, block_size: f64) -> Point {
        let x_value = if self.rotation_rad.sin() > 0.0 {
            (block_size * self.x) + ((DEGREES_90_RADIANS - self.rotation_rad).tan() * block_size)
        } else {
            (block_size * self.x) - ((DEGREES_90_RADIANS - self.rotation_rad).tan() * block_size)
        };
        let y_value = if self.rotation_rad.sin() > 0.0 {
            block_size * (self.y + 1.0)
        } else {
            block_size * (self.y - 1.0)
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
            block_size * (self.x + 1.0)
        } else {
            block_size * (self.x - 1.0)
        };
        let y_value = if self.rotation_rad.cos() > 0.0 {
            (block_size * self.y) + (self.rotation_rad.tan() * block_size)
        } else {
            (block_size * self.y) - (self.rotation_rad.tan() * block_size)
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
        transform: graphics::math::Matrix2d,
        gl: &mut opengl_graphics::GlGraphics,
        point: Point,
        color: [f32; 4],
    ) {
        let xform = transform.trans(point.x, point.y).trans(-5.0, -5.0);
        rectangle(color, [0.0, 0.0, 10.0, 10.0], xform, gl);
    }
}
