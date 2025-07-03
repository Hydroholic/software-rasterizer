use crate::{
    HEIGHT, WIDTH,
    renderer::{self, RGBA},
};

#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
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
        let ab = substract2(&b, &a);
        let bc = substract2(&c, &b);
        let ca = substract2(&a, &c);
        let perpendicular_ab = perpendicular2_clockwise(&ab);
        let perpendicular_bc = perpendicular2_clockwise(&bc);
        let perpendicular_ca = perpendicular2_clockwise(&ca);

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
        let ap = substract2(p, &self.a);
        let bp = substract2(p, &self.b);
        let cp = substract2(p, &self.c);
        let dot_abp = dot2(&self.perpendicular_ab, &ap) >= 0.0;
        let dot_bcp = dot2(&self.perpendicular_bc, &bp) >= 0.0;
        let dot_cap = dot2(&self.perpendicular_ca, &cp) >= 0.0;

        (dot_cap == dot_bcp) && (dot_bcp == dot_abp)
    }
}

pub fn dot2(a: &Vector2, b: &Vector2) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn perpendicular2_clockwise(a: &Vector2) -> Vector2 {
    Vector2 { x: a.y, y: -a.x }
}

pub fn substract2(a: &Vector2, b: &Vector2) -> Vector2 {
    Vector2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
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

    for y in min_height..max_height {
        for x in min_width..max_width {
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


fn world_to_screen(point: &Vector3) -> Vector2 {
    let world_height = HEIGHT as f32/ 5.0;
    let offset = Vector2 {
        x: point.x * world_height, y: point.y * world_height,
    };
    Vector2 {
        x: HEIGHT as f32 / 2.0 + offset.x,
        y: HEIGHT as f32 / 2.0 - offset.y,
    }
}

pub fn draw_triangles(pixels_buffer: &mut [RGBA], points: &[Vector3]) {
    for i in (0..points.len()).step_by(3) {
        if i + 2 >= points.len() {
            panic!("Not enough points to form a triangle");
        }
        let a = world_to_screen(&points[i]);
        let b = world_to_screen(&points[i + 1]);
        let c = world_to_screen(&points[i + 2]);
        let triangle = Triangle2::new(a, b, c);
        let color = renderer::RGBA {
            r: rand::random(),
            g: rand::random(),
            b: rand::random(),
            a: 255,
        };
        println!(
            "Drawing triangle with points: {:?}, {:?}, {:?}",
            triangle.a, triangle.b, triangle.c
        );
        fill_triangle_buffer(pixels_buffer, &triangle, color);
    }
}
