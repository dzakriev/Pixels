mod draw_tools;
mod math_tools;

use draw_tools::{DrawTools, Point};
use math_tools::dot_product;
use minifb::Key;

const CANVAS_WIDTH: usize = 500;
const CANVAS_HEIGHT: usize = 500;
const VIEWPORT_WIDTH: usize = 1000;
const VIEWPORT_HEIGHT: usize = 10;
const CAMERA_DISTANCE: i32 = 10;
const CAMERA_POSITION: Point = Point::new(0, 0, 0);
const SPHERES: [Sphere; 3] = [
    Sphere::new(Point::new(10, 10, 0), 5, 0xFF0000),
    Sphere::new(Point::new(-10, -10, 0), 5, 0x00FF00),
    Sphere::new(Point::new(10, 0, 0), 5, 0x0000FF),
];

fn main() {
    let mut tools = DrawTools::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    use_window(&mut tools);

    while tools.window.is_open() && !tools.window.is_key_down(Key::Escape) {
        tools.update();
    }
}

fn use_window(tools: &mut DrawTools) {
    let width = (tools.width) as isize;
    let height = (tools.height) as isize;
    for x in -width / 2..width / 2 {
        for y in -height / 2 + 1..height / 2 + 1 {
            println!("{} {}", x, y);
            let view_port_point: Point = canvas_to_view_port(x, y);
            println!("{}, {}", x, y);
            let color: u32 = trace_ray(CAMERA_POSITION, view_port_point);
            tools.draw(x, y, color);
        }
    }

    tools.draw(0, 0, 0xFF1199);
    tools.draw_direct(0, 0, 0xFF1199);
    tools.draw_direct(1, 0, 0xFF1199);
    tools.draw_direct(2, 0, 0xFF1199);
    tools.draw_direct(3, 0, 0xFF1199);
    tools.draw_direct(4, 0, 0xFF1199);
    tools.draw(width / 2, height / 2, 0xFF1199);
    tools.draw(width / 2 - 1, height / 2, 0xFF1199);
    tools.draw(width / 2 - 2, height / 2, 0xFF1199);
}

fn canvas_to_view_port(x: isize, y: isize) -> Point {
    Point { x, y, z: 0 }
}

fn trace_ray(camera_position: Point, point: Point) -> u32 {
    let t_min = 0;
    let t_max = u32::MAX;
    let default_color: u32 = 0xFFFFFF;

    let mut closest_sphere: Option<Sphere> = None;

    for sphere in SPHERES {
        let intersection = intersect_ray_sphere(&camera_position, &point, &sphere);
        if intersection.is_some() {
            closest_sphere = Some(sphere);
        }
    }

    match closest_sphere {
        Some(s) => s.color,
        _ => default_color,
    }
}

fn intersect_ray_sphere(camera_position: &Point, D: &Point, sphere: &Sphere) -> Option<Point>
{
    let r = sphere.radius as isize;
    let CO = camera_position.subtract_rework(&sphere.center);

    let a = dot_product(&D, &D);

    let b = 2 * dot_product(&CO, &D);
    let c = dot_product(&CO, &CO) * r * r;

    let discriminant = b*b - 4*a*c;

    if discriminant < 0
    {
        return None
    }

    let intersection1 = (- b - (discriminant as f64).sqrt() as isize) / 2* a;
    let intersection2 = (- b + (discriminant as f64).sqrt() as isize) / 2* a;

    return Some(Point::new(intersection1, intersection2, 0));
}

struct Sphere {
    pub center: Point,
    pub color: u32,
    pub radius: u32,
}

impl Sphere {
    pub const fn new(center: Point, radius: u32, color: u32) -> Self {
        Self {
            center,
            color,
            radius,
        }
    }
    pub const fn new_from_coordinates(
        x: isize,
        y: isize,
        z: isize,
        radius: u32,
        color: u32,
    ) -> Self {
        Self {
            center: Point { x, y, z },
            color,
            radius,
        }
    }
}

struct Intersections(Option<isize>, Option<isize>);
