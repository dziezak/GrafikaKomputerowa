use super::point::Point;

pub struct Polygon {
    pub vertices: Vec<Point>
}

impl Polygon {
    pub fn new(vertices: Vec<Point>) -> Self {
        Self{vertices}
    }

    pub fn move_vertex(&mut self, index: usize, dx:f32, dy: f32){
        if let Some(v) = self.vertices.get_mut(index){
            v.translate(dx, dy);
        }
    }

    pub fn remove_vertex(&mut self, index: usize){
        if index < self.vertices.len() {
            self.vertices.remove(index);
        }
    }

    pub fn add_vertex_mid_edge(&mut self, start_idx:usize, end_idx:usize){
        if start_idx < self.vertices.len() && end_idx < self.vertices.len(){
            let start = &self.vertices[start_idx];
            let end = &self.vertices[end_idx];
            let mid = Point {
                x: (start.x + end.x) / 2.0,
                y: (start.y + end.y) / 2.0,
            };
            self.vertices.insert(end_idx, mid);
        }
    }
}