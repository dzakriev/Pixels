mod draw_tools;
mod math_tools;

use std::isize;

use draw_tools::{DrawTools, Point};
use math_tools::dot_product;
use minifb::Key;

const CANVAS_WIDTH: usize = 800;
const CANVAS_HEIGHT: usize = 800;
const VIEWPORT_WIDTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 1.0;
const CAMERA_DISTANCE: f64 = 1.0;
const CAMERA_POSITION: Point = Point::new(0., 0., 0.);
const SPHERES: [Sphere; 3] = [
    Sphere::new(Point::new(0., -1., 4.), 1.0, 0xFF0000),
    Sphere::new(Point::new(2., 0., 4.), 1.0, 0x00FF00),
    Sphere::new(Point::new(-2., 0., 4.), 1.0, 0x0000FF),
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
            let view_port_point: Point = canvas_to_view_port(x, y);
            let color: u32 = trace_ray(CAMERA_POSITION, view_port_point);
            tools.draw(x, y, color);
        }
    }

    tools.draw(0, 0, 0xFF1199);
}

fn canvas_to_view_port(x: isize, y: isize) -> Point {
    Point {
        x: x as f64 * (VIEWPORT_WIDTH / CANVAS_WIDTH as f64),
        y: y as f64 * (VIEWPORT_HEIGHT / CANVAS_HEIGHT as f64),
        z: CAMERA_DISTANCE,
    }
}

fn trace_ray(camera_position: Point, point: Point) -> u32 {
    let t_min: f64 = 0.0;
    let t_max: f64 = f64::MAX;
    let default_color: u32 = 0xFFFFFF;

    let mut closest_sphere_id: Option<usize> = None;
    let mut closest_t: f64 = f64::MAX;

    for i in 0..SPHERES.len() {
        let intersection = intersect_ray_sphere(&camera_position, &point, &SPHERES[i]);

        match intersection {
            Some(value) => {
                if value.0 > t_min && value.0 < t_max && value.0 < closest_t {
                    closest_t = value.0;
                    closest_sphere_id = Some(i);
                }
                if value.1 > t_min && value.1 < t_max && value.1 < closest_t {
                    closest_t = value.1;
                    closest_sphere_id =Some(i);
                }
            },
            _ => ()
        }
    }

    match closest_sphere_id {
        Some(i) => SPHERES[i].color,
        _ => default_color,
    }
}

fn intersect_ray_sphere(camera_position: &Point, D: &Point, sphere: &Sphere) -> Option<(f64, f64)> {
    let r = sphere.radius;
    let CO = camera_position.subtract_rework(&sphere.center);

    let a = dot_product(&D, &D);

    let b = dot_product(&CO, &D) * 2.0;
    let c = dot_product(&CO, &CO) - r * r;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return None;
    }

    let intersection1 = (-b - discriminant.sqrt()) / 2.0 * a;
    let intersection2 = (-b + discriminant.sqrt()) / 2.0 * a;

    return Some((intersection1, intersection2));
}

struct Sphere {
    pub center: Point,
    pub color: u32,
    pub radius: f64,
}

impl Sphere {
    pub const fn new(center: Point, radius: f64, color: u32) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }
    pub const fn new_from_coordinates(
        x: f64,
        y: f64,
        z: f64,
        radius: f64,
        color: u32,
    ) -> Self {
        Self {
            center: Point { x, y, z },
            color,
            radius,
        }
    }
}
