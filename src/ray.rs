use colors;
use graphics::*;
use point::RayPoint;

const DEGREES_90_RADIANS: f64 = 1.5708;

#[derive(Debug, Clone)]
pub struct Ray {
    pub angle: f64,
    pub start_position: RayPoint, // probably needs to be owned by player
    pub x_intercepts: Vec<RayPoint>,
    pub y_intercepts: Vec<RayPoint>,
    pub wall_intersection: Option<RayPoint>,
    pub wall_distance: f64,
    pub wall_height: f64,
}

impl std::fmt::Display for Ray {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Ray {{ dist: {}, height: {}, wall_x_ion: {:?}, intercepts: {}",
            self.wall_distance,
            self.wall_height,
            self.wall_intersection,
            self.x_intercepts.len() + self.y_intercepts.len(),
        )
    }
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            angle: 0.0,
            start_position: RayPoint::new(),
            x_intercepts: vec![],
            y_intercepts: vec![],
            wall_intersection: None,
            wall_distance: 0.0,
            wall_height: 0.0,
        }
    }
}

impl Ray {
    pub fn update(&mut self, block_size: f64, board: &Vec<u32>) {
        let (sin, cos) = self.angle.sin_cos();
        self.x_intercepts = self.get_x_intercepts(block_size, board, sin);
        self.y_intercepts = self.get_y_intercepts(block_size, board, cos);
        let (wall_intersection, wall_distance) = self.get_wall_intersection();
        self.wall_intersection = wall_intersection;
        self.wall_distance = wall_distance;
        self.wall_height = (100.0 / wall_distance) * 200.0;
    }

    pub fn draw(
        &self,
        context: graphics::Context,
        gl: &mut opengl_graphics::GlGraphics,
        block_size: f64,
    ) {
        if let Some(point) = self.wall_intersection {
            line(
                colors::BLACK,
                1.0,
                [
                    self.start_position.x * block_size,
                    self.start_position.y * block_size,
                    point.x,
                    point.y,
                ],
                context.transform,
                gl,
            );
            self.draw_intercept(context.transform, gl, point, colors::BLACK);
        }

        // self.draw_intercepts(context.transform, gl);
    }

    fn get_wall_intersection(&self) -> (Option<RayPoint>, f64) {
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
            return (None, 0.0);
        }

        // do line length compare
        let player_point = RayPoint {
            x: self.start_position.x * 50.0,
            y: self.start_position.y * 50.0,
            board_index: None,
        };

        if x_intersections.len() == 0 {
            let point = **y_intersections.last().unwrap();
            return (Some(point), point.get_distance(player_point));
        }
        if y_intersections.len() == 0 {
            let point = **x_intersections.last().unwrap();
            return (Some(point), point.get_distance(player_point));
        }

        let point_1 = **x_intersections.last().unwrap();
        let point_2 = **y_intersections.last().unwrap();
        let p1_distance = player_point.get_distance(point_1);
        let p2_distance = player_point.get_distance(point_2);

        if p1_distance < p2_distance {
            return (Some(point_1), p1_distance);
        } else {
            return (Some(point_2), p2_distance);
        }
    }

    fn get_x_intercepts(&self, block_size: f64, board: &Vec<u32>, sin: f64) -> Vec<RayPoint> {
        let mut x_intercept = self.get_initial_x_intercept(block_size);
        let mut x_intercepts = vec![];

        if let Some(board_index) = Ray::get_board_index_x(block_size, x_intercept, sin) {
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
            if let Some(board_index) = Ray::get_board_index_x(block_size, x_intercept, sin) {
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

        if let Some(board_index) = Ray::get_board_index_y(block_size, y_intercept, cos) {
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
            if let Some(board_index) = Ray::get_board_index_y(block_size, y_intercept, cos) {
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

    fn get_initial_x_intercept(&self, block_size: f64) -> RayPoint {
        let x_value = if self.angle.sin() > 0.0 {
            (block_size * self.start_position.x)
                + ((DEGREES_90_RADIANS - self.angle).tan() * block_size)
        } else {
            (block_size * self.start_position.x)
                - ((DEGREES_90_RADIANS - self.angle).tan() * block_size)
        };
        let y_value = if self.angle.sin() > 0.0 {
            block_size * (self.start_position.y + 1.0)
        } else {
            block_size * (self.start_position.y - 1.0)
        };
        RayPoint {
            x: x_value,
            y: y_value,
            board_index: None,
        }
    }

    fn get_x_intercept(&self, block_size: f64, last_point: RayPoint) -> RayPoint {
        let x_value = if self.angle.sin() > 0.0 {
            last_point.x + (DEGREES_90_RADIANS - self.angle).tan() * block_size
        } else {
            last_point.x - (DEGREES_90_RADIANS - self.angle).tan() * block_size
        };
        let y_value = if self.angle.sin() > 0.0 {
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
        let x_value = if self.angle.cos() > 0.0 {
            block_size * (self.start_position.x + 1.0)
        } else {
            block_size * (self.start_position.x - 1.0)
        };
        let y_value = if self.angle.cos() > 0.0 {
            (block_size * self.start_position.y) + (self.angle.tan() * block_size)
        } else {
            (block_size * self.start_position.y) - (self.angle.tan() * block_size)
        };
        RayPoint {
            x: x_value,
            y: y_value,
            board_index: None,
        }
    }

    fn get_y_intercept(&self, block_size: f64, last_point: RayPoint) -> RayPoint {
        let x_value = if self.angle.cos() > 0.0 {
            last_point.x + block_size
        } else {
            last_point.x - block_size
        };
        let y_value = if self.angle.cos() > 0.0 {
            last_point.y + self.angle.tan() * block_size
        } else {
            last_point.y - self.angle.tan() * block_size
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

    // fn draw_intercepts(
    //     &self,
    //     transform: graphics::math::Matrix2d,
    //     gl: &mut opengl_graphics::GlGraphics,
    // ) {
    //     for &x_intercept in self.x_intercepts.iter()
    //     // .filter(|point| point.board_index.is_some())
    //     {
    //         self.draw_intercept(transform, gl, x_intercept, colors::RED_ALPHA);
    //     }

    //     for &y_intercept in self.y_intercepts.iter()
    //     // .filter(|point| point.board_index.is_some())
    //     {
    //         self.draw_intercept(transform, gl, y_intercept, colors::BLUE_ALPHA);
    //     }
    // }
}
