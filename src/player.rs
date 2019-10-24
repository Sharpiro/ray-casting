use board::Board;
use colors;
use graphics::{math::Matrix2d, Transformed};
use point::BoardPoint;
use point::InterceptType;
use ray::Ray;
use sharp_graphics::SharpGraphics;

#[derive(Debug)]
pub struct Player {
    pub position: BoardPoint,
    pub angle: f64,
    pub angle_tick: f64,
    pub rays: Vec<Ray>,
    pub move_step: f64,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _temp = vec![1].iter().filter(|_| true).collect::<Vec<&i32>>().len();
        write!(
            f,
            "Player {{ pos: {}, xes: {:?}, yes: {:?}, rot: {}}}",
            self.position,
            self.rays
                .iter()
                .filter(|r| {
                    match r.wall_intersection {
                        Some(ray_point) => ray_point.intercept_type == InterceptType::XIntercept,
                        None => false,
                    }
                })
                .collect::<Vec<&Ray>>()
                .len(),
            self.rays
                .iter()
                .filter(|r| {
                    match r.wall_intersection {
                        Some(ray_point) => ray_point.intercept_type == InterceptType::YIntercept,
                        None => false,
                    }
                })
                .collect::<Vec<&Ray>>()
                .len(),
            self.angle
        )
    }
}

impl Player {
    pub fn update(&mut self, board: &Board) {
        let fov = std::f64::consts::PI / 2.0;
        let ray_angle_tick = if self.rays.len() > 1 {
            fov / (self.rays.len() - 1) as f64
        } else {
            0.0
        };
        let start_rotation = if self.rays.len() > 1 {
            self.angle - fov / 2.0
        } else {
            self.angle
        };
        for (i, ray) in self.rays.iter_mut().enumerate() {
            let ray_angle = start_rotation + ray_angle_tick * i as f64;
            ray.update(self.position, ray_angle, board);
        }
    }

    pub fn draw(&self, transform: Matrix2d, graphics: &mut SharpGraphics, board: &Board) {
        const PLAYER_RECT_WIDTH: f64 = 10.0;
        const PLAYER_RECT_WIDTH_HALF: f64 = PLAYER_RECT_WIDTH / -2.0;

        let line_rot = transform
            .trans(
                board.block_size * self.position.x,
                board.block_size * self.position.y,
            )
            .rot_rad(self.angle);
        graphics.draw_line(
            colors::BLUE_ALPHA,
            [0.0, 0.0, board.block_size * board.tiles_x as f64, 0.0],
            line_rot,
        );
        graphics.draw_rectangle(
            colors::BLACK,
            [0.0, 0.0, PLAYER_RECT_WIDTH, PLAYER_RECT_WIDTH],
            line_rot.trans(PLAYER_RECT_WIDTH_HALF, PLAYER_RECT_WIDTH_HALF),
        );

        for ray in self.rays.iter() {
            ray.draw(transform, graphics, board.block_size);
        }
    }

    pub fn move_forward(&mut self, board: &Board) {
        self.move_angle(self.angle, board);
    }

    pub fn move_backward(&mut self, board: &Board) {
        let angle = self.angle + std::f64::consts::PI;
        self.move_angle(angle, board);
    }

    pub fn strafe_left(&mut self, board: &Board) {
        let perpendicular_angle = self.angle - std::f64::consts::FRAC_PI_2;
        self.move_angle(perpendicular_angle, board);
    }

    pub fn strafe_right(&mut self, board: &Board) {
        let perpendicular_angle = self.angle + std::f64::consts::FRAC_PI_2;
        self.move_angle(perpendicular_angle, board);
    }

    fn move_angle(&mut self, angle: f64, board: &Board) {
        let (sin, cos) = angle.sin_cos();
        let new_point = BoardPoint {
            x: self.position.x + cos * self.move_step,
            y: self.position.y + sin * self.move_step,
        };
        if board.is_wall_at(new_point) {
            return;
        }
        self.position.x = new_point.x;
        self.position.y = new_point.y;
    }
}
