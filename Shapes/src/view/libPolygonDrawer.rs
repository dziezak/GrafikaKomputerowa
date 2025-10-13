use consts::TAU;
use f32::consts;
use std::f32;
use egui::{Painter, Color32, Pos2, Stroke, Align2};
use crate::geometry::polygon::{Polygon, ConstraintType, EdgeType};
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

    fn draw_arc(painter: &egui::Painter, start: Point, end: Point, center: Point, radius: f32, color: egui::Color32) {
        let steps = 64; // im więcej, tym gładszy łuk
        let angle_start = (start.y - center.y).atan2(start.x - center.x);
        let angle_end = (end.y - center.y).atan2(end.x - center.x);

        // Zakładamy łuk mniejszy niż pół okręgu
        let mut points = Vec::new();
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let angle = angle_start + t * (angle_end - angle_start);
            points.push(egui::pos2(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            ));
        }

        for w in points.windows(2) {
            painter.line_segment([w[0], w[1]], egui::Stroke::new(2.0, color));
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

                Some(ConstraintType::Arc {..}) => {
                    let (center, radius) = polygon.compute_default_arc(*start, *end);
                    PolygonDrawer::draw_arc(painter, *start, *end, center, radius, Color32::LIGHT_GRAY);
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



}
