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

    pub fn draw_direct(&mut self, x: usize, y: usize, color: u32)
    {
        let index: usize = y * self.width + x;
        self.buffer[index] = color;
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

        let converted_x: usize = (converted_width + x) as usize;
        let converted_y: usize = (converted_height - y) as usize;
        let index: usize = converted_y * self.width + converted_x;
        self.buffer[index] = color;
    }
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Point {
    pub fn subtract_rework(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }

    pub const fn new(x: isize, y: isize, z: isize) -> Self {
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
    type Item = isize;

    fn next(&mut self) -> Option<isize> {
        self.index+=1;
        print!("{}", self.index);
        match self.index {
            1 => {Some(self.point.x)} 
            2 => {Some(self.point.y)} 
            3 => {Some(self.point.z)} 
            _ => Some(0)
        }
    }
}


impl Point {
    // pub fn iter(&self) -> PointIterator {
    //     PointIterator {
    //         point: self,
    //         index: 0,
    //     }
    // }
    pub fn iter(&self) -> IntoIter<isize> {
        let i: Vec<isize> = [self.x, self.y, self.z].to_vec();
        i.into_iter()
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
