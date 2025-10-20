use crate::geometry::point::PointRole::Vertex;
use super::point::{Continuity, Point, PointRole};

#[derive(Clone, Copy)]
pub enum ConstraintType {
    Horizontal,
    Vertical,
    Diagonal45,
    FixedLength(f64),
    Arc {
        g1_start: bool,
        g1_end: bool,
    },
    Line,
    Bezier {
        control1: Point,
        control2: Point,
        g1_start: bool,
        g1_end: bool,
        c1_start: bool,
        c1_end: bool,
    },
}

#[derive(Clone, Debug)]
pub enum ContinuityType {
    G0,
    G1,
    G2,
}

pub struct Polygon {
    pub vertices: Vec<Point>,
    pub constraints: Vec<Option<ConstraintType>>,
}


impl Polygon {
    //klasyk konstruktor
    pub fn new(vertices: Vec<Point>) -> Self {
        let constraints = vec![None; vertices.len()];
        Self{vertices, constraints }
    }

    fn sync_constraints(&mut self){
        if self.constraints.len() != self.vertices.len() {
            self.constraints.resize(self.vertices.len(), None);
        }
    }

    //normal move
    pub fn move_vertex(&mut self, index: usize, dx:f32, dy: f32){
        if let Some(v) = self.vertices.get_mut(index){
            v.translate(dx, dy);
        }
        //self.enforce_vertex_continuity_after_vertex_move(index);
    }

    //usun wierzcholek
    pub fn remove_vertex(&mut self, index: usize){
        if self.vertices.len() <= 3 {
            return;
        }
        self.vertices.remove(index);
        if index > 0 {
            self.constraints[index - 1] = None;
        }

        if index < self.constraints.len() {
            self.constraints[index] = None;
        }
        self.ensure_constraints_len();
        self.apply_constraints();
    }

