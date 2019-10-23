use graphics;
use graphics::{math::Matrix2d, Context};
use opengl_graphics::{GlGraphics, GlyphCache};

pub struct SharpGraphics {
    gl: GlGraphics,
    glyphs: GlyphCache<'static>,
}

impl SharpGraphics {
    pub fn new<'a>(gl: GlGraphics, glyphs: GlyphCache<'static>) -> SharpGraphics {
        SharpGraphics {
            gl: gl,
            glyphs: glyphs,
        }
    }

    pub fn clear(&mut self, color: [f32; 4]) {
        graphics::clear(color, &mut self.gl);
    }
    /// [x1, y1, x2, y2]
    pub fn draw_line(&mut self, color: [f32; 4], line: [f64; 4], transform: Matrix2d) {
        graphics::line(color, 1.0, line, transform, &mut self.gl);
    }

    /// Rectangle dimensions: [x, y, w, h]
    pub fn draw_rectangle(&mut self, color: [f32; 4], rect: [f64; 4], transform: Matrix2d) {
        graphics::rectangle(color, rect, transform, &mut self.gl);
    }

    pub fn draw_text(
        &mut self,
        color: [f32; 4],
        font_size: u32,
        text_slice: &str,
        location: Matrix2d,
    ) {
        graphics::text(
            color,
            font_size,
            text_slice,
            &mut self.glyphs,
            location,
            &mut self.gl,
        )
        .expect("write text failure");
    }

    pub fn draw<F, U>(&mut self, _viewport: graphics::Viewport, _f: F) -> U
    where
        F: FnOnce(Context, &mut Self) -> U,
    {
        let context = self.draw_begin(_viewport);
        let res = _f(context, self);
        self.draw_end();
        res
    }

    pub fn draw_begin(&mut self, viewport: graphics::Viewport) -> Context {
        self.gl.draw_begin(viewport)
    }

    pub fn draw_end(&mut self) {
        self.gl.draw_end();
    }
}
