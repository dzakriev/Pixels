mod draw_tools;
use minifb::Key;

fn main() {
    let mut buffer = draw_tools::get_buffer();
    let mut window = draw_tools::get_window();

    window.limit_update_rate(Some(std::time::Duration::from_secs(1)));

    use_window(&mut buffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_tools::update(&mut window, &mut buffer);
    }
}

fn use_window(buffer: &mut Vec<u32>) {
    draw_tools::draw(buffer, 0, 0, 0xFF1199);

    let point1 = Point::new(100, 0);
    let point2 = Point::new(-100, 0);
    let point3 = Point::new(0, 100);
    let point4 = Point::new(0, -100);

    let color = color_from_rgb(255, 0, 0);

    draw_square(buffer, [point1, point2, point3, point4]);
}

fn draw_square(buffer: &mut Vec<u32>, points: [Point; 4]) {
    for i in points[1].x..points[0].x {
        draw_tools::draw(buffer, i, points[0].y, 0xFF1199);
    }
}

struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
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
