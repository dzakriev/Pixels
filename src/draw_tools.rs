extern crate minifb;
use minifb::{Window, WindowOptions};
use std::ops::Sub;
use std::vec::IntoIter;

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
            "Press ESC to exit",
            width,
            height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.limit_update_rate(Some(std::time::Duration::from_secs(1/60)));
        return window;
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    pub fn draw_direct(&mut self, x: usize, y: usize, color: u32)
    {
        let index: usize = y * self.width + x;
        self.buffer[index] = color;
    }

    pub fn draw(&mut self, x: isize, y: isize, color: u32) {
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

        let converted_x: usize = (converted_width + x) as usize;
        let converted_y: usize = (converted_height - y) as usize;
        let index: usize = converted_y * self.width + converted_x;
        self.buffer[index] = color;
    }
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn subtract_rework(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

pub struct PointIterator<'a>
{
    point: &'a Point,
    index: usize
}

impl<'a> Iterator for PointIterator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        self.index+=1;
        print!("{}", self.index);
        match self.index {
            1 => {Some(self.point.x)} 
            2 => {Some(self.point.y)} 
            3 => {Some(self.point.z)} 
            _ => Some(0.0)
        }
    }
}

impl Point {
    pub fn iter(&self) -> IntoIter<f64> {
        let i: Vec<f64> = [self.x, self.y, self.z].to_vec();
        i.into_iter()
    }

}

fn color_from_rgb(r: u32, g: u32, b: u32) -> u32 {
    let min:u32 = 0;
    let max: u32 = 255;

    let red: u32 = r.clamp(min, max) * 16 * 16 * 16;
    let green: u32 = g.clamp(min, max) * 16 * 16;
    let blue: u32 = b.clamp(min, max) * 16;

    red + green + blue
}

fn color_from_rgb_vec(color: Vec<u32>) -> u32 {
    return color_from_rgb(color[0], color[1], color[2]);
}
