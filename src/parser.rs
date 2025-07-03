use crate::vector::Vector3;

pub fn parse_obj(obj_data: &str) -> Vec<Vector3> {
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
                for part in parts.iter() {
                    let indices: Vec<&str> = part.split('/').collect();
                    let point_index = indices[0].parse::<usize>().unwrap() - 1;
                    triangle_points.push(vertices[point_index].clone());
                }
            }
        }
    }
    triangle_points
}
