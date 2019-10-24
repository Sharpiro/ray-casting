use board::Board;
use colors;
use display_vec::DisplayVec;
use graphics::{math::Matrix2d, Transformed};
use point::{BoardPoint, InterceptType, RayPoint};
use sharp_graphics::SharpGraphics;

const DEGREES_90_RADIANS: f64 = 1.5708;

#[derive(Debug, Clone)]
pub struct Ray {
    pub angle: f64,
    pub start_position: BoardPoint,
    pub x_intercepts: DisplayVec<RayPoint>,
    pub y_intercepts: DisplayVec<RayPoint>,
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
            start_position: BoardPoint::new(),
            x_intercepts: DisplayVec::<RayPoint>::new(),
            y_intercepts: DisplayVec::<RayPoint>::new(),
            wall_intersection: None,
            wall_distance: 0.0,
            wall_height: 0.0,
        }
    }
}

impl Ray {
    pub fn update(&mut self, start_position: BoardPoint, angle: f64, board: &Board) {
        // todo: see if 'Into' trait removes 1 clone operation here
        self.start_position = start_position;
        self.angle = angle;
        let (sin, cos) = self.angle.sin_cos();
        self.x_intercepts = self.get_x_intercepts(board, sin, cos);
        self.y_intercepts = self.get_y_intercepts(board, cos);
        let (wall_intersection, wall_distance) = self.get_wall_intersection();
        self.wall_intersection = wall_intersection;
        self.wall_distance = wall_distance;
        self.wall_height = (100.0 / wall_distance) * 100.0;
    }

    pub fn draw(&self, transform: Matrix2d, graphics: &mut SharpGraphics, block_size: f64) {
        self._draw_intercepts(transform, graphics);

        if let Some(point) = self.wall_intersection {
            graphics.draw_line(
                colors::YELLOW,
                [
                    self.start_position.x * block_size,
                    self.start_position.y * block_size,
                    point.x,
                    point.y,
                ],
                transform,
            );
            let color = if point.intercept_type == InterceptType::XIntercept {
                colors::RED_ALPHA
            } else {
                colors::BLUE_ALPHA
            };
            self.draw_intercept(transform, graphics, point, color);
        }
    }

    fn get_wall_intersection(&self) -> (Option<RayPoint>, f64) {
        let x_intersections: Vec<&RayPoint> = self
            .x_intercepts
            .iter()
            .filter(|p| p.has_wall_intersection)
            .collect();
        let y_intersections: Vec<&RayPoint> = self
            .y_intercepts
            .iter()
            .filter(|p| p.has_wall_intersection)
            .collect();
        if x_intersections.len() == 0 && y_intersections.len() == 0 {
            return (None, 0.0);
        }

        // do line length compare
        let player_point = BoardPoint {
            x: self.start_position.x * 50.0,
            y: self.start_position.y * 50.0,
        };

        if x_intersections.len() == 0 {
            let point = **y_intersections.last().unwrap();
            return (Some(point), point.get_distance(player_point));
        }
        if y_intersections.len() == 0 {
            let point = **x_intersections.last().unwrap();
            return (Some(point), point.get_distance(player_point));
        }

        let x_intercept = **x_intersections.last().unwrap();
        let y_intercept = **y_intersections.last().unwrap();
        let x_distance = player_point.get_distance(x_intercept);
        let y_distance = player_point.get_distance(y_intercept);

        if x_distance < y_distance {
            return (Some(x_intercept), x_distance);
        } else {
            return (Some(y_intercept), y_distance);
        }
    }

    fn get_x_intercepts(&self, board: &Board, sin: f64, cos: f64) -> DisplayVec<RayPoint> {
        let x_tan = (DEGREES_90_RADIANS - self.angle).tan();
        let mut x_intercept = self.get_initial_x_intercept(board.block_size, sin, x_tan);
        let mut x_intercepts = DisplayVec::<RayPoint>::new();

        if let Some(board_index) = Ray::get_board_index_x(board, x_intercept, sin, cos) {
            if board.tiles[board_index] != 0 {
                x_intercept.has_wall_intersection = true;
                x_intercept.board_index = Some(board_index);
                x_intercepts.push(x_intercept);
                return x_intercepts;
            } else {
                x_intercept.board_index = Some(board_index);
                x_intercepts.push(x_intercept);
            }
        } else {
            return x_intercepts;
        }

        for _ in 0..board.tiles_y {
            x_intercept = self.get_x_intercept(board.block_size, x_intercept, sin, x_tan);
            if let Some(board_index) = Ray::get_board_index_x(board, x_intercept, sin, cos) {
                if board.tiles[board_index] != 0 {
                    x_intercept.has_wall_intersection = true;
                    x_intercept.board_index = Some(board_index);
                    x_intercepts.push(x_intercept);
                    return x_intercepts;
                } else {
                    x_intercept.board_index = Some(board_index);
                    x_intercepts.push(x_intercept);
                }
            } else {
                return x_intercepts;
            }
        }

        x_intercepts
    }

