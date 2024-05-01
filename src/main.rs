mod maths;
mod ray;
mod sphere;

use image::{RgbImage, Rgb};
use maths::*;
use sphere::*;
use ray::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 600;
const FOV: f64 = 90.0;
const PI:  f64 = std::f64::consts::PI;

fn main() {
    let mut image = RgbImage::new(WIDTH, HEIGHT);
    let ray_scalar: f64 = ((90.0 - FOV / 2.0) * PI / 180.0).cos();
    let aspect_ratio: f64 = HEIGHT as f64 / WIDTH as f64;

    let sphere = Sphere{position: Vector::new(vec![0.0, 0.0, 3.0, 0.0]), radius: 1.0};

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = Ray {
                origin: Vector::new(vec![0.0, 0.0, 0.0, 0.0]),
                direction: Vector::new(vec![
                    ((x as f64 / WIDTH as f64) * 2.0 - 1.0) * ray_scalar,
                    ((y as f64 / HEIGHT as f64) * 2.0 - 1.0) * aspect_ratio * ray_scalar,
                    1.0,
                    0.0
                ])
            };

            let intersection = sphere.intersect(ray);
            if intersection.is_some() && (intersection.unwrap().0 > 0.0 || intersection.unwrap().1 > 0.0){
                image.put_pixel(x, y, Rgb([255, 255, 255]));
                continue;
            }
            image.put_pixel(x, y, Rgb([50, 50, 50]));
        }
    }

    image.save("result.png").unwrap();
}
