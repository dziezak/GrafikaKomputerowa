use super::point::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConstraintType {
    Horizontal, // przyszlosciowa mozliwosc
    Vertical,
    Diagonal45,
    FixedLength(f64),
}

pub struct Polygon {
    pub vertices: Vec<Point>,
    pub constraints: Vec<Option<ConstraintType>>,
}


impl Polygon {
    //klasyk konstruktor
    pub fn new(vertices: Vec<Point>) -> Self {
        let constraints = vec![None; vertices.len()];
        Self{vertices, constraints}
    }

    fn sync_constraints(&mut self){
        if self.constraints.len() != self.vertices.len() {
            self.constraints.resize(self.vertices.len(), None);
        }
    }

    pub fn move_vertex(&mut self, index: usize, dx:f32, dy: f32){
        if let Some(v) = self.vertices.get_mut(index){
            v.translate(dx, dy);
        }
    }

    //usun wierzcholek
    pub fn remove_vertex(&mut self, index: usize){
        if index < self.vertices.len() {
            self.vertices.remove(index);
            self.sync_constraints();
        }
    }

    //dodaj wierzcholek ez
    pub fn add_vertex_mid_edge(&mut self, start_idx:usize, end_idx:usize){
        if start_idx < self.vertices.len() && end_idx < self.vertices.len(){
            let start = &self.vertices[start_idx];
            let end = &self.vertices[end_idx];
            let mid = Point {
                x: (start.x + end.x) / 2.0,
                y: (start.y + end.y) / 2.0,
            };
            self.vertices.insert(end_idx, mid);
            self.sync_constraints();
            self.ensure_constraints_len();
        }
    }

    // ustawiamy ograniczenia na wybrana krawedz
    pub fn set_constaint(&mut self, edge_idx: usize, constaint: ConstraintType){
        if edge_idx < self.constraints.len() {
            self.constraints[edge_idx] = Some(constaint);
        }
    }

    // usuwamy ograniczenia na wybrana krawedz
    pub fn remove_constaint(&mut self, edge_idx: usize){
        if edge_idx < self.constraints.len() {
            self.constraints[edge_idx] = None;
        }
    }

    // self explaiable
    pub fn get_constraint(&self, edge_idx: usize) -> Option<ConstraintType> {
        if edge_idx < self.constraints.len() {
            self.constraints[edge_idx]
        }else{
            None
        }
    }

    // sprawdzanie constrainow
    pub fn ensure_constraints_len(&mut self){
        let edge_count = if self.vertices.len() > 1 {
            self.vertices.len()
        }else{
            0
        };
        if self.constraints.len() != edge_count {
            self.constraints.resize(edge_count, None);
        }
    }

    pub fn apply_constraints(&mut self, moved_vertex_idx: usize) {
        let n = self.vertices.len();
        if n < 2 {
            return;
        }

        let prev_idx = if moved_vertex_idx == 0 { n - 1 } else { moved_vertex_idx - 1 };
        let next_idx = (moved_vertex_idx + 1) % n;

        // Constraint na krawędzi poprzedniej (prev_idx -> moved_vertex_idx)
        if let Some(constraint) = &self.constraints[prev_idx].clone() {
            self.enforce_constraint(prev_idx, moved_vertex_idx, &constraint);
        }

        // Constraint na krawędzi następnej (moved_vertex_idx -> next_idx)
        if let Some(constraint) = &self.constraints[moved_vertex_idx].clone() {
            self.enforce_constraint(moved_vertex_idx, next_idx, &constraint);
        }

        //dodatkowa tablica do sledzenia
        let mut applied = vec![false; self.vertices.len()];

        self.apply_edge(prev_idx, false, &mut applied);
        self.apply_edge(next_idx, true, &mut applied);

    }


    fn enforce_constraint(&mut self, start_idx: usize, end_idx: usize, constraint: &ConstraintType) {
        let start = &self.vertices[start_idx];
        let end = &self.vertices[end_idx];

        let dx = end.x - start.x;
        let dy = end.y - start.y;

        match constraint {
            ConstraintType::Horizontal => {
                let mid_y = (start.y + end.y) / 2.0;
                self.vertices[start_idx].y = mid_y;
                self.vertices[end_idx].y = mid_y;
            }
            ConstraintType::Vertical => {
                let mid_x = (start.x + end.x) / 2.0;
                self.vertices[start_idx].x = mid_x;
                self.vertices[end_idx].x = mid_x;
            }
            ConstraintType::Diagonal45 => {
                let mid_x = (start.x + end.x) / 2.0;
                let mid_y = (start.y + end.y) / 2.0;
                let sign_dx = (end.x - start.x).signum();
                let sign_dy = (end.y - start.y).signum();
                let half_len = ((dx * dx + dy * dy).sqrt()) / 2.0;
                self.vertices[start_idx].x = mid_x - half_len * sign_dx;
                self.vertices[start_idx].y = mid_y - half_len * sign_dy;
                self.vertices[end_idx].x = mid_x + half_len * sign_dx;
                self.vertices[end_idx].y = mid_y + half_len * sign_dy;
            }
            ConstraintType::FixedLength(len) => {
                let current_len = (dx * dx + dy * dy).sqrt();
                if current_len > 0.0 {
                    let scale = *len as f32 / current_len;
                    let mid_x = (start.x + end.x) / 2.0;
                    let mid_y = (start.y + end.y) / 2.0;
                    self.vertices[start_idx].x = mid_x - dx * scale / 2.0;
                    self.vertices[start_idx].y = mid_y - dy * scale / 2.0;
                    self.vertices[end_idx].x = mid_x + dx * scale / 2.0;
                    self.vertices[end_idx].y = mid_y + dy * scale / 2.0;
                }
            }
        }
    }


    pub fn apply_edge(&mut self, edge_idx: usize, move_next: bool, applied: &mut Vec<bool>) {
        if applied[edge_idx] {
            return;
        }
        applied[edge_idx] = true;

        let n = self.vertices.len();
        if n < 2 {
            return;
        }

        let start_idx = edge_idx;
        let end_idx = (edge_idx + 1) % n;

        if let Some(constraint) = self.constraints[edge_idx].clone() {
            match constraint {
                ConstraintType::Horizontal | ConstraintType::Vertical | ConstraintType::Diagonal45 => {
                    self.enforce_constraint(start_idx, end_idx, &constraint);

                    // Rozszerzamy propagację do sąsiednich krawędzi
                    if move_next {
                        let next_edge = (edge_idx + 1) % n;
                        self.apply_edge(next_edge, true, applied);
                    } else if edge_idx > 0 {
                        let prev_edge = if edge_idx == 0 { n - 1 } else { edge_idx - 1 };
                        self.apply_edge(prev_edge, false, applied);
                    }
                }

                ConstraintType::FixedLength(_) => {
                    self.enforce_constraint(start_idx, end_idx, &constraint);
                }
            }
        }
    }


}