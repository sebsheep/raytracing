mod geom3d;
use geom3d::{Point, Vect};

extern crate image;

type Color = image::Rgb<u8>;

mod sphere3d {
    use crate::geom3d::{Point, Vect};

    pub struct Sphere3D {
        center: Point,
        color: Vect,
        squared_radius: f64,
    }

    impl Sphere3D {
        pub fn new(center: Point, radius: f64, color: crate::Color) -> Sphere3D {
            Sphere3D {
                center: center,
                squared_radius: radius * radius,
                color: Vect::new(color[0] as f64, color[1] as f64, color[2] as f64),
            }
        }

        pub fn squared_radius(&self) -> f64 {
            self.squared_radius
        }

        pub fn center(&self) -> Point {
            self.center
        }

        pub fn color(&self) -> Vect {
            self.color
        }

        pub fn trace(&self, start: Point, direction: Vect) -> f64 {
            let minus_b_over_2 = direction.dot(start.to(self.center()));
            let c = self.center.to(start).norm2() - self.squared_radius();
            let reduced_delta = minus_b_over_2 * minus_b_over_2 - c;
            if reduced_delta < 0.0 {
                -f64::INFINITY
            } else if reduced_delta > 0.0 {
                let sqrt_reduced_delta = f64::sqrt(reduced_delta);
                let x1 = minus_b_over_2 - sqrt_reduced_delta;
                let x2 = minus_b_over_2 + sqrt_reduced_delta;

                if x1 >= 0.0 {
                    x1
                } else if x2 >= 0.0 {
                    x2
                } else {
                    -f64::INFINITY
                }
            } else {
                f64::max(minus_b_over_2, 0.0)
            }
        }
    }
}

use sphere3d::Sphere3D;

fn screen_to_space(x: u32, y: u32) -> Point {
    Point::new(
        x as f64 - HALF_WIDTH,
        y as f64 - HALF_HEIGHT,
        DISTANCE_TO_SCREEN,
    )
}

fn color(x: u32, y: u32, spheres: &[Sphere3D]) -> Color {
    let direction = Point::ORIGIN.to(screen_to_space(x, y)).unit();
    let mut distances_spheres = spheres
        .into_iter()
        .map(|sphere| (sphere.trace(Point::ORIGIN, direction), sphere))
        .filter(|(dist, _)| dist > &0.0)
        .collect::<Vec<(f64, &Sphere3D)>>();
    distances_spheres.sort_by(|(dist1, _), (dist2, _)| dist1.partial_cmp(dist2).unwrap());

    match distances_spheres.get(0) {
        None => BACKGROUND_COLOR,
        Some((distance, sphere)) => {
            let m = Point::ORIGIN + *distance * direction;
            let ml = m.to(LIGHT_SOURCE);
            let l = ml.unit();
            let n = (sphere.center().to(m)).unit();
            let luminosity = f64::max(l.dot(n), 0.0);
            image::Rgb((luminosity * sphere.color()).as_vec_u8())
        }
    }
}

const WIDTH: u32 = 1920;
const HALF_WIDTH: f64 = WIDTH as f64 / 2.0;
const HEIGHT: u32 = 1080;
const HALF_HEIGHT: f64 = HEIGHT as f64 / 2.0;
const DISTANCE_TO_SCREEN: f64 = 500.0;

const LIGHT_SOURCE: Point = Point::new(5.0, 0.0, 8.0);
const BACKGROUND_COLOR: Color = image::Rgb([50, 50, 50]);

pub fn create_image() -> image::ImageBuffer<Color, std::vec::Vec<u8>> {
    let mut imgbuf = image::ImageBuffer::new(WIDTH, HEIGHT);
    let spheres = [
        Sphere3D::new(Point::new(0.0, 5.0, 10.0), 1.0, image::Rgb([216, 84, 52])),
        Sphere3D::new(Point::new(0.0, 0.0, 10.0), 1.0, image::Rgb([52, 84, 216])),
    ];
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color(x, y, &spheres);
    }
    imgbuf
}
