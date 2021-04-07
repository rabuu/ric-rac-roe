use piston_window::{Context, G2d, rectangle, types::Color};

pub fn draw_rect(col: Color, x: u32, y: u32, width: u32, height: u32, con: &Context, g: &mut G2d) {
    rectangle(
        col,
        [
            x as f64, y as f64,
            width as f64, height as f64,
        ],
        con.transform,
        g,
    )
}
