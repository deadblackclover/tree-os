#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tree_os::serial_print;
use tree_os::tree::{happy_new_year, toy, tree, HEIGHT, WIDTH};

use vga::colors::Color16;
use vga::writers::{Graphics640x480x16, GraphicsWriter};
use vga_figures::figures2d::Figures2D;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mode = Graphics640x480x16::new();
    mode.set_mode();
    mode.clear_screen(Color16::Black);

    let figures = Figures2D::new(mode);

    let center_x = WIDTH / 2;
    let center_y = HEIGHT / 2;

    happy_new_year(&figures, center_x as usize, 10, Color16::White);
    tree(&figures, center_x, center_y - 100);

    toy(&figures, center_x - 50, center_y - 40, Color16::Red);
    toy(&figures, center_x + 50, center_y - 40, Color16::Blue);
    toy(&figures, center_x - 75, center_y + 10, Color16::Yellow);
    toy(&figures, center_x + 75, center_y + 10, Color16::Cyan);

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_print!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    testing::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
