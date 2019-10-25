use board::Board;
use colors;
use graphics::{math::Matrix2d, Transformed};
use player::Player;
use point::BoardPoint;
use sharp_graphics::SharpGraphics;

pub struct MiniMap {}

const LINE_LENGTH: f64 = 100.0;

impl MiniMap {
    pub fn draw(
        &self,
        transform: Matrix2d,
        graphics: &mut SharpGraphics,
        board: &Board,
        player: &Player,
    ) {
        const X_OFFSET: f64 = 800.0;
        const Y_OFFSET: f64 = 600.0;
        const TWO_PI: f64 = std::f64::consts::PI * 2.0;
        const BUMP: f64 = TWO_PI / 3600.0;

        let origin = transform.trans(X_OFFSET, Y_OFFSET);
        graphics.draw_rectangle(
            colors::BLACK,
            [0.0, 0.0, 10.0, 10.0],
            origin.rot_rad(player.angle).trans(-5.0, -5.0),
        );
        graphics.draw_line(
            colors::YELLOW,
            [0.0, 0.0, 150.0, 0.0],
            origin.rot_rad(player.angle),
        );

        let mut angle = 0.0;
        while angle < TWO_PI {
            let (sin, cos) = angle.sin_cos();
            let x = LINE_LENGTH * cos;
            let y = LINE_LENGTH * sin;
            graphics.draw_line(colors::BLACK, [x, y, x + 1.0, y + 1.0], origin);
            angle += BUMP;
        }

        self.draw_x_axis(origin, graphics, board, player);
        self.draw_y_axis(origin, graphics, board, player);
        self.draw_walls(origin, graphics, board, player);
    }

    fn draw_x_axis(
        &self,
        transform: Matrix2d,
        graphics: &mut SharpGraphics,
        board: &Board,
        player: &Player,
    ) {
        graphics.draw_line(
            colors::BLACK,
            [0.0, 0.0, LINE_LENGTH * 2.0, 0.0],
            transform.trans(-LINE_LENGTH, (player.position.y % 1.0) * -board.block_size),
        );

        graphics.draw_line(
            colors::BLACK,
            [0.0, 0.0, LINE_LENGTH * 2.0, 0.0],
            transform
                .trans(-LINE_LENGTH, (player.position.y % 1.0) * -board.block_size)
                .trans(0.0, board.block_size),
        );

        graphics.draw_line(
            colors::BLACK,
            [0.0, 0.0, LINE_LENGTH * 2.0, 0.0],
            transform
                .trans(-LINE_LENGTH, (player.position.y % 1.0) * -board.block_size)
                .trans(0.0, -board.block_size),
        );

        graphics.draw_line(
            colors::BLACK,
            [0.0, 0.0, LINE_LENGTH * 2.0, 0.0],
            transform
                .trans(-LINE_LENGTH, (player.position.y % 1.0) * -board.block_size)
                .trans(0.0, board.block_size * -2.0),
        );

        graphics.draw_line(
            colors::BLACK,
            [0.0, 0.0, LINE_LENGTH * 2.0, 0.0],
            transform
                .trans(-LINE_LENGTH, (player.position.y % 1.0) * -board.block_size)
                .trans(0.0, board.block_size * 2.0),
        );
    }

    fn draw_y_axis(
        &self,
        transform: Matrix2d,
        graphics: &mut SharpGraphics,
        board: &Board,
        player: &Player,
    ) {
        graphics.draw_line(
            colors::BLACK,
            [0.0, 0.0, 0.0, LINE_LENGTH * 2.0],
            transform.trans((player.position.x % 1.0) * -board.block_size, -LINE_LENGTH),
        );

        graphics.draw_line(
            colors::BLACK,
            [0.0, 0.0, 0.0, LINE_LENGTH * 2.0],
            transform
                .trans((player.position.x % 1.0) * -board.block_size, -LINE_LENGTH)
                .trans(board.block_size, 0.0),
        );

        graphics.draw_line(
            colors::BLACK,
            [0.0, 0.0, 0.0, LINE_LENGTH * 2.0],
            transform
                .trans((player.position.x % 1.0) * -board.block_size, -LINE_LENGTH)
                .trans(-board.block_size, 0.0),
        );

        graphics.draw_line(
            colors::BLACK,
            [0.0, 0.0, 0.0, LINE_LENGTH * 2.0],
            transform
                .trans((player.position.x % 1.0) * -board.block_size, -LINE_LENGTH)
                .trans(-board.block_size * 2.0, 0.0),
        );

        graphics.draw_line(
            colors::BLACK,
            [0.0, 0.0, 0.0, LINE_LENGTH * 2.0],
            transform
                .trans((player.position.x % 1.0) * -board.block_size, -LINE_LENGTH)
                .trans(board.block_size * 2.0, 0.0),
        );
    }

    fn draw_walls(
        &self,
        transform: Matrix2d,
        graphics: &mut SharpGraphics,
        board: &Board,
        player: &Player,
    ) {
        let _temp = (player.position.y % 1.0) * board.block_size;
        // top
        let point = BoardPoint {
            x: player.position.x,
            y: player.position.y - 1.0,
        };
        let index = board.get_index(point);
        let color_option = match board.tiles[index] {
            1 => Some(colors::RED_ALPHA),
            2 => Some(colors::BLUE_ALPHA),
            3 => Some(colors::GREEN_ALPHA),
            4 => Some(colors::ORANGE_ALPHA),
            _ => None,
        };

        if let Some(color) = color_option {
            graphics.draw_rectangle(
                color,
                [0.0, 0.0, board.block_size, board.block_size],
                transform.trans(0.0, -board.block_size - _temp),
            );
        };

        let point = BoardPoint {
            x: player.position.x + 1.0,
            y: player.position.y - 1.0,
        };

        let index = board.get_index(point);
        let color_option = match board.tiles[index] {
            1 => Some(colors::RED_ALPHA),
            2 => Some(colors::BLUE_ALPHA),
            3 => Some(colors::GREEN_ALPHA),
            4 => Some(colors::ORANGE_ALPHA),
            _ => None,
        };

        if let Some(color) = color_option {
            graphics.draw_rectangle(
                color,
                [0.0, 0.0, board.block_size, board.block_size],
                transform.trans(board.block_size, -board.block_size),
            );
        };

        // bottom
        let point = BoardPoint {
            x: player.position.x,
            y: player.position.y + 1.0,
        };
        let index = board.get_index(point);
        let color_option = match board.tiles[index] {
            1 => Some(colors::RED_ALPHA),
            2 => Some(colors::BLUE_ALPHA),
            3 => Some(colors::GREEN_ALPHA),
            4 => Some(colors::ORANGE_ALPHA),
            _ => None,
        };

        if let Some(color) = color_option {
            graphics.draw_rectangle(
                color,
                [0.0, 0.0, board.block_size, board.block_size],
                transform,
            );
        };

        let point = BoardPoint {
            x: player.position.x + 1.0,
            y: player.position.y + 1.0,
        };

        let index = board.get_index(point);
        let color_option = match board.tiles[index] {
            1 => Some(colors::RED_ALPHA),
            2 => Some(colors::BLUE_ALPHA),
            3 => Some(colors::GREEN_ALPHA),
            4 => Some(colors::ORANGE_ALPHA),
            _ => None,
        };

        if let Some(color) = color_option {
            graphics.draw_rectangle(
                color,
                [0.0, 0.0, board.block_size, board.block_size],
                transform.trans(board.block_size, 0.0),
            );
        };
    }
}
