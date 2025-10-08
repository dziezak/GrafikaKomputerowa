use egui::{Painter, Color32, Pos2, Stroke, Align2};
use crate::geometry::polygon::{Polygon, ConstraintType};
use eframe::egui;
use crate::view::IPolygonDrawer::IPolygonDrawer;

pub struct PolygonDrawer;

impl PolygonDrawer {
    pub fn new() -> Self{
        Self
    }
}

impl IPolygonDrawer for PolygonDrawer {
    fn draw(&self, painter: &egui::Painter, polygon: &mut Polygon) {
        // Rysujemy kweawdzie
        polygon.ensure_constraints_len();
        for (i, window) in polygon.vertices.windows(2).enumerate() {
            painter.line_segment(
                [egui::pos2(window[0].x, window[0].y), egui::pos2(window[1].x, window[1].y)],
                egui::Stroke::new(2.0, egui::Color32::WHITE),
            );

            let mid = egui::pos2(
                (window[0].x + window[1].x) / 2.0,
                (window[0].y + window[1].y) / 2.0,
            );

            if let Some(constraint) = polygon.constraints[i] {
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

        if polygon.vertices.len() > 2 {
            let first = polygon.vertices[0];
            let last = polygon.vertices.last().unwrap();
            painter.line_segment(
                [egui::pos2(first.x, first.y), egui::pos2(last.x, last.y)],
                egui::Stroke::new(2.0, egui::Color32::WHITE),
            );
        }

        //rysowanie wierzchołków
        for v in &polygon.vertices {
            painter.circle_filled(egui::pos2(v.x, v.y), 5.0, egui::Color32::RED);
        }
    }
}