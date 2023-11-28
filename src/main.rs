pub mod vec3;
pub mod color;
pub mod ray;
use std::io::Write;
use std::io::stderr;

use color::Color;
use vec3::Vec3;
use vec3::Point3;

use ray::Ray;

use crate::color::write_color;

fn main() {
    
    // Image
    const ASPECT_RATIO : f64 = 16.0 / 9.0;
    const IMAGE_WIDTH : u64 = 400;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new_with_values(0.0, 0.0, 0.0);
    let horizontal = Vec3::new_with_values(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new_with_values(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - Point3::new_with_values(0.0, 0.0, focal_length);
    
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {            
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(origin,
                             lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);
            let _ = write_color(&mut std::io::stdout(), &pixel_color);
        }
        eprint!("\rDone");
    }
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Vec3::new_with_values(0.0, 0.0, -1.0), 0.5, r) {
        Vec3::new_with_values(1.0, 0.0, 0.0)
    }
    else {
        let unit_direction = r.dir().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new_with_values(1.0, 1.0, 1.0) + a * Color::new_with_values(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.orig() - *center;
    let a = ray.dir().dot(&ray.dir());
    let b = 2.0 * oc.dot(&ray.dir());
    let c = oc.dot(&oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;
    discriminant > 0.0
}

    