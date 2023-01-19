use std::ops;

const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;

fn main() {
    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    for z in ((-WIDTH / 2)..(WIDTH / 2)) {
        for y in ((-HEIGHT / 2)..(HEIGHT / 2)).rev() {
            let r = Ray {
                dir: Vec3(1000.0, y as f64, z as f64),
                ori: Vec3::default(),
            };

            print!("{} ", ray_color(&r));
        }
        println!()
    }
}

trait Hittable {
    fn hit(&self, r: &Ray) -> f64;
}

impl Hittable for &Sphear {
    fn hit(&self, r: &Ray) -> f64 {
        let oc = &r.ori - &self.ori;
        let a = &r.dir * &r.dir;
        let b = 2.0 * (&oc * &r.dir);
        let c = &oc * &oc - (&self.radius * &self.radius);
        let disc = b * b - 4.0 * (&a * &c);
        if disc < -1.0 {
            -1.0
        } else {
            (-b - disc.sqrt()) / (2.0 * a)
        }
    }
}

impl Hittable for &Plane {
    fn hit(&self, r: &Ray) -> f64 {
        (&self.norm * &r.ori + self.d) / (&r.dir * &self.norm)
    }
}

fn ray_color(ray: &Ray) -> Color {
    let sph = &Sphear {
        ori: Vec3(1400.0, 0.0, 200.0),
        radius: 100.0,
    };

    let plane = &Plane {
        norm: Vec3(0.0, 0.0, 1.0),
        d: (HEIGHT / 2) as f64,
    };

    let light = &sph.ori + &Vec3(0.0, 0.0, -150000.0);

    let x = sph.hit(ray);

    if x > 0.0 {
        let norm = (&sph.ori - &light).unit();
        let ööh = &norm * &ray.dir.unit();
        return &Color(255, 255, 255) * ööh;
    }

    let t = plane.hit(&ray);
    if t < 0.0 {
        // sky
        return Color(255, 255, 255)  ;
    }

    let plane_hit = ray.at(t);

    let mut col = &Color(1, 1, 1) * ( t*20.0);

    if sph.hit(&Ray {
        ori: plane_hit,
        dir: &light - &plane_hit,
    }) > 0.0
    {
        col = &col * 0.5;
    }

    return col;
}

struct Color(u8, u8, u8);

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {} {}", self.0, self.1, self.2))
    }
}

struct Ray {
    ori: Vec3,
    dir: Vec3,
}

struct Plane {
    norm: Vec3,
    d: f64,
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct Vec3(f64, f64, f64);

struct Sphear {
    ori: Vec3,
    radius: f64,
}

impl Vec3 {
    fn unit(&self) -> Vec3 {
        self * (1.0 / self.length())
    }

    fn length(&self) -> f64 {
        (self * self).sqrt()
    }
}

impl Ray {
    fn at(&self, x: f64) -> Vec3 {
        &self.ori + &(&self.dir * x)
    }
}

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

// this is the dot product
impl ops::Mul<&Vec3> for &Vec3 {
    type Output = f64;

    fn mul(self, rhs: &Vec3) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color(
            (self.0 as f64 * rhs) as u8,
            (self.1 as f64 * rhs) as u8,
            (self.2 as f64 * rhs) as u8,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn unit() {
        let x = Vec3(3.0, 4.0, -1.0);
        assert_eq!(1.0, x.unit().length());
        assert_eq!(Vec3(-1.0, 0.0, 0.0), Vec3(-1.0, 0.0, 0.0).unit());
    }
}
