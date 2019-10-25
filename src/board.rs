use colors;
use graphics::{math::Matrix2d, Transformed};
use maths;
use point::{BoardPoint, ScreenPoint};
use sharp_graphics::SharpGraphics;

pub struct Board {
    pub tiles: Vec<u32>,
    pub tiles_x: usize,
    pub block_size: f64,
    pub tiles_y: usize,
}

impl Board {
    pub fn new(tiles: Vec<u32>, tiles_x: usize, tiles_y: usize, block_size: f64) -> Board {
        Board {
            tiles: tiles,
            tiles_x: tiles_x,
            block_size: block_size,
            tiles_y: tiles_y,
        }
    }

    pub fn point_from(&self, screen_point: ScreenPoint) -> BoardPoint {
        BoardPoint {
            x: screen_point.x / self.block_size,
            y: screen_point.y / self.block_size,
        }
    }

    pub fn get_index(&self, point: BoardPoint) -> usize {
        let board_x = point.x as usize;
        let board_y = point.y as usize;
        self.get_index_from_tile(board_x, board_y)
    }

    pub fn get_index_from_tile(&self, x_tile: usize, y_tile: usize) -> usize {
        y_tile * self.tiles_x + x_tile
    }

    pub fn get_tile(&self, board_index: usize) -> BoardPoint {
        let (y, x) = maths::div_mod(board_index, self.tiles_x);
        BoardPoint {
            x: x as f64,
            y: y as f64,
        }
    }

    pub fn is_wall_at(&self, point: BoardPoint) -> bool {
        let board_index = self.get_index(point);
        self.tiles[board_index] != 0
    }

    pub fn draw(&self, transform: Matrix2d, graphics: &mut SharpGraphics) {
        self.draw_grid(transform, graphics);
        self.draw_walls(transform, graphics);
    }

    fn draw_walls(&self, transform: Matrix2d, graphics: &mut SharpGraphics) {
        for (i, &cell) in (0..).zip(self.tiles.iter()) {
            let color = match cell {
                1 => Some(colors::RED_ALPHA),
                2 => Some(colors::BLUE_ALPHA),
                3 => Some(colors::GREEN_ALPHA),
                4 => Some(colors::ORANGE_ALPHA),
                _ => None,
            };

            if let Some(color) = color {
                let point = self.get_tile(i);
                graphics.draw_rectangle(
                    color,
                    [
                        point.x * self.block_size,
                        point.y * self.block_size,
                        self.block_size,
                        self.block_size,
                    ],
                    transform,
                );
            };
        }
    }

    fn draw_grid(&self, transform: Matrix2d, sharp_graphics_x: &mut SharpGraphics) {
        for i in 1..self.tiles_y {
            let offset = i as f64;

            // draw x axis
            sharp_graphics_x.draw_line(
                colors::BLACK,
                [0.0, 0.0, self.tiles_x as f64 * self.block_size, 0.0],
                transform.trans(0.0, offset * self.block_size),
            );
        }

        for i in 1..self.tiles_x {
            let offset = i as f64;
            // draw y axis
            sharp_graphics_x.draw_line(
                colors::BLACK,
                [0.0, 0.0, 0.0, self.tiles_y as f64 * self.block_size],
                transform.trans(offset * self.block_size, 0.0),
            );
        }
    }
}
