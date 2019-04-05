use super::point::Point;
use super::ray::Ray;
use super::intersect::Intersectable;
use super::color::Color;

pub struct Sphere
{
    center: Point,
    radius: f32,
    color: Color
}

impl Sphere
{
    pub fn new(p:Point, r:f32, c:Color) -> Sphere
    {
        Sphere
        {
            center: p,
            radius: r,
            color: c,
        }
    }
}

impl Intersectable for Sphere
{
    fn intersect(&self, ray: &Ray) -> Option<Color> {
        for i in 0..ray.length() as u32
        {
            let p = Point::new(ray.origin.x, ray.origin.y, i as f32);
            if p.distance(&self.center) <= self.radius
            {
                return Some(self.color.clone());
            }
        }
        None
    }
}