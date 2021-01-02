use vga::colors::Color16;
use vga::writers::Graphics640x480x16;
use vga_figures::figures2d::Figures2D;

pub const WIDTH: isize = 640;
pub const HEIGHT: isize = 480;

pub fn happy_new_year(figures: Figures2D<Graphics640x480x16>, color: Color16, x: usize, y: usize) {
    figures.text(x - 55, y, "Happy New Year!", color);
}
