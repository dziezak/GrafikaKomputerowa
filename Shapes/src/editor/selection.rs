use crate::geometry::polygon::Polygon;
use crate::geometry::point::Point;

pub struct Selection {
    pub selected_vertex: Option<usize>,
}

impl Selection {
    pub fn new() -> Self {
        Self{ selected_vertex: None}
    }

    pub fn select_vertex(&mut self, polygon: &Polygon, mouse_pos:Point, radius: f32) {
        self.selected_vertex = polygon.vertices.iter().position(|v| v.distance(&mouse_pos) < radius);
    }

    pub fn select_edge(&self, polygon: &Polygon, mouse: &Point, radius: f32) -> Option<usize> {
        let mut closest = None;
        let mut min_dist = radius;

        for (i, window) in polygon.vertices.windows(2).enumerate() {
            let start = &window[0];
            let end = &window[1];

            // Odległość punktu od odcinka
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
    let ap = Point { x: p.x - a.x, y: p.y - a.y };
    let ab = Point { x: b.x - a.x, y: b.y - a.y };
    let ab2 = ab.x * ab.x + ab.y * ab.y;
    let dot = ap.x * ab.x + ap.y * ab.y;
    let t = (dot / ab2).clamp(0.0, 1.0);
    let closest = Point {
        x: a.x + ab.x * t,
        y: a.y + ab.y * t,
    };
    closest.distance(&p)
}