    fn get_y_intercepts(&self, board: &Board, cos: f64) -> DisplayVec<RayPoint> {
        let y_tan = self.angle.tan();
        let mut y_intercept = self.get_initial_y_intercept(board.block_size, cos, y_tan);
        let mut y_intercepts = DisplayVec::<RayPoint>::new();

        if let Some(board_index) = Ray::get_board_index_y(board, y_intercept, cos) {
            if board.tiles[board_index] != 0 {
                y_intercept.has_wall_intersection = true;
                y_intercept.board_index = Some(board_index);
                y_intercepts.push(y_intercept);
                return y_intercepts;
            } else {
                y_intercept.board_index = Some(board_index);
                y_intercepts.push(y_intercept);
            }
        } else {
            return y_intercepts;
        }

        for _ in 0..board.tiles_x {
            y_intercept = self.get_y_intercept(board.block_size, y_intercept, cos, y_tan);
            if let Some(board_index) = Ray::get_board_index_y(board, y_intercept, cos) {
                if board.tiles[board_index] != 0 {
                    y_intercept.has_wall_intersection = true;
                    y_intercept.board_index = Some(board_index);
                    y_intercepts.push(y_intercept);
                    return y_intercepts;
                } else {
                    y_intercept.board_index = Some(board_index);
                    y_intercepts.push(y_intercept);
                }
            } else {
                return y_intercepts;
            }
        }

        y_intercepts
    }

    fn get_board_index_x(
        board: &Board,
        x_intercept: RayPoint,
        sin: f64,
        cos: f64,
    ) -> Option<usize> {
        if x_intercept.x < 0.0 {
            return None;
        }
        let rounded_x_intercept = x_intercept.x.round();
        if rounded_x_intercept >= board.tiles_x as f64 * board.block_size {
            return None;
        }
        let x_tile_float = rounded_x_intercept / board.block_size;
        let mut x_tile = x_tile_float.floor() as usize;

        // if the rounded intercept is a tile intersection, use cosine to estimate most accurate tile
        // todo: may need to be done with y intercept as well, we'll see
        if x_tile_float % 1.0 == 0.0 && cos < 0.0 {
            if x_tile == 0 {
                return None;
            }
            x_tile -= 1
        }

        let mut y_tile = (x_intercept.y / board.block_size) as usize;
        if sin < 0.0 {
            if y_tile == 0 {
                //todo: check seems unnecessary when inside board since this is for x intercept
                return None;
            }
            y_tile -= 1;
        }
        if y_tile >= 10 {
            //todo: check seems unnecessary when inside board since this is for x intercept
            return None;
        }
        let index = x_tile + y_tile * 10;
        Some(index)
    }

    fn get_board_index_y(board: &Board, y_intercept: RayPoint, cos: f64) -> Option<usize> {
        let mut x_tile = (y_intercept.x / board.block_size) as usize;
        let y_tile = (y_intercept.y.round() / board.block_size).floor() as usize;
        if cos < 0.0 {
            if x_tile == 0 {
                return None;
            }
            x_tile -= 1;
        }
        if x_tile >= board.tiles_x || y_tile >= board.tiles_y {
            return None;
        }

        let index = board.get_index_from_tile(x_tile, y_tile);
        Some(index)
    }

    fn get_initial_x_intercept(&self, block_size: f64, sin: f64, x_tan: f64) -> RayPoint {
        // 1. (floor + 1) vs 2. (ceil)
        // 1 will always cause a jump on x.0 values
        // 2 will not move on x.0 values
        let board_y = if sin > 0.0 {
            self.start_position.y.floor() + 1.0
        } else {
            self.start_position.y.ceil() - 1.0
        };
        let delta_next_y = self.start_position.y - board_y;
        let delta_next_x = x_tan * delta_next_y;
        let board_x = self.start_position.x - delta_next_x;
        let point_x = board_x * block_size;

        let point_y = board_y * block_size;
        RayPoint::new(point_x, point_y, InterceptType::XIntercept)
    }

    fn get_x_intercept(
        &self,
        block_size: f64,
        last_point: RayPoint,
        sin: f64,
        x_tan: f64,
    ) -> RayPoint {
        let x_value = if sin > 0.0 {
            last_point.x + x_tan * block_size
        } else {
            last_point.x - x_tan * block_size
        };
        let y_value = if sin > 0.0 {
            last_point.y + block_size
        } else {
            last_point.y - block_size
        };
        RayPoint::new(x_value, y_value, InterceptType::XIntercept)
    }

    fn get_initial_y_intercept(&self, block_size: f64, cos: f64, y_tan: f64) -> RayPoint {
        let board_x = if cos > 0.0 {
            self.start_position.x.floor() + 1.0
        } else {
            self.start_position.x.ceil() - 1.0
        };
        let delta_next_x = self.start_position.x - board_x;
        let delta_next_y = y_tan * delta_next_x;
        let board_y = self.start_position.y - delta_next_y;
        let point_y = board_y * block_size;

        let point_x = board_x * block_size;
        RayPoint::new(point_x, point_y, InterceptType::YIntercept)
    }

    fn get_y_intercept(
        &self,
        block_size: f64,
        last_point: RayPoint,
        cos: f64,
        y_tan: f64,
    ) -> RayPoint {
        let x_value = if cos > 0.0 {
            last_point.x + block_size
        } else {
            last_point.x - block_size
        };
        let y_value = if cos > 0.0 {
            last_point.y + y_tan * block_size
        } else {
            last_point.y - y_tan * block_size
        };
        RayPoint::new(x_value, y_value, InterceptType::YIntercept)
    }

    fn draw_intercept(
        &self,
        transform: Matrix2d,
        graphics: &mut SharpGraphics,
        point: RayPoint,
        color: [f32; 4],
    ) {
        let xform = transform.trans(point.x, point.y).trans(-5.0, -5.0);
        graphics.draw_rectangle(color, [0.0, 0.0, 10.0, 10.0], xform);
    }

    fn _draw_intercepts(&self, transform: Matrix2d, graphics: &mut SharpGraphics) {
        // for &x_intercept in self.x_intercepts.iter() {
        //     self.draw_intercept(transform, graphics, x_intercept, colors::RED_ALPHA);
        // }

        for &y_intercept in self.y_intercepts.iter() {
            self.draw_intercept(transform, graphics, y_intercept, colors::BLUE_ALPHA);
        }
    }
}
