use colors;
use graphics::*;
use point::RayPoint;
use ray::Ray;

#[derive(Debug)]
pub struct Player {
    pub position: RayPoint,
    pub rotation_rad: f64,
    pub rays: Vec<Ray>,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rot_str = &self.rotation_rad.to_string();
        let rot_size = std::cmp::min(rot_str.len(), 7);
        write!(
            f,
            "Player {{ x: {}, y: {}, rot: {} }}",
            self.position.x,
            self.position.y,
            &rot_str[..rot_size],
        )
    }
}

impl Player {
    pub fn update(&mut self, block_size: f64, board: &Vec<u32>) {
        for (i, ray) in self.rays.iter_mut().enumerate() {
            ray.start_position = self.position;
            ray.angle = self.rotation_rad + (std::f64::consts::PI / (i + 2) as f64);
            ray.update(block_size, board);
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
            .rot_rad(self.rotation_rad);
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
