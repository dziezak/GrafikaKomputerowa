use crate::geometry::polygon::{ConstraintType, Polygon};
use crate::geometry::point::{Continuity, Point};
use crate::geometry::point::PointRole::Vertex;

pub struct Selection {
    pub selected_vertex: Option<usize>,
    pub selected_control: Option<(usize, u8)>,
}

impl Selection {
    pub fn new() -> Self {
        Self
        {
            selected_vertex: None,
            selected_control: None,
        }
    }

    //TODO: tutaj jest problem bo musi byc mozliwosc wyboru wierzcholka kontrolnego Beziera
    pub fn select_vertex(&mut self, polygon: &Polygon, mouse_pos: Point, radius: f32) -> Option<usize> {
        if let Some(idx) = polygon.vertices.iter().position(|v| v.distance(&mouse_pos) < radius) {
            self.selected_vertex = Some(idx);
            return Some(idx);
        }

        for (i, constraint_opt) in polygon.constraints.iter().enumerate() {
            if let Some(ConstraintType::Bezier { control1, control2, .. }) = constraint_opt {
                if control1.distance(&mouse_pos) < radius {
                    self.selected_vertex = None;
                    self.selected_control = Some((i, 1));
                    return None;
                }
                if control2.distance(&mouse_pos) < radius {
                    self.selected_vertex = None;
                    self.selected_control = Some((i, 2));
                    return None;
                }
            }
        }

        self.selected_vertex = None;
        self.selected_control = None;
        None
    }



    pub fn select_edge(&self, polygon: &Polygon, mouse: &Point, radius: f32) -> Option<usize> {
        let mut closest = None;
        let mut min_dist = radius;

        let n = polygon.vertices.len();
        if n < 2 {
            return None;
        }

        for i in 0..n {
            let start = &polygon.vertices[i];
            let end = &polygon.vertices[(i + 1) % n]; // <--- UWAGA TU: % n robi wrap-around
            let dist = distance_point_to_segment(mouse, start, end);
            if dist < min_dist {
                min_dist = dist;
                closest = Some(i);
            }
        }
        closest
    }
}
fn distance_point_to_segment(p: &Point, a: &Point, b: &Point) -> f32 {
    let ap = Point { x: p.x - a.x, y: p.y - a.y , role: Vertex, continuity: Continuity::None };
    let ab = Point { x: b.x - a.x, y: b.y - a.y , role: Vertex, continuity: Continuity::None };
    let ab2 = ab.x * ab.x + ab.y * ab.y;
    let dot = ap.x * ab.x + ap.y * ab.y;
    let t = (dot / ab2).clamp(0.0, 1.0);
    let closest = Point {
        x: a.x + ab.x * t,
        y: a.y + ab.y * t,
        role: Vertex,
        continuity: Continuity::None,
    };
    closest.distance(&p)
}
