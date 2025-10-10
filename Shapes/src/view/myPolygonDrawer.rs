use crate::view::IPolygonDrawer::IPolygonDrawer;
use egui::{Painter, Pos2, Stroke, Align2};
use crate::geometry::polygon::{Polygon, ConstraintType};
use eframe::egui;

pub struct myPolygonDrawer;

impl myPolygonDrawer {
    pub fn new() -> Self{
        Self
    }

}
impl IPolygonDrawer for myPolygonDrawer {
    fn draw(&self, painter: &egui::Painter, polygon: &mut Polygon) {
        polygon.ensure_constraints_len();
        let n = polygon.vertices.len();
        if n < 2 {
            return;
        }

        for i in 0..n {
            let start = &polygon.vertices[i];
            let end = &polygon.vertices[(i + 1) % n]; // wrap-around

            ///TUTAJ Trzeba napisać swoje rysowanie linii algotyrmem Bresenhama
            painter.line_segment(
                [egui::pos2(start.x, start.y), egui::pos2(end.x, end.y)],
                egui::Stroke::new(2.0, egui::Color32::WHITE),
            );

            let mid = egui::pos2((start.x + end.x) / 2.0, (start.y + end.y) / 2.0);

            if let Some(constraint) = polygon.constraints.get(i).copied().flatten() {
                let text = match constraint {
                    ConstraintType::Horizontal => "H".to_string(),
                    ConstraintType::Vertical => "V".to_string(),
                    ConstraintType::Diagonal45 => "D".to_string(),
                    ConstraintType::FixedLength(len) => format!("{:.1}", len),
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
