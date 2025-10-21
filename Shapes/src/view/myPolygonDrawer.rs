use crate::view::IPolygonDrawer::IPolygonDrawer;
use egui::{Painter, Pos2};
use crate::geometry::polygon::{Polygon, ConstraintType};
use eframe::egui;
use eframe::epaint::{Color32, Stroke};
use crate::geometry::point;
use crate::geometry::point::{Continuity, Point};

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

    /// Implementacja algorytmu Bresenhama TODO: czy już dziła???
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

            Self::bresenham_line(
                painter,
                (start.x as i32, start.y as i32),
                (end.x as i32, end.y as i32),
                egui::Color32::WHITE,
            );

            let mid = egui::pos2((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);

            if let Some(constraint) = polygon.constraints.get(i).copied().flatten() {
                let text = match constraint {
                    ConstraintType::Horizontal => "H".to_string(),
                    ConstraintType::Vertical => "V".to_string(),
                    ConstraintType::Diagonal45 => "D".to_string(),
                    ConstraintType::Arc { g1_start: _, g1_end: _ } => "A".to_string(),
                    ConstraintType::FixedLength(len) => format!("{:.1}", len),
                    _ => "".to_string(),
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

    fn draw_cubic_bezier(&self, painter: &Painter, p0: Point, p1: Point, p2: Point, p3: Point, stroke: Stroke) {
        todo!()
    }

    fn draw_dashed_polyline(&self, painter: &Painter, pts: &[Pos2], stroke: Stroke) {
        todo!()
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

