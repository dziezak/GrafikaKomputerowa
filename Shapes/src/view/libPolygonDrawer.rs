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

    fn draw_arc(
        painter: &egui::Painter,
        center: Point,
        radius: f32,
        start: Point,
        end: Point,
        clockwise: bool,
    ) {
        let start_angle = (start.y - center.y).atan2(start.x - center.x);
        let end_angle = (end.y - center.y).atan2(end.x - center.x);

        let mut angle_diff = if clockwise {
            start_angle - end_angle
        } else {
            end_angle - start_angle
        };

        if angle_diff < 0.0 {
            angle_diff += TAU;
        }

        let steps = 40;
        let mut last = start;
        for i in 1..=steps {
            let t = i / steps;
            let angle = if clockwise {
                start_angle - t as f32 * angle_diff
            } else {
                start_angle + t as f32 * angle_diff
            };

            let p = Point {
                x: center.x + radius * angle.cos(),
                y: center.y + radius * angle.sin(),
            };

            painter.line_segment(
                [egui::pos2(last.x as f32, last.y as f32), egui::pos2(p.x as f32, p.y as f32)],
                egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
            );

            last = p;
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

            match polygon.edge_types[i] {
                EdgeType::Line => {
                    painter.line_segment(
                        [egui::pos2(start.x, start.y), egui::pos2(end.x, end.y)],
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    );
                }
                EdgeType::Arc { center, radius, clockwise, .. } => {
                    Self::draw_arc(painter, center, radius, *start, *end, clockwise);
                }
                _ => {
                    // Bezier na razie pomijamy
                    todo!();
                }
            }



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
