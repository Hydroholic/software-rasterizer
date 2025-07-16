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

fn dot3(v1: &Vector3, v2: &Vector3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
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

struct Vector2WithDepth {
    v: Vector2,
    depth: f32,
}

struct PixelWithDepth {
    color: RGBA,
    depth: f32,
}

fn signed_triangle_area(a: &Vector2, b: &Vector2, c: &Vector2) -> f32 {
    let ac = c - a;
    let ab_perp = (b - a).perpendicular_clockwise();
    ac.dot(&ab_perp) as f32 / 2.0
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

pub struct Model {
    pub triangles: Vec<ColoredTriangle>,
    pub transform: Transform,
}

impl From<Vec<ColoredTriangle>> for Model {
    fn from(triangles: Vec<ColoredTriangle>) -> Self {
        Model {
            triangles,
            transform: Transform {
                position: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                direction: Vector3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
        }
    }
}

impl Model {
    pub fn apply_transform(&self) -> Vec<ColoredTriangle> {
        let (i, j, k) = get_quaternion(self.transform.direction.x, self.transform.direction.y);

        self.triangles
            .iter()
            .map(|triangle| ColoredTriangle {
                triangle: Triangle3 {
                    a: triangle.triangle.a.transform(&i, &j, &k) + self.transform.position.clone(),
                    b: triangle.triangle.b.transform(&i, &j, &k) + self.transform.position.clone(),
                    c: triangle.triangle.c.transform(&i, &j, &k) + self.transform.position.clone(),
                },
                color: triangle.color,
            })
            .collect()
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

fn world_to_screen(point: &Vector3, fov: f32) -> Vector2WithDepth {
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
    let screen_point = Vector2 {
        x: HEIGHT as i32 / 2 + (x_offset as i32),
        y: HEIGHT as i32 / 2 - (y_offset as i32),
    };
    Vector2WithDepth {
        v: screen_point,
        depth: point.z,
    }
}

pub fn draw_triangles(pixels_buffer: &mut [RGBA], triangles: &[ColoredTriangle], fov: f32) {
    // Separate depth buffer for performance
    let mut depth_buffer = vec![f32::MIN; pixels_buffer.len()];
    for tri in triangles {
        // Skip triangles that are behind the camera
        if (tri.triangle.a.z >= 0.0) || (tri.triangle.b.z >= 0.0) || (tri.triangle.c.z >= 0.0) {
            continue;
        }
        let a = world_to_screen(&tri.triangle.a, fov);
        let b = world_to_screen(&tri.triangle.b, fov);
        let c = world_to_screen(&tri.triangle.c, fov);

        // Compute bounding box once for the triangle
        let min_x = a.v.x.min(b.v.x).min(c.v.x).max(0);
        let min_y = a.v.y.min(b.v.y).min(c.v.y).max(0);
        let max_x = a.v.x.max(b.v.x).max(c.v.x).min(WIDTH as i32);
        let max_y = a.v.y.max(b.v.y).max(c.v.y).min(HEIGHT as i32);

        let area = signed_triangle_area(&a.v, &b.v, &c.v);
        if area <= 0.0 {
            continue; // Triangle is not oriented correctly
        }
        let inv_area = 1.0 / area;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = Vector2 { x, y };

                // Test if point is on the right side of each edge of the triangle
                let area_abp = signed_triangle_area(&a.v, &b.v, &p);
                let area_bcp = signed_triangle_area(&b.v, &c.v, &p);
                let area_cap = signed_triangle_area(&c.v, &a.v, &p);
                let inside = area_abp >= 0.0 && area_bcp >= 0.0 && area_cap >= 0.0;

                // Weighting factors (barycentric coordinates)
                let weight_a = area_abp * inv_area;
                let weight_b = area_bcp * inv_area;
                let weight_c = area_cap * inv_area;
                let weights = Vector3 {
                    x: weight_a,
                    y: weight_b,
                    z: weight_c,
                };

                let depth = dot3(
                    &weights,
                    &Vector3 {
                        x: a.depth,
                        y: b.depth,
                        z: c.depth,
                    },
                );

                let index = (y * WIDTH as i32 + x) as usize;
                if inside && depth > depth_buffer[index] {
                    pixels_buffer[index] = tri.color;
                    depth_buffer[index] = depth;
                }
            }
        }
    }
}
