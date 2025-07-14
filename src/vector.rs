use std::f32::consts::PI;

use crate::{ColoredTriangle, HEIGHT, WIDTH, renderer::RGBA};

#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn transform(&self, i: &Vector3, j: &Vector3, k: &Vector3) -> Vector3 {
        i.clone() * self.x + j.clone() * self.y + k.clone() * self.z
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f32) -> Self::Output {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    pub fn dot(&self, b: &Vector2) -> i32 {
        self.x * b.x + self.y * b.y
    }

    pub fn perpendicular_clockwise(&self) -> Vector2 {
        Vector2 {
            x: self.y,
            y: -self.x,
        }
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn sub(self, other: &Vector2) -> Self::Output {
        Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone)]
pub struct Triangle3 {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
}

pub struct Transform {
    pub position: Vector3,
    pub direction: Vector3,
}

pub struct Model(pub Vec<ColoredTriangle>);

impl Model {
    pub fn apply_transform(&mut self, transform: &Transform) {
        let position = &transform.position;
        let (i, j, k) = get_quaternion(transform.direction.x, transform.direction.y);

        self.0.iter_mut().for_each(|triangle| {
            triangle.triangle.a = triangle.triangle.a.transform(&i, &j, &k) + position.clone();
            triangle.triangle.b = triangle.triangle.b.transform(&i, &j, &k) + position.clone();
            triangle.triangle.c = triangle.triangle.c.transform(&i, &j, &k) + position.clone();
        })
    }
}

pub fn get_quaternion(yaw: f32, pitch: f32) -> (Vector3, Vector3, Vector3) {
    let i_yaw = Vector3 {
        x: yaw.cos(),
        y: 0.0,
        z: yaw.sin(),
    };
    let j_yaw = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let k_yaw = Vector3 {
        x: -yaw.sin(),
        y: 0.0,
        z: yaw.cos(),
    };

    let i_pitch = Vector3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    let j_pitch = Vector3 {
        x: 0.0,
        y: pitch.cos(),
        z: -pitch.sin(),
    };
    let k_pitch = Vector3 {
        x: 0.0,
        y: pitch.sin(),
        z: pitch.cos(),
    };

    let i = i_pitch.transform(&i_yaw, &j_yaw, &k_yaw);
    let j = j_pitch.transform(&i_yaw, &j_yaw, &k_yaw);
    let k = k_pitch.transform(&i_yaw, &j_yaw, &k_yaw);

    (i, j, k)
}

fn world_to_screen(point: &Vector3, fov: f32) -> Vector2 {
    if fov <= 0.0 {
        panic!("FOV must be greater than 0");
    }
    let randiant_fov = fov * PI / 180.0;
    let world_unit = f32::tan(randiant_fov / 2.0) * 2.0;
    let mut pixels_per_world_unit = HEIGHT as f32 / world_unit;
    if point.z < 0.0 {
        pixels_per_world_unit /= point.z;
    }
    let x_offset = point.x * pixels_per_world_unit;
    let y_offset = point.y * pixels_per_world_unit;
    Vector2 {
        x: HEIGHT as i32 / 2 + (x_offset as i32),
        y: HEIGHT as i32 / 2 - (y_offset as i32),
    }
}

pub fn draw_triangles(pixels_buffer: &mut [RGBA], triangles: &[ColoredTriangle], fov: f32) {
    for i in triangles {
        // Skip triangles that are behind the camera
        if (i.triangle.a.z >= 0.0) || (i.triangle.b.z >= 0.0) || (i.triangle.c.z >= 0.0) {
            continue;
        }
        let a = world_to_screen(&i.triangle.a, fov);
        let b = world_to_screen(&i.triangle.b, fov);
        let c = world_to_screen(&i.triangle.c, fov);

        // Compute bounding box once for the triangle
        let min_x = a.x.min(b.x).min(c.x).max(0);
        let min_y = a.y.min(b.y).min(c.y).max(0);
        let max_x = a.x.max(b.x).max(c.x).min(WIDTH as i32);
        let max_y = a.y.max(b.y).max(c.y).min(HEIGHT as i32);

        // Precompute edge functions for barycentric test
        let ab = &b - &a;
        let bc = &c - &b;
        let ca = &a - &c;
        let perp_ab = ab.perpendicular_clockwise();
        let perp_bc = bc.perpendicular_clockwise();
        let perp_ca = ca.perpendicular_clockwise();

        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = Vector2 { x, y };
                let ap = &p - &a;
                let bp = &p - &b;
                let cp = &p - &c;
                let inside =
                    perp_ab.dot(&ap) >= 0 && perp_bc.dot(&bp) >= 0 && perp_ca.dot(&cp) >= 0;
                if inside {
                    let index = y * WIDTH as i32 + x;
                    pixels_buffer[index as usize] = i.color.clone();
                }
            }
        }
    }
}
