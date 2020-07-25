mod vec3d;
use vec3d::Vec3d;

extern crate image;

const WIDTH:u32 = 1920;
const HALF_WIDTH:f64 = WIDTH as f64/ 2.0;
const HEIGHT:u32 = 1080;
const HALF_HEIGHT:f64 = HEIGHT as f64/ 2.0;
const D:f64 = 500.0;

const CENTER:Vec3d = Vec3d::new(0.0, 0.0, 10.0);
const LIGHT_DIRECTION:Vec3d = Vec3d::new(-20.0, -25.0, -15.0);
const SPHERE_COLOR:Vec3d = Vec3d::new(52.0, 84.0, 216.0); 
const RADIUS:f64 = 1.0;
const SQUARED_RADIUS:f64 = RADIUS * RADIUS;
const BACKGROUND_COLOR:image::Rgb<u8> = image::Rgb([50, 50, 50]);

fn intersection(v: Vec3d, center_norm2:f64, r2:f64) -> f64 {
    let minus_b_over_2 = v.dot(CENTER);
    let c = center_norm2 - r2;
    let reduced_delta = minus_b_over_2 * minus_b_over_2  -  c;
    if reduced_delta < 0.0 {
        0.0
    } else if reduced_delta > 0.0 {
        let sqrt_reduced_delta = f64::sqrt(reduced_delta);
        let x1 = minus_b_over_2 - sqrt_reduced_delta;
        let x2 = minus_b_over_2 + sqrt_reduced_delta;

        if x1 >= 0.0 {
            x1
        } else if x2 >= 0.0 {
            x2
        } else {
            0.0
        }
    } else {
        f64::max(minus_b_over_2, 0.0)
    }
}

fn direction(x: u32, y: u32) -> Vec3d {
    Vec3d::new(x as f64 - HALF_WIDTH, y as f64 - HALF_HEIGHT, D)
}

fn color(x: u32, y: u32, center_norm2:f64)  -> image::Rgb<u8> {
    let unit = direction(x, y).unit();
    let inter = intersection(unit, center_norm2, SQUARED_RADIUS);
    if inter > 0.0 {
        let om = inter * unit;
        let ml = LIGHT_DIRECTION - om;
        let l = ml.unit();
        let n = (om - CENTER).unit();
        image::Rgb((f64::max(l.dot(n), 0.0) * SPHERE_COLOR  ).as_vec_u8())
    } else {
        BACKGROUND_COLOR
    } 
}

pub fn create_image() -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);
    let center_norm2 = CENTER.norm2();
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color(x, y, center_norm2);
    }  
    
    imgbuf
}