use std::f32::consts::PI;

use crate::{
    renderer::{self, RGBA}, ColoredTriangle, HEIGHT, WIDTH
};

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
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn dot(&self, b: &Vector2) -> f32 {
        self.x * b.x + self.y * b.y
    }

    pub fn perpendicular_clockwise(&self) -> Vector2 {
        Vector2 { x: self.y, y: -self.x }
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

#[derive(Debug)]
pub struct Triangle2 {
    pub a: Vector2,
    pub b: Vector2,
    pub c: Vector2,
    perpendicular_ab: Vector2,
    perpendicular_bc: Vector2,
    perpendicular_ca: Vector2,
}

impl Triangle2 {
    pub fn new(a: Vector2, b: Vector2, c: Vector2) -> Self {
        let ab = &b - &a;
        let bc = &c - &b;
        let ca = &a - &c;
        let perpendicular_ab = ab.perpendicular_clockwise();
        let perpendicular_bc = bc.perpendicular_clockwise();
        let perpendicular_ca = ca.perpendicular_clockwise();

        Self {
            a,
            b,
            c,
            perpendicular_ab,
            perpendicular_bc,
            perpendicular_ca,
        }
    }

    pub fn point_in_triangle(&self, p: &Vector2) -> bool {
        let ap = p - &self.a;
        let bp = p - &self.b;
        let cp = p - &self.c;
        let dot_abp = self.perpendicular_ab.dot(&ap) >= 0.0;
        let dot_bcp = self.perpendicular_bc.dot(&bp) >= 0.0;
        let dot_cap = self.perpendicular_ca.dot(&cp) >= 0.0;

        dot_cap && dot_bcp && dot_abp
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
        let apply_position = {
            let position = &transform.position;
            move |triangle: &mut ColoredTriangle| {
                triangle.triangle.a = triangle.triangle.a.clone() + position.clone();
                triangle.triangle.b = triangle.triangle.b.clone() + position.clone();
                triangle.triangle.c = triangle.triangle.c.clone() + position.clone();
            }
        };

        let apply_direction = {
            let direction = &transform.direction;
            move |triangle: &mut ColoredTriangle| {
                let (i, j, k) = get_quaternion(direction.x, direction.y);
                triangle.triangle.a = triangle.triangle.a.transform(&i, &j, &k);
                triangle.triangle.b = triangle.triangle.b.transform(&i, &j, &k);
                triangle.triangle.c = triangle.triangle.c.transform(&i, &j, &k);
            }
        };

        self.0.iter_mut().for_each(|t| {
            apply_direction(t);
            apply_position(t);
        })
    }
}

pub fn get_quaternion(yaw: f32, pitch: f32) -> (Vector3, Vector3, Vector3) {
    let i_yaw = Vector3 {x: yaw.cos(), y: 0.0, z: yaw.sin()};
    let j_yaw = Vector3 {x: 0.0, y: 1.0, z: 0.0};
    let k_yaw = Vector3 {x: -yaw.sin(), y: 0.0, z: yaw.cos()};

    let i_pitch = Vector3 {x: 1.0, y: 0.0, z: 0.0};
    let j_pitch = Vector3 {x: 0.0, y: pitch.cos(), z: -pitch.sin()};
    let k_pitch = Vector3 {x: 0.0, y: pitch.sin(), z: pitch.cos()};

    let i = i_pitch.transform(&i_yaw, &j_yaw, &k_yaw);
    let j = j_pitch.transform(&i_yaw, &j_yaw, &k_yaw);
    let k = k_pitch.transform(&i_yaw, &j_yaw, &k_yaw);

    (i, j, k)
}

pub fn fill_triangle_buffer(buffer: &mut [RGBA], triangle: &Triangle2, color: renderer::RGBA) {
    let min_x = triangle.a.x.min(triangle.b.x).min(triangle.c.x);
    let min_y = triangle.a.y.min(triangle.b.y).min(triangle.c.y);
    let max_x = triangle.a.x.max(triangle.b.x).max(triangle.c.x);
    let max_y = triangle.a.y.max(triangle.b.y).max(triangle.c.y);

    let min_height = min_y as usize;
    let min_width = min_x as usize;
    let max_height = max_y.ceil() as usize;
    let max_width = max_x.ceil() as usize;

    for y in min_height.max(0)..max_height.min(HEIGHT) {
        for x in min_width.max(0)..max_width.min(WIDTH) {
            let p = Vector2 {
                x: x as f32,
                y: y as f32,
            };
            if triangle.point_in_triangle(&p) {
                let index = y * WIDTH + x;
                buffer[index] = color.clone();
            }
        }
    }
}


fn world_to_screen(point: &Vector3, fov: f32) -> Vector2 {
    if fov <= 0.0 {
        panic!("FOV must be greater than 0");
    }
    // TODO: Below should not be computed for every point
    let randiant_fov = fov * PI / 180.0;
    let world_unit = f32::tan(randiant_fov / 2.0) * 2.0;
    let mut pixels_per_world_unit = HEIGHT as f32 / world_unit;
    if point.z < 0.0 {
        pixels_per_world_unit /= point.z;
    }
    let offset = Vector2 {
        x: point.x * pixels_per_world_unit, y: point.y * pixels_per_world_unit,
    };
    Vector2 {
        x: HEIGHT as f32 / 2.0 + offset.x,
        y: HEIGHT as f32 / 2.0 - offset.y,
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
        let triangle = Triangle2::new(a, b, c);
        fill_triangle_buffer(pixels_buffer, &triangle, i.color.clone());
    }
}
