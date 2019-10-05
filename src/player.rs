use graphics::*;

use colors;

const DEGREES_90_RADIANS: f64 = 1.5708;

#[derive(Debug, Clone, Copy)]
pub struct RayPoint {
    x: f64,
    y: f64,
    board_index: Option<usize>,
}

impl RayPoint {
    fn get_distance(self, other_point: Self) -> f64 {
        let dx = self.x - other_point.x;
        let dy = self.y - other_point.y;
        let d = (dx + dy).abs().sqrt();
        d
    }
}

#[derive(Debug)]
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub rotation_rad: f64,
    pub x_intercepts: Vec<RayPoint>,
    pub y_intercepts: Vec<RayPoint>,
    pub wall_intersection: Option<RayPoint>,
    pub count: u32,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rot_str = &self.rotation_rad.to_string();
        let rot_size = std::cmp::min(rot_str.len(), 7);
        write!(
            f,
            "Player {{ count: {}, rot: {}, wall_x_ion: {:?} xes: {:?}, ys: {:?} }}",
            self.x_intercepts.len() + self.y_intercepts.len(),
            &rot_str[..rot_size],
            self.wall_intersection,
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
        rectangle(
            colors::BLACK,
            [0.0, 0.0, 10.0, 10.0],
            line_rot.trans(-5.0, -5.0),
            gl,
        );

        if let Some(point) = self.wall_intersection {
            line(
                colors::BLACK,
                1.0,
                [self.x * 50.0, self.y * 50.0, point.x, point.y],
                context.transform,
                gl,
            );
            self.draw_intercept(context.transform, gl, point, colors::BLACK);
        }
        // self.draw_intercepts(context.transform, gl);
    }

    pub fn update(&mut self, block_size: f64, board: &Vec<u32>) {
        let (sin, cos) = self.rotation_rad.sin_cos();
        self.x_intercepts = self.get_x_intercepts(block_size, board, sin);
        self.y_intercepts = self.get_y_intercepts(block_size, board, cos);
        self.wall_intersection = self.get_wall_intersection();
    }

    fn get_wall_intersection(&self) -> Option<RayPoint> {
        let x_intersections: Vec<&RayPoint> = self
            .x_intercepts
            .iter()
            .filter(|x| x.board_index.is_some())
            .collect();
        let y_intersections: Vec<&RayPoint> = self
            .y_intercepts
            .iter()
            .filter(|x| x.board_index.is_some())
            .collect();
        if x_intersections.len() == 0 && y_intersections.len() == 0 {
            // panic!("no intersections found")
            return None;
        }
        if x_intersections.len() == 0 {
            return Some(**y_intersections.last().unwrap());
        }
        if y_intersections.len() == 0 {
            return Some(**x_intersections.last().unwrap());
        }

        // do line length compare
        let player_point = RayPoint {
            x: self.x * 50.0,
            y: self.y * 50.0,
            board_index: None,
        };
        let point_1 = **x_intersections.last().unwrap();
        let point_2 = **y_intersections.last().unwrap();
        let p1_distance = player_point.get_distance(point_1);
        let p2_distance = player_point.get_distance(point_2);

        if p1_distance < p2_distance {
            return Some(point_1);
        } else {
            return Some(point_2);
        }
    }

    fn get_x_intercepts(&self, block_size: f64, board: &Vec<u32>, sin: f64) -> Vec<RayPoint> {
        let mut x_intercept = self.get_initial_x_intercept(block_size);
        let mut x_intercepts = vec![];

        if let Some(board_index) = Player::get_board_index_x(block_size, x_intercept, sin) {
            if board[board_index] != 0 {
                x_intercept.board_index = Some(board_index);
                x_intercepts.push(x_intercept);
                return x_intercepts;
            } else {
                x_intercepts.push(x_intercept);
            }
        } else {
            return x_intercepts;
        }

        for _ in 0..10 {
            x_intercept = self.get_x_intercept(block_size, x_intercept);
            if let Some(board_index) = Player::get_board_index_x(block_size, x_intercept, sin) {
                if board[board_index] != 0 {
                    x_intercept.board_index = Some(board_index);
                    x_intercepts.push(x_intercept);
                    return x_intercepts;
                } else {
                    x_intercepts.push(x_intercept);
                }
            } else {
                return x_intercepts;
            }
        }

        x_intercepts
    }

    fn get_y_intercepts(&self, block_size: f64, board: &Vec<u32>, cos: f64) -> Vec<RayPoint> {
        let mut y_intercept = self.get_initial_y_intercept(block_size);
        let mut y_intercepts = vec![];

        if let Some(board_index) = Player::get_board_index_y(block_size, y_intercept, cos) {
            if board[board_index] != 0 {
                y_intercept.board_index = Some(board_index);
                y_intercepts.push(y_intercept);
                return y_intercepts;
            } else {
                y_intercepts.push(y_intercept);
            }
        } else {
            return y_intercepts;
        }

        for _ in 0..10 {
            y_intercept = self.get_y_intercept(block_size, y_intercept);
            if let Some(board_index) = Player::get_board_index_y(block_size, y_intercept, cos) {
                if board[board_index] != 0 {
                    y_intercept.board_index = Some(board_index);
                    y_intercepts.push(y_intercept);
                    return y_intercepts;
                } else {
                    y_intercepts.push(y_intercept);
                }
            } else {
                return y_intercepts;
            }
        }

        y_intercepts
    }

    fn get_board_index_x(block_size: f64, x_intercept: RayPoint, sin: f64) -> Option<usize> {
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

    fn get_board_index_y(block_size: f64, y_intercept: RayPoint, cos: f64) -> Option<usize> {
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

    fn draw_intercepts(
        &self,
        transform: graphics::math::Matrix2d,
        gl: &mut opengl_graphics::GlGraphics,
    ) {
        for &x_intercept in self.x_intercepts.iter()
        // .filter(|point| point.board_index.is_some())
        {
            self.draw_intercept(transform, gl, x_intercept, colors::RED_ALPHA);
        }

        for &y_intercept in self.y_intercepts.iter()
        // .filter(|point| point.board_index.is_some())
        {
            self.draw_intercept(transform, gl, y_intercept, colors::BLUE_ALPHA);
        }
    }

    fn get_initial_x_intercept(&self, block_size: f64) -> RayPoint {
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
        RayPoint {
            x: x_value,
            y: y_value,
            board_index: None,
        }
    }

    fn get_x_intercept(&self, block_size: f64, last_point: RayPoint) -> RayPoint {
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
        RayPoint {
            x: x_value,
            y: y_value,
            board_index: None,
        }
    }

    fn get_initial_y_intercept(&self, block_size: f64) -> RayPoint {
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
        RayPoint {
            x: x_value,
            y: y_value,
            board_index: None,
        }
    }

    fn get_y_intercept(&self, block_size: f64, last_point: RayPoint) -> RayPoint {
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
        RayPoint {
            x: x_value,
            y: y_value,
            board_index: None,
        }
    }

    fn draw_intercept(
        &self,
        transform: graphics::math::Matrix2d,
        gl: &mut opengl_graphics::GlGraphics,
        point: RayPoint,
        color: [f32; 4],
    ) {
        let xform = transform.trans(point.x, point.y).trans(-5.0, -5.0);
        rectangle(color, [0.0, 0.0, 10.0, 10.0], xform, gl);
    }
}