    //dodaj wierzcholek ez
    pub fn add_vertex_mid_edge(&mut self, start_idx: usize, end_idx: usize) {
        let n = self.vertices.len();
        if n < 2 {
            return
        }

        let start = self.vertices[start_idx];
        let end = self.vertices[(end_idx)%n];
        let mid = Point {
            x: (start.x + end.x) / 2.0,
            y: (start.y + end.y) / 2.0,
            role: Vertex,
            continuity: Continuity::None,
        };

        if end_idx == 0 {
            self.vertices.push(mid);
            self.constraints.push(None);
        } else {
            self.vertices.insert(end_idx, mid);
            self.constraints.insert(start_idx + 1, None);
        }

        // usuń constraint na starej krawędzi
        if start_idx < self.constraints.len() {
            self.constraints[start_idx] = None;
        }

        self.ensure_constraints_len();
        self.apply_constraints();
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

    pub fn apply_constraints(&mut self) {
        let n = self.vertices.len();
        if n < 2 {
            return;
        }

        for i in 0..n {
            if let Some(constraint) = self.get_constraint(i) {
                let start_idx = i;
                let end_idx = (i + 1) % n;
                self.enforce_constraint(start_idx, end_idx, &constraint);
            }
        }
    }

    // matematyczna logika constrainow (najwazniejsza funkcja)
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
                let dx = end.x - start.x;
                let dy = end.y - start.y;
                let len = (dx * dx + dy * dy).sqrt();

                if len < std::f32::EPSILON {
                    return;
                }

                let mid_x = (start.x + end.x) / 2.0;
                let mid_y = (start.y + end.y) / 2.0;

                // Kierunek 45° – zachowujemy orientację (czyli znak, w którą stronę ma się „pochylać”)
                let sign_x = if dx >= 0.0 { 1.0 } else { -1.0 };
                let sign_y = if dy >= 0.0 { 1.0 } else { -1.0 };

                // odległość od środka do końca wzdłuż 45°
                let offset = len / (2.0 * std::f32::consts::SQRT_2);

                self.vertices[start_idx].x = mid_x - offset * sign_x;
                self.vertices[start_idx].y = mid_y - offset * sign_y;
                self.vertices[end_idx].x = mid_x + offset * sign_x;
                self.vertices[end_idx].y = mid_y + offset * sign_y;
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

            ConstraintType::Bezier { .. } => {
                let n = self.vertices.len();
                if n < 2 {
                    return;
                }

                let v_start = self.vertices[start_idx];
                let v_end = self.vertices[end_idx];
                let prev_idx = if start_idx == 0 { n - 1 } else { start_idx - 1 };
                let next_idx = (end_idx + 1) % n;

                // --- Jeśli ruszono początek tej krzywej ---
                if let Some(ConstraintType::Bezier { control1, .. }) =
                    self.constraints.get_mut(start_idx).and_then(|c| c.as_mut())
                {
                    let cont = v_start.continuity;
                    let prev = self.vertices[prev_idx];

                    let new_control1 = match cont {
                        Continuity::G1 =>{
                            let dir_x = v_start.x - prev.x;
                            let dir_y = v_start.y - prev.y;
                            let len = (dir_x * dir_x + dir_y * dir_y).sqrt();

                            if len < std::f32::EPSILON {
                            *control1 // nie zmieniamy, bo nie ma sensu
                            } else {
                                let current_len = ((control1.x - v_start.x).powi(2) + (control1.y - v_start.y).powi(2)).sqrt();
                                let norm_x = dir_x / len;
                                let norm_y = dir_y / len;

                                Point {
                                    x: v_start.x + norm_x * current_len,
                                    y: v_start.y + norm_y * current_len,
                                    role: PointRole::Control,
                                    continuity: cont,
                                }
                            }
                        },
                        Continuity::C1 => {
                            let dx = (v_start.x - prev.x)/3.0;
                            let dy = (v_start.y - prev.y)/3.0;
                            Point {
                                x: v_start.x + dx,
                                y: v_start.y + dy,
                                role: PointRole::Control,
                                continuity: cont,
                            }
                        }
                        _ => *control1,
                    };

                    *control1 = new_control1;

                    // aktualizuj poprzednią krzywą (odbicie control2)

                }

                //
                // --- Jeśli ruszono koniec tej krzywej ---
                //
                if let Some(ConstraintType::Bezier { control2, .. }) =
                    self.constraints.get_mut(start_idx).and_then(|c| c.as_mut())
                {
                    //eprintln!("CHUJ");
                    let cont = v_end.continuity;
                    let next = self.vertices[next_idx];

                    let new_control2 = match cont {
                        Continuity::G1 => {
                            let dir_x = v_end.x - next.x;
                            let dir_y = v_end.y - next.y;
                            let len = (dir_x * dir_x + dir_y * dir_y).sqrt();

                            if len < std::f32::EPSILON {
                                *control2 // nie zmieniamy, bo nie ma sensu
                            } else {
                                let current_len = ((control2.x - v_end.x).powi(2) + (control2.y - v_end.y).powi(2)).sqrt();
                                let norm_x = dir_x / len;
                                let norm_y = dir_y / len;

                                Point {
                                x: v_end.x + norm_x * current_len,
                                y: v_end.y + norm_y * current_len,
                                role: PointRole::Control,
                                continuity: cont,
                                }
                            }
                        },
                        Continuity::C1 => {
                            let dx = (v_end.x - next.x)/3.0;
                            let dy = (v_end.y - next.y)/3.0;
                            Point {
                                x: v_end.x + dx,
                                y: v_end.y + dy,
                                role: PointRole::Control,
                                continuity: cont,
                            }
                        }
                        _ => *control2,
                    };

                    *control2 = new_control2;

                }
            }


            _=> {}
        }
    }

    // sprawdzanie czy obok edge nie ma juz takiego samego constraintu
    pub fn is_constraint_legal(&self, edge_idx:usize, new_constraint: &ConstraintType) -> bool {
        if !matches!(new_constraint, ConstraintType::Horizontal | ConstraintType::Vertical) {
            return true;
        }

        let n= self.constraints.len();
        if n == 0 {
            return true;
        }

        let prev_idx = if edge_idx == 0 {n -1} else {edge_idx - 1};
        let next_idx = (edge_idx + 1) % n;
        let prev = &self.constraints[prev_idx];
        let next = &self.constraints[next_idx];

        match  new_constraint {
            ConstraintType::Horizontal => {
                !matches!(prev, Some(ConstraintType::Horizontal)) &&
                !matches!(next, Some(ConstraintType::Horizontal))
            }
            ConstraintType::Vertical => {
                matches!(prev, Some(ConstraintType::Vertical)) &&
                matches!(next, Some(ConstraintType::Vertical))
            }
            _=> true,
        }
    }


    ///ARCS:
    pub fn compute_default_arc(&self, start: Point, end: Point) -> (Point, f32) {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let length = (dx * dx + dy * dy).sqrt();
        let radius = length / 2.0;

        let mid = Point {
            x: (start.x + end.x) / 2.0,
            y: (start.y + end.y) / 2.0,
            role: Vertex,
            continuity: Continuity::None,
        };

        // Normalny wektor (prostopadły)
        let nx = -dy / length;
        let ny = dx / length;

        // Przesuwamy środek o promień po normalnej
        let center = Point {
            x: mid.x + nx * radius,
            y: mid.y + ny * radius,
            role: Vertex,
            continuity: Continuity::None,
        };

        (center, radius)
    }

    pub fn compute_arc_from_chord(start: Point, end: Point, radius_opt: Option<f32>, clockwise: bool) -> Option<(Point, f32)> {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let chord_len = (dx * dx + dy * dy).sqrt();

        if chord_len == 0.0 {
            return None;
        }

        // domyślny promień: połowa długości cięciwy (-> łuk półokręgu z centrum w midpoint)
        let r = radius_opt.unwrap_or(chord_len / 2.0);

        // promień musi spełniać r >= L/2
        if r < chord_len / 2.0 {
            return None;
        }

        let mid = Point { x: (start.x + end.x) / 2.0, y: (start.y + end.y) / 2.0, role: Vertex, continuity: Continuity::None };

        // wysokość od środka cięciwy do środka okręgu
        let half = chord_len / 2.0;
        let h = (r * r - half * half).max(0.0).sqrt(); // .max(0.0) zabezpiecza przed ujemnym zbog floatów

        // unit normal (prostopadły do wektora cięciwy)
        let ux = -dy / chord_len;
        let uy = dx / chord_len;

        // wybierz stronę normalnej w zależności od kierunku (clockwise)
        // (możesz też pozwolić użytkownikowi wybrać, ale tu prosty warunek)
        let sign = if clockwise { 1.0 } else { -1.0 };

        let cx = mid.x + sign * ux * h;
        let cy = mid.y + sign * uy * h;

        Some((Point { x: cx, y: cy, role: Vertex, continuity: Continuity::None}, r))
    }

    ///BEZIER
    pub fn enforce_continuity_after_control_move(&mut self, constraint_index: usize, control_id: u8) {
        if let Some(ConstraintType::Bezier {
                        control1,
                        control2,
                        ..
                    }) = &mut self.constraints[constraint_index]
        {
            let n = self.vertices.len();
            let start_idx = constraint_index;
            let end_idx = (constraint_index + 1) % n;

            match control_id {
                1 => {
                    // Przesunięto control1 → modyfikujemy poprzedni wierzchołek (vertex[i - 1])
                    let v_start = self.vertices[start_idx];
                    let prev_idx = if start_idx == 0 { n - 1 } else { start_idx - 1 };
                    let prev = self.vertices[prev_idx];
                    let cont = v_start.continuity;

                    match cont {
                        Continuity::G1 => {
                            // chcemy zachować długość poprzedniej krawędzi, tylko ustawić jej kierunek
                            let dx = control1.x - v_start.x;
                            let dy = control1.y - v_start.y;
                            let norm = (dx * dx + dy * dy).sqrt().max(1e-6);
                            let ux = dx / norm;
                            let uy = dy / norm;

                            // oryginalna długość krawędzi prev -> v_start
                            let prev_len = prev.distance(&v_start).max(1e-6);

                            // nowy poprzedni wierzchołek: v_start - unit * prev_len
                            self.vertices[prev_idx].x = v_start.x - ux * prev_len;
                            self.vertices[prev_idx].y = v_start.y - uy * prev_len;
                        }
                        Continuity::C1 => {
                            let dx = control1.x - v_start.x;
                            let dy = control1.y - v_start.y;
                            // C1: skala 1:3 (kontrolka wpływa na położenie wierzchołka z mnożnikiem 3)
                            self.vertices[prev_idx].x = v_start.x - dx * 3.0;
                            self.vertices[prev_idx].y = v_start.y - dy * 3.0;
                        }
                        _ => {}
                    }
                }

                2 => {
                    // Przesunięto control2 → modyfikujemy następny wierzchołek (vertex[i + 2])
                    let v_end = self.vertices[end_idx];
                    let next_idx = (end_idx + 1) % n;
                    let next = self.vertices[next_idx];
                    let cont = v_end.continuity;

                    match cont {
                        Continuity::G1 => {
                            // zachowujemy długość krawędzi v_end -> next, tylko zmieniamy kierunek
                            let dx = control2.x - v_end.x;
                            let dy = control2.y - v_end.y;
                            let norm = (dx * dx + dy * dy).sqrt().max(1e-6);
                            let ux = dx / norm;
                            let uy = dy / norm;

                            // oryginalna długość krawędzi v_end -> next
                            let next_len = next.distance(&v_end).max(1e-6);

                            // nowy next: v_end + unit * next_len
                            self.vertices[next_idx].x = v_end.x - ux * next_len;
                            self.vertices[next_idx].y = v_end.y - uy * next_len;
                        }
                        Continuity::C1 => {
                            let dx = control2.x - v_end.x;
                            let dy = control2.y - v_end.y;
                            self.vertices[next_idx].x = v_end.x - dx * 3.0;
                            self.vertices[next_idx].y = v_end.y - dy * 3.0;
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }


    // zwraca czy dana krawędź (index) jest Bezierem
    fn is_bezier_edge(&self, edge_idx: usize) -> bool {
        matches!(self.constraints.get(edge_idx).and_then(|c| c.as_ref()), Some(ConstraintType::Bezier { .. }))
    }

    // zwraca Option(control2) z poprzedniego segmentu (bez mut)
    fn prev_control2(&self, start_idx: usize) -> Option<Point> {
        let n = self.vertices.len();
        if n == 0 { return None; }
        let prev_idx = if start_idx == 0 { n - 1 } else { start_idx - 1 };
        match self.constraints.get(prev_idx).and_then(|c| c.as_ref()) {
            Some(ConstraintType::Bezier { control2, .. }) => Some(*control2),
            _ => None,
        }
    }

    // zwraca Option(control1) z następnego segmentu (bez mut)
    fn next_control1(&self, end_idx: usize) -> Option<Point> {
        let n = self.vertices.len();
        if n == 0 { return None; }
        let next_idx = (end_idx + 1) % n;
        match self.constraints.get(next_idx).and_then(|c| c.as_ref()) {
            Some(ConstraintType::Bezier { control1, .. }) => Some(*control1),
            _ => None,
        }
    }

    // sprawdza czy którąś z krawędzi incydentnych do wierzchołków start/end ma FixedLength
    fn is_prev_fixed(&self, start_idx: usize) -> bool {
        let n = self.vertices.len();
        let prev_idx = if start_idx == 0 { n - 1 } else { start_idx - 1 };
        matches!(self.constraints.get(prev_idx).and_then(|c| c.as_ref()), Some(ConstraintType::FixedLength(_)))
    }

    fn is_next_fixed(&self, end_idx: usize) -> bool {
        let n = self.vertices.len();
        let next_idx = (end_idx + 1) % n;
        matches!(self.constraints.get(next_idx).and_then(|c| c.as_ref()), Some(ConstraintType::FixedLength(_)))
    }







}


