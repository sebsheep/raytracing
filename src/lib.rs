mod geom3d;
use geom3d::{Point, Unit, Vect};

extern crate image;

type Color = image::Rgb<u8>;

mod sphere3d {
    use crate::geom3d::{Point, Unit, Vect};

    pub struct Sphere3D {
        pub id: u32,
        center: Point,
        color: Vect,
        squared_radius: f64,
    }

    impl Sphere3D {
        pub fn new(id: u32, center: Point, radius: f64, color: crate::Color) -> Sphere3D {
            Sphere3D {
                id: id,
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

        pub fn trace(&self, start: Point, direction: Unit) -> f64 {
            let minus_b_over_2 = direction.to_vect().dot(start.to(self.center()));
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

        pub fn normal_vector(&self, point: Point) -> Unit {
            self.center().to(point).unit()
        }
    }
}

fn bounce(ray: Unit, normal: Unit) -> Unit {
    /* Building an orthonormal (e1, e2, e3) basis */
    let e1 = normal;
    let e2 = e1.cross(ray);
    let e3 = e1.cross(e2);

    geom3d::x_reflexion(ray, (e1, e2, e3))
}

use sphere3d::Sphere3D;

fn screen_to_space(x: u32, y: u32) -> Point {
    Point::new(
        x as f64 - HALF_WIDTH,
        y as f64 - HALF_HEIGHT,
        DISTANCE_TO_SCREEN,
    )
}

use std::ptr;

fn first_collision<'a>(
    start: Point,
    direction: Unit,
    maybe_last_sphere: Option<&'a Sphere3D>,
    objects: &'a [Sphere3D],
) -> Option<(f64, &'a Sphere3D)> {
    let mut distances_spheres = objects
        .into_iter()
        .map(|sphere| (sphere.trace(start, direction), sphere))
        .filter(|(dist, sphere)| match maybe_last_sphere {
            None => dist > &0.0,
            Some(last_sphere) => dist > &0.0 && sphere.id != last_sphere.id,
        })
        .collect::<Vec<(f64, &'a Sphere3D)>>();
    distances_spheres.sort_by(|(dist1, _), (dist2, _)| dist1.partial_cmp(dist2).unwrap());

    distances_spheres.get(0).map(|x| *x)
}

fn color(observer: Point, screen_point: Point, spheres: &[Sphere3D]) -> Color {
    let mut current_point = observer;
    let mut current_direction = observer.to(screen_point).unit();
    let mut current_color = None;
    let mut last_sphere = None;
    let mut reflexion_count = 0;

    //println!("{:?}", current_point);
    while let Some((distance, sphere)) =
        first_collision(current_point, current_direction, last_sphere, spheres)
    {
        // print!(
        //     "*******\npoint:{:?}\ndirection:{:?}\ndistance:{}\n",
        //     current_point, current_direction, distance
        // );
        current_point = current_point + distance * current_direction.to_vect();
        //println!("{}, {:?}, {:?}", distance, current_point, current_direction);
        current_direction = bounce(current_direction, sphere.normal_vector(current_point));
        last_sphere = Some(sphere);
        reflexion_count += 1;
        let ml = current_point.to(LIGHT_SOURCE);
        let l = ml.unit();
        let n = (sphere.center().to(current_point)).unit();
        let luminosity = f64::max(l.dot(n), 0.0);
        let sphere_color = luminosity * sphere.color();
        current_color = Some(add_color(sphere_color, current_color));

        if reflexion_count > 3 {
            //break;
        }
    }
    current_color
        .map(|v| image::Rgb(v.as_vec_u8()))
        .unwrap_or(BACKGROUND_COLOR)
}

fn add_color(color: Vect, current_color: Option<Vect>) -> Vect {
    match current_color {
        None => color,
        Some(current) => 0.5 * current + 0.5 * color,
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
        Sphere3D::new(
            1,
            Point::new(0.0, 5.0, 10.0),
            1.0,
            image::Rgb([216, 84, 52]),
        ),
        Sphere3D::new(
            2,
            Point::new(0.0, 0.0, 10.0),
            1.0,
            image::Rgb([52, 84, 216]),
        ),
        Sphere3D::new(
            3,
            Point::new(5.0, -5.0, 15.0),
            5.0,
            image::Rgb([84, 216, 52]),
        ),
    ];
    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = color(Point::ORIGIN, screen_to_space(x, y), &spheres);
    }
    //color(Point::ORIGIN, screen_to_space(961, 537), &spheres);
    imgbuf
}

#[cfg(test)]
mod tests {
    use crate::geom3d::*;

    #[test]
    fn bounce_test() {
        let ray = Vect::new(1.0, 0.0, -1.0).unit();
        let normal = Vect::new(0.0, 0.0, 1.0).unit();
        let z_ref = crate::bounce(ray, normal);
        assert_eq!(z_ref, Vect::new(1.0, 0.0, 1.0).unit());
    }
}
