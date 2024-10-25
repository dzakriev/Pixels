use crate::draw_tools::Point;

pub fn dot_product(point1: &Point, point2: &Point) -> f64
{
    point1.iter().zip(point2.iter()).map(|(value1 , value2)| value1 * value2).sum()
}