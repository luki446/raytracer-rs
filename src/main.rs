extern crate image;

use image::ImageBuffer;

mod basic;

use basic::color::Color;
use basic::point::Point;
use basic::sphere::Sphere;
use basic::ray::Ray;
use basic::intersect::Intersectable;

fn main() {
    const WIDTH:u32 = 880;
    const HEIGHT:u32 = 880; 

    let mut imgbuf = ImageBuffer::new(WIDTH, HEIGHT);

    let mut vec = Vec::<Sphere>::new();
    vec.push(Sphere::new(Point::new(250.0, 250.0, 50.0), 30.0, Color::new(0, 255, 0)));
    vec.push(Sphere::new(Point::new(400.0, 600.0, 25.0), 50.0, Color::new(255, 0, 0)));

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let ray = Ray::new(Point::new(x as f32, y as f32, 0.0), Point::new(x as f32, y as f32, 100.0));
        for o in vec.iter()
        {
            let color = o.intersect(&ray);
            match color {
                Some(col) => 
                {
                    *pixel = image::Rgb([col.r, col.g, col.b]);
                },
                None => (),
            }
        }
        //*pixel = image::Rgb([r, 0, b]);
    }

    imgbuf.save("output.png").expect("Cannot save to file");
}
