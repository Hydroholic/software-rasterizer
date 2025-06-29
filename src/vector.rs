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

pub fn point_is_on_right_side_of_vect2(a: &Vector2, b: &Vector2, p: &Vector2) -> bool {
    let ab = substract2(b, a);
    let ap = substract2(p, a);
    let perp_ab = perpendicular2_clockwise(&ab);
    dot2(&perp_ab, &ap) >= 0.0
}

pub fn point_in_triangle(a: &Vector2, b: &Vector2, c: &Vector2, p: &Vector2) -> bool {
    let side_ab = point_is_on_right_side_of_vect2(a, b, p);
    let side_bc = point_is_on_right_side_of_vect2(b, c, p);
    let side_ca = point_is_on_right_side_of_vect2(c, a, p);

    (side_bc == side_ca) && (side_ab == side_bc)
}

