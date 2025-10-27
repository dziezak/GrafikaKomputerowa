use crate::view::IPolygonDrawer::IPolygonDrawer;
use egui::{Painter, Pos2};
use crate::geometry::polygon::{Polygon, ConstraintType};
use eframe::egui;
use eframe::epaint::{Color32, Stroke};
use crate::geometry::point;
use crate::geometry::point::{Continuity, Point};
use crate::view::PolygonDrawer;

pub struct MyPolygonDrawer;

impl MyPolygonDrawer {
    pub fn new() -> Self {
        Self
    }

    fn draw_pixel(painter: &egui::Painter, x: i32, y: i32, color: egui::Color32) {
        let size = 2.0;
        painter.rect_filled(
            egui::Rect::from_min_size(egui::pos2(x as f32, y as f32), egui::vec2(size, size)),
            0.0,
            color,
        );
    }

    fn bresenham_line(
        painter: &egui::Painter,
        start: (i32, i32),
        end: (i32, i32),
        color: egui::Color32,
    ) {
        let (mut x0, mut y0) = start;
        let (x1, y1) = end;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            Self::draw_pixel(painter, x0, y0, color);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }

}

impl IPolygonDrawer for MyPolygonDrawer {
    fn draw(&self, painter: &egui::Painter, polygon: &mut Polygon) {
        polygon.ensure_constraints_len();
        let n = polygon.vertices.len();
        if n < 2 {
            return;
        }

        for i in 0..n {
            let start = &polygon.vertices[i];
            let end = &polygon.vertices[(i + 1) % n]; // wrap-around

            match polygon.constraints[i] {

                Some(ConstraintType::Arc {g1_start, g1_end}) => {

                    let prev = if i == 0 { None } else { Some(polygon.vertices[i - 1]) };
                    let next = if i + 1 < n - 1 { Some(polygon.vertices[i + 2]) } else { None };

                    let (center, radius) = PolygonDrawer::compute_arc_geometry(
                        *start,
                        *end,
                        prev,
                        next,
                        g1_start,
                        g1_end,
                    );


                    //if(g1_start || g1_end) { eprintln!("g1_start != g1_end");}

                    let start_angle = (*start - center).y.atan2((*start - center).x);
                    let end_angle = (*end - center).y.atan2((*end - center).x);
                    let mut arc_angle = end_angle - start_angle;
                    if arc_angle < 0.0 {
                        arc_angle = arc_angle * (-1.0);
                    }
                    self.draw_arc_between_points(
                        painter,
                        Pos2::new(start.x, start.y),
                        Pos2::new(end.x, end.y),
                        arc_angle,
                        Color32::WHITE,
                        1.0,
                    );
                }

                Some(ConstraintType::Bezier {control1, control2, g1_start, g1_end, c1_start, c1_end }) => {
                    self.draw_dashed_polyline(
                        painter,
                        &[
                            egui::pos2(start.x, start.y),
                            egui::pos2(control1.x, control1.y),
                            egui::pos2(control2.x, control2.y),
                            egui::pos2(end.x, end.y),
                        ],
                        egui::Stroke::new(1.0, egui::Color32::WHITE),
                    );
                    self.draw_cubic_bezier(
                        painter,
                        *start,
                        control1,
                        control2,
                        *end,
                        egui::Stroke::new(1.0, egui::Color32::WHITE),
                    );
                    painter.circle_filled(egui::pos2(control1.x, control1.y), 4.0, egui::Color32::GRAY);
                    painter.circle_filled(egui::pos2(control2.x, control2.y), 4.0, egui::Color32::GRAY);

                    if g1_start {
                        painter.line_segment(
                            [egui::pos2(start.x, start.y), egui::pos2(control1.x, control1.y)],
                            egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
                        );
                    }

                    if g1_end {
                        painter.line_segment(
                            [egui::pos2(end.x, end.y), egui::pos2(control2.x, control2.y)],
                            egui::Stroke::new(1.0, egui::Color32::LIGHT_BLUE),
                        );
                    }

                }

                _ => {
                    let start_i32 = (start.x as i32, start.y as i32);
                    let end_i32 = (end.x as i32, end.y as i32);

                    Self::bresenham_line(
                        painter,
                        start_i32,
                        end_i32,
                        egui::Color32::WHITE,
                    );
                }

            }



            let mid = egui::pos2((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);

            if let Some(Some(constraint)) = polygon.constraints.get(i) {
                let text = match constraint {
                    ConstraintType::Horizontal => "H".to_string(),
                    ConstraintType::Vertical => "V".to_string(),
                    ConstraintType::Diagonal45 => "D".to_string(),
                    ConstraintType::Arc { g1_start: _, g1_end: _ } => "A".to_string(),
                    ConstraintType::Bezier { .. } => "B".to_string(),
                    ConstraintType::FixedLength(len) => format!("{:.1}", len),
                    _=> "".to_string(),
                };
                painter.text(
                    mid,
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::monospace(14.0),
                    egui::Color32::YELLOW,
                );
            }
        }

        for v in &polygon.vertices { //TODO można zmieniac kolor jak jesteś nad nim
            painter.circle_filled(egui::pos2(v.x, v.y), 5.0, egui::Color32::RED);
            self.draw_continuity_label(painter, v);
        }
    }



    fn draw_arc_between_points(
        &self,
        painter: &Painter,
        p1: Pos2,
        p2: Pos2,
        arc_angle: f32,
        color: Color32,
        thickness: f32,
    ) {
        let mid = Pos2::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0);

        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let chord_length = (dx * dx + dy * dy).sqrt();

        let radius = chord_length / (2.0 * (arc_angle / 2.0).sin());

        let chord_angle = dy.atan2(dx);

        let perp_angle = chord_angle + std::f32::consts::FRAC_PI_2;

        let h = (radius * radius - (chord_length / 2.0).powi(2)).sqrt();

        let center = Pos2::new(mid.x + h * perp_angle.cos(), mid.y + h * perp_angle.sin());

        let start_angle = (p1.y - center.y).atan2(p1.x - center.x);
        let end_angle = (p2.y - center.y).atan2(p2.x - center.x);

        let segments = 1000;
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let angle = start_angle + t * (end_angle - start_angle);
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            let pos = Pos2::new(x, y);
            painter.circle_filled(pos, thickness, color);
        }
    }


    fn compute_arc_geometry(
        start: Point,
        end: Point,
        tangent_start: Option<Point>, // punkt kierunku dla G1 start
        tangent_end: Option<Point>,   // punkt kierunku dla G1 end
        g1_start: bool,
        g1_end: bool,
    ) -> (Point, f32) {
        let chord = end - start;
        let chord_len = chord.length();
        let mid = (start + end) * 0.5;

        if !g1_start && !g1_end {
            let normal = Point::new(-chord.y, chord.x).normalized();
            let center = mid + normal * (chord_len / 2.0);
            return (center, (center - start).length());
        }

        if g1_start {
            if let Some(ts) = tangent_start {
                let tangent_dir = (start - ts).normalized();
                let normal_start = Point::new(-tangent_dir.y, tangent_dir.x);

                let normal_chord = Point::new(-chord.y, chord.x).normalized();

                let denom = normal_start.x * normal_chord.y - normal_start.y * normal_chord.x;
                if denom.abs() < 1e-6 {
                    let center = mid + normal_chord * (chord_len / 2.0);
                    return (center, (center - start).length());
                }

                let delta = mid - start;
                let t = (delta.x * normal_chord.y - delta.y * normal_chord.x) / denom;
                let center = start + normal_start * t;
                let radius = (center - start).length();
                return (center, radius);
            }
        }

        if g1_end {
            if let Some(te) = tangent_end {
                let tangent_dir = (te - end).normalized();
                let normal_end = Point::new(-tangent_dir.y, tangent_dir.x);
                let normal_chord = Point::new(-chord.y, chord.x).normalized();

                let denom = normal_end.x * normal_chord.y - normal_end.y * normal_chord.x;
                if denom.abs() < 1e-6 {
                    let center = mid + normal_chord * (chord_len / 2.0);
                    return (center, (center - start).length());
                }

                let delta = mid - end;
                let t = (delta.x * normal_chord.y - delta.y * normal_chord.x) / denom;
                let center = end + normal_end * t;
                let radius = (center - end).length();
                return (center, radius);
            }
        }

        let normal = Point::new(-chord.y, chord.x).normalized();
        let center = mid + normal * (chord_len / 2.0);
        (center, (center - start).length())
    }

    fn draw_cubic_bezier(
        &self,
        painter: &egui::Painter,
        p0: Point,
        p1: Point,
        p2: Point,
        p3: Point,
        stroke: egui::Stroke,
    ) {
        let steps = 64;
        let mut prev = egui::pos2(p0.x, p0.y);
        for i in 1..=steps {
            let t = i as f32 / steps as f32;
            let u = 1.0 - t;
            let x = u*u*u*p0.x + 3.0*u*u*t*p1.x + 3.0*u*t*t*p2.x + t*t*t*p3.x;
            let y = u*u*u*p0.y + 3.0*u*u*t*p1.y + 3.0*u*t*t*p2.y + t*t*t*p3.y;
            let cur = egui::pos2(x, y);
            painter.line_segment([prev, cur], stroke);
            prev = cur;
        }
    }

    fn draw_dashed_polyline(&self, painter: &egui::Painter, pts: &[egui::Pos2], stroke: egui::Stroke) {
        for w in pts.windows(2) {
            let a = w[0]; let b = w[1];
            let dir = (b - a);
            let len = dir.length();
            if len <= 0.0 { continue; }
            let step = 6.0_f32; // dash length
            let n = (len / step).ceil() as usize;
            for i in 0..n {
                let t0 = (i as f32) * step / len;
                let t1 = ((i as f32) * step + step/2.0) / len; // half on, half off
                let t0 = t0.min(1.0);
                let t1 = t1.min(1.0);
                let p0 = a + dir * t0;
                let p1 = a + dir * t1;
                painter.line_segment([p0, p1], stroke);
            }
        }
    }

    fn draw_continuity_label(&self, painter: &Painter, point: &Point) {
        use egui::Align2;

        let label = match point.continuity {
            Continuity::G1 => Some("G1"),
            Continuity::C1 => Some("C1"),
            _ => None,
        };

        if let Some(text) = label {
            let pos = egui::pos2(point.x, point.y - 15.0);
            painter.text(
                pos,
                Align2::CENTER_BOTTOM,
                text,
                egui::FontId::proportional(14.0),
                egui::Color32::from_rgb(200, 255, 200),
            );
        }
    }
}

