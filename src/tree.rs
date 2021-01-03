use vga::colors::Color16;
use vga::writers::Graphics640x480x16;
use vga_figures::figures2d::Figures2D;

pub const WIDTH: isize = 640;
pub const HEIGHT: isize = 480;

pub fn happy_new_year(figures: &Figures2D<Graphics640x480x16>, color: Color16, x: usize, y: usize) {
    figures.text(x - 55, y, "Happy New Year!", color);
}

pub fn tree(figures: &Figures2D<Graphics640x480x16>, color: Color16, x: isize, y: isize) {
    let arr = [
        x,
        y,
        (x - 50),
        (y + 50),
        x,
        (y + 50),
        (x - 100),
        (y + 100),
        x,
        (y + 100),
        (x - 150),
        (y + 150),
        x,
        (y + 150),
        x,
        (y + 200),
        x,
        (y + 150),
        (x + 150),
        (y + 150),
        x,
        (y + 100),
        (x + 100),
        (y + 100),
        x,
        (y + 50),
        (x + 50),
        (y + 50),
        x,
        y,
    ];

    figures.polygon(&arr, color);
}
