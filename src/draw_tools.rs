extern crate minifb;
use minifb::{Window, WindowOptions};

pub struct DrawTools {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub window: Window,
}

impl DrawTools {
    pub fn new(width: usize, height: usize) -> Self {
        let draw_tools = Self {
            width,
            height,
            buffer: vec![0; width * height],
            window: Self::get_window(width, height),
        };
        draw_tools
    }

    fn get_window(width: usize, height: usize) -> Window {
        let mut window = Window::new(
            "Draw Pixel - Press ESC to exit",
            width,
            height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limit the update rate to 1 fps
        window.limit_update_rate(Some(std::time::Duration::from_micros(1000000)));
        return window;
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    pub fn draw(&mut self, x: isize, y: isize, color: u32) {
        // Optimize ?
        let converted_width = self.width as isize / 2;
        let converted_height = self.height as isize / 2;

        if x > converted_width
            || x < -converted_width
            || y > converted_height
            || y < -converted_height
        {
            panic!(
                "Invalid coordinates x: {}, y: {}, while width: {} height: {}",
                x, y, self.width, self.height
            );
        }

        let converted_x: usize = self.width / 2 + x as usize;
        let converted_y: usize = self.height / 2 - y as usize;
        let index: usize = converted_y * self.width + converted_x;
        self.buffer[index] = color;
    }
}



pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point {
    pub const fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

fn color_from_rgb(r: u8, g: u8, b: u8) -> u32 {
    let min = 0;
    let max = 255;

    let red: u32 = (r.clamp(min, max) * 16 * 16 * 16).into();
    let green: u32 = (g.clamp(min, max) * 16 * 16).into();
    let blue: u32 = (b.clamp(min, max) * 16).into();

    let value: u32 = red + green + blue;
    value
}

fn color_from_rgb_vec(color: Vec<u8>) -> u32 {
    let r = color[0];
    let g = color[1];
    let b = color[2];

    return color_from_rgb(r, g, b);
}
