use crate::draw_tools::Point;

pub fn dot_product(point1: &Point, point2: &Point) -> isize
{
    let a = point1.iter().zip(point2.iter()).map(|(value1 , value2)| value1 * value2).sum();

    //println!("Dot product of ({},{}) with ({},{}) is {}", point1.x, point1.y, point2.x, point2.y, a);
    a
}