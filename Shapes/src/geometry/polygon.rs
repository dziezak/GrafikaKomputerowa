use super::point::Point;

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
    },
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
    }


    //better move
    pub fn move_vertex_or_control(
        &mut self,
        vertex_index: Option<usize>,
        control: Option<(usize, u8)>,
        dx: f32,
        dy: f32,
    ) {
        if let Some(i) = vertex_index {
            if let Some(v) = self.vertices.get_mut(i) {
                v.translate(dx, dy);
            }

            for constraint_opt in self.constraints.iter_mut() {
                if let Some(ConstraintType::Bezier { control1, control2, g1_start, g1_end }) =
                    constraint_opt
                {
                    if *g1_start && i == 0 {
                        control1.translate(dx / 2.0, dy / 2.0);
                    }
                    if *g1_end && i == self.vertices.len() - 1 {
                        control2.translate(dx / 2.0, dy / 2.0);
                    }
                }
            }
        }

        if let Some((edge_idx, ctrl_num)) = control {
            if let Some(ConstraintType::Bezier { control1, control2, .. }) =
                self.constraints.get_mut(edge_idx).and_then(|x| x.as_mut())
            {
                match ctrl_num {
                    1 => control1.translate(dx, dy),
                    2 => control2.translate(dx, dy),
                    _ => {}
                }
            }
        }
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
        };

        // Normalny wektor (prostopadły)
        let nx = -dy / length;
        let ny = dx / length;

        // Przesuwamy środek o promień po normalnej
        let center = Point {
            x: mid.x + nx * radius,
            y: mid.y + ny * radius,
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

        let mid = Point { x: (start.x + end.x) / 2.0, y: (start.y + end.y) / 2.0 };

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

        Some((Point { x: cx, y: cy }, r))
    }

}