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

fn use_window(buffer: &mut Vec<u32>)
{
    draw_tools::draw(buffer, 0, 0, 0xFF1199);
    // let radius:isize = 50;


    // draw_tools::draw(buffer, radius, 0, 0xFF1199);
    // draw_tools::draw(buffer, -radius, 0, 0xFF1199);
    // draw_tools::draw(buffer, 0, radius, 0xFF1199);
    // draw_tools::draw(buffer, 0, -radius, 0xFF1199);

    // for i in 0..radius
    // {
    //     draw_tools::draw(buffer, i, radius-i, 0xFF1199);
    //     draw_tools::draw(buffer, -i, -radius+i, 0xFF1199);
    //     draw_tools::draw(buffer, i, -radius+i, 0xFF1199);
    //     draw_tools::draw(buffer, -i, radius-i, 0xFF1199);
    // }

    let point1 = Point::new(100,0);
    let point2 = Point::new(-100,0);
    let point3 = Point::new(0,100);
    let point4 = Point::new(0, -100);

    draw_square(buffer, [point1, point2, point3, point4]);
}

fn draw_square(buffer: &mut Vec<u32>, points: [Point; 4])
{
    for i in points[1].x .. points[0].x
    {
        draw_tools::draw(buffer, i, points[0].y,0xFF1199);
    }
}


struct Point{x: isize, y: isize}


impl Point {
    fn new(x: isize, y:isize) -> Self {
        Self {
            x,
            y
        }
    }
}
