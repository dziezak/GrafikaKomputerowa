use consts::TAU;
use f32::consts;
use std::f32;
use egui::{Painter, Color32, Pos2, Stroke, Align2};
use crate::geometry::polygon::{Polygon, ConstraintType};
use eframe::egui;
//use egui::accesskit::Point;
use crate::view::IPolygonDrawer::IPolygonDrawer;
use crate::geometry::point::Point;

pub struct PolygonDrawer;

impl PolygonDrawer {
    pub fn new() -> Self{
        Self
    }
}
impl IPolygonDrawer for PolygonDrawer {

    // funkcja tylko rsuje odpowiednio okrag
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
                _ => {
                    painter.line_segment(
                        [egui::pos2(start.x, start.y), egui::pos2(end.x, end.y)],
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    );
                }
            }



            let mid = egui::pos2((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);

            if let Some(constraint) = polygon.constraints.get(i).copied().flatten() {
                let text = match constraint {
                    ConstraintType::Horizontal => "H".to_string(),
                    ConstraintType::Vertical => "V".to_string(),
                    ConstraintType::Diagonal45 => "D".to_string(),
                    ConstraintType::Arc { g1_start: _, g1_end: _ } => "A".to_string(),
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

        for v in &polygon.vertices {
            painter.circle_filled(egui::pos2(v.x, v.y), 5.0, egui::Color32::RED);
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
        // Wektor cięciwy
        let chord = end - start;
        let chord_len = chord.length();
        let mid = (start + end) * 0.5;

        // --- klasyczny G0 ---
        if !g1_start && !g1_end {
            let normal = Point::new(-chord.y, chord.x).normalized();
            let center = mid + normal * (chord_len / 2.0);
            return (center, (center - start).length());
        }

        // --- G1 continuity on start ---
        if g1_start {
            if let Some(ts) = tangent_start {
                let tangent_dir = (start - ts).normalized();
                let normal_start = Point::new(-tangent_dir.y, tangent_dir.x);

                // linia normalna do stycznej w start: center = start + normal_start * t
                // linia prostopadła do cięciwy w połowie: center = mid + normal_chord * s
                // rozwiązujemy dla t i s:
                let normal_chord = Point::new(-chord.y, chord.x).normalized();

                let denom = normal_start.x * normal_chord.y - normal_start.y * normal_chord.x;
                if denom.abs() < 1e-6 {
                    // proste równoległe — fallback do G0
                    let center = mid + normal_chord * (chord_len / 2.0);
                    return (center, (center - start).length());
                }

                // proste się przetną -> znajdź punkt przecięcia
                let delta = mid - start;
                let t = (delta.x * normal_chord.y - delta.y * normal_chord.x) / denom;
                let center = start + normal_start * t;
                let radius = (center - start).length();
                return (center, radius);
            }
        }

        // --- G1 continuity on end ---
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

        // fallback
        let normal = Point::new(-chord.y, chord.x).normalized();
        let center = mid + normal * (chord_len / 2.0);
        (center, (center - start).length())
    }


}