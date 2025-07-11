use crate::vector::{self, Vector3};

pub fn parse_obj(obj_data: &str) -> Vec<vector::Triangle3> {
    let mut vertices = Vec::new();
    let mut triangle_points = Vec::new();

    for line in obj_data.lines() {
        let line = line.trim();
        if let Some(prefix) = line.strip_prefix("v ") {
            let parts: Vec<&str> = prefix.split_whitespace().collect();
            if parts.len() == 3 {
                let v = Vector3 { 
                    x: parts[0].parse::<f32>().unwrap(),
                    y: parts[1].parse::<f32>().unwrap(),
                    z: parts[2].parse::<f32>().unwrap(),
                };
                vertices.push(v);
            }
        } else if let Some(prefix) = line.strip_prefix("f ") {
            let parts: Vec<&str> = prefix.split_whitespace().collect();
            if parts.len() == 3 {
                let mut indices = Vec::new();
                for part in parts.iter() {
                    let idx: usize = part.split('/').next().unwrap().parse::<usize>().unwrap() - 1;
                    indices.push(idx);
                }
                let triangle = vector::Triangle3 {
                    a: vertices[indices[0]].clone(),
                    b: vertices[indices[1]].clone(),
                    c: vertices[indices[2]].clone(),
                };
                triangle_points.push(triangle);
            }
        }
    }
    triangle_points
}
