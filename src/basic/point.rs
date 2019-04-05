use std::ops;


#[derive(Debug)]
pub struct Point
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point
{
    pub fn new(a:f32 ,b:f32 , c:f32 ) -> Point
    {
        Point
        {
            x: a,
            y: b,
            z: c
        }
    }
    
    pub fn dot(&self, p:&Point) -> f32
    {
        self.x * p.x + self.y * p.y + self.z * p.z
    }

    pub fn norm(&self) -> f32
    {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Point
    {
        self * (1.0 / self.norm())
    }

    pub fn distance(&self, other: &Point) -> f32
    {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0) + (self.z - other.z).powf(2.0)).sqrt()
    }
}

impl ops::Add<Point> for Point
{
    type Output = Point;

    fn add(self, other: Point) -> Point
    {
        Point
        {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub<Point> for Point
{
    type Output = Point;

    fn sub(self, other: Point) -> Point
    {
        Point
        {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<Point> for Point
{
    type Output = Point;

    fn mul(self, other: Point) -> Point
    {
        Point
        {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Mul<Point> for f32
{
    type Output = Point;

    fn mul(self, p: Point) -> Point
    {
        Point
            {
                x: p.x * self,
                y: p.x * self,
                z: p.x * self,
            }
    }
}

impl ops::Mul<f32> for &Point
{
    type Output = Point;

    fn mul(self, muu: f32) -> Point
    {
        Point
            {
                x: self.x * muu,
                y: self.y * muu,
                z: self.z * muu,
            }
    }
}

impl ops::Div<Point> for Point
{
    type Output = Point;

    fn div(self, other: Point) -> Point
    {
        Point
        {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}