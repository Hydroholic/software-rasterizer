#[derive(Debug)]
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
pub struct Triangle {
    pub a: Vector2,
    pub b: Vector2,
    pub c: Vector2,
    perpendicular_ab: Vector2,
    perpendicular_bc: Vector2,
    perpendicular_ca: Vector2,
}

impl Triangle {
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
