use super::point::Point;

pub struct Ray
{
    pub origin: Point,
    pub destination: Point,
}

impl Ray
{
    pub fn new(o:Point, d:Point) -> Ray
    {
        Ray
        {
            origin: o,
            destination: d,
        }
    } 

    pub fn length(&self) -> f32
    {
        self.origin.distance(&self.destination)
    } 
}