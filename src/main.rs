mod draw_tools;
use draw_tools::{DrawTools, Point};
use minifb::Key;

const CANVAS_WIDTH: usize = 500;
const CANVAS_HEIGHT: usize = 500;
const VIEWPORT_WIDTH: usize = 1000;
const VIEWPORT_HEIGHT: usize = 10;
const CAMERA_DISTANCE: i32 = 10;
const CAMERA_POSITION: Point = Point::new(0, 0, 0);

fn main() {
    let mut tools = DrawTools::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    use_window(&mut tools);

    while tools.window.is_open() && !tools.window.is_key_down(Key::Escape) {
        tools.update();
    }
}

fn use_window(tools: &mut DrawTools) {
    let width = tools.width as isize;
    let height = tools.height as isize;
    for x in width / 2..width / 2 {
        for y in height / 2..height / 2 {
            let view_port_point: Point = canvas_to_view_port(x, y);
            let color: u32 = trace_ray(CAMERA_POSITION, view_port_point);
            tools.draw(x, y, color);
        }
    }

    tools.draw(0, 0, 0xFF1199);
}

fn canvas_to_view_port(x: isize, y: isize) -> Point {
    Point { x, y, z: 0 }
}

fn trace_ray(camera_position: Point, point: Point) -> u32 {
    let t_min = 0;
    let t_max = u32::MAX;

    let spheres: [Sphere; 3] = [
        Sphere::new(10, 10, 0, 0xFF0000),
        Sphere::new(-10, -10, 0, 0x00FF00),
        Sphere::new(5, 5, 5, 0x0000FF),
    ];

    let closest_sphere: Option<Sphere> = None;

    return 0;
}

struct Sphere {
    pub center: Point,
    pub color: u32,
}

impl Sphere {
    pub const fn new(x: isize, y: isize, z: isize, color: u32) -> Self {
        Self {
            center: Point { x, y, z },
            color,
        }
    }
}
