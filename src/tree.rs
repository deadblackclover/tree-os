use vga::colors::Color16;
use vga::writers::Graphics640x480x16;
use vga_figures::figures2d::Figures2D;

const WIDTH: isize = 640;
const HEIGHT: isize = 480;

pub fn congratulation(figures: Figures2D<Graphics640x480x16>, color: Color16) {
    let x = WIDTH / 2;
    let y = HEIGHT / 2;
    figures.text((x - 40) as usize, y as usize, "Happy New Year!", color);
}
