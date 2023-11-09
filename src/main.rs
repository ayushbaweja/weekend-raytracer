pub mod vec3;
pub mod color;
use color::Color;
use color::write_color;

fn main() {
    
    // Image
    let image_width = 256;
    let image_height = 256;
    
    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height { // bottom to top
        //println!("Scanlines remaining: {}", (image_height - j));
        for i in 0..image_width { // left to right
            let pixel_color = Color::new_with_values(
                (i as f64 / (image_width - 1) as f64) as f64,
                (j as f64 / (image_height - 1) as f64) as f64,
                0.0,
            );
            let _ = write_color(&mut std::io::stdout(), &pixel_color);
        }
    }
    //println!("Done!")
}
