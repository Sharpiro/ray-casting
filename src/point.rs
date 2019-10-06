#[derive(Debug, Clone, Copy)]
pub struct RayPoint {
    pub x: f64,
    pub y: f64,
    pub board_index: Option<usize>,
}

impl RayPoint {
    pub fn get_distance(self, other_point: Self) -> f64 {
        let dx = self.x - other_point.x;
        let dy = self.y - other_point.y;
        let d = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
        d
    }
}

impl RayPoint {
    pub fn new() -> RayPoint {
        RayPoint {
            x: 0.0,
            y: 0.0,
            board_index: None,
        }
    }
}
