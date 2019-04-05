use super::ray::Ray;
use super::color::Color;

pub trait Intersectable
{
    fn intersect(&self, ray: &Ray) -> Option<Color>;
}