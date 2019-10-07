use colors;
use graphics::*;
use point::Point;
use ray::Ray;

#[derive(Debug)]
pub struct Player {
    pub position: Point,
    pub angle: f64,
    pub angle_tick: f64,
    pub rays: Vec<Ray>,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // let rot_str = &self.angle.to_string();
        // let rot_size = std::cmp::min(rot_str.len(), 7);
        write!(
            f,
            "Player {{ pos: {}, rot: {} }}",
            self.position,
            // &rot_str[..],
            self.angle
        )
    }
}

impl Player {
    pub fn update(&mut self, block_size: f64, board: &Vec<u32>) {
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
            ray.update(self.position, ray_angle, block_size, board);
        }
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
            .trans(block_size * self.position.x, block_size * self.position.y)
            .rot_rad(self.angle);
        line(
            colors::BLUE_ALPHA,
            1.0,
            [0.0, 0.0, block_size * 5.0, 0.0],
            line_rot,
            gl,
        );
        rectangle(
            colors::BLACK,
            [0.0, 0.0, 10.0, 10.0],
            line_rot.trans(-5.0, -5.0),
            gl,
        );

        // draw rays
        for ray in self.rays.iter() {
            ray.draw(context, gl, block_size);
        }
    }
}
