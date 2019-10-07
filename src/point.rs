#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl From<RayPoint> for Point {
    fn from(ray_point: RayPoint) -> Point {
        Point {
            x: ray_point.x,
            y: ray_point.y,
        }
    }
}

impl Point {
    pub fn get_distance(self, other_point: RayPoint) -> f64 {
        let dx = self.x - other_point.x;
        let dy = self.y - other_point.y;
        let d = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
        d
    }
    pub fn new() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y,)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Intercept {
    XIntercept,
    YIntercept,
}

#[derive(Debug, Clone, Copy)]
pub struct RayPoint {
    pub x: f64,
    pub y: f64,
    pub board_index: Option<usize>,
    pub intercept: Intercept,
}

impl RayPoint {
    pub fn get_distance(self, other_point: Point) -> f64 {
        let dx = self.x - other_point.x;
        let dy = self.y - other_point.y;
        let d = (dx.powf(2.0) + dy.powf(2.0)).sqrt();
        d
    }
}
