extern crate minifb;
use minifb::{Window, WindowOptions};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const WIDTH_I: isize = 1920;
const HEIGHT_I: isize = 1080;

pub fn get_window() -> Window {
    let mut window = Window::new(
        "Draw Pixel - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit the update rate to 1 fps
    window.limit_update_rate(Some(std::time::Duration::from_micros(1000000)));

    // draw_pixel(&mut buffer, 0, 0, 0xFF1199);
    // draw_pixel(&mut buffer, 1, 1, 0xFF1199);
    // draw_pixel(&mut buffer, -WIDTH_I/2+50, -HEIGHT_I/2+50, 0xFF1199);
    
    //window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    return window;
    // while window.is_open() && !window.is_key_down(Key::Escape) {
    //     window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    // }

}

pub fn get_buffer() -> Vec<u32>  {
    vec![0; WIDTH * HEIGHT]
}

pub fn update(window: &mut Window, buffer: &mut Vec<u32>) {
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
}


pub fn draw(buffer: &mut Vec<u32>, x: isize, y: isize, color: u32) {
    if (x < WIDTH_I / 2 || x > -WIDTH_I / 2) && (y < HEIGHT_I / 2 || y > -HEIGHT_I / 2) {
        let converted_x = WIDTH_I / 2 + x;
        let converted_y = HEIGHT_I / 2 - y;
        let index = converted_y * WIDTH_I + converted_x;
        buffer[usize::try_from(index).expect("error")] = color;
    }
}

// struct Color(i32, i32, i32);

// impl Color {
    
// }