use crate::view::IPolygonDrawer::IPolygonDrawer;
use egui::{Painter, Pos2};
use crate::geometry::polygon::{Polygon, ConstraintType};
use eframe::egui;
use eframe::epaint::{Color32, Stroke};
use crate::geometry::point;
use crate::geometry::point::Point;

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
        arc_angle: f32, // np. std::f32::consts::PI dla półokręgu
        color: Color32,
        thickness: f32,
    ) {todo!();}

    fn compute_arc_geometry(
        start: Point,
        end: Point,
        tangent_start: Option<Point>, // punkt kierunku dla G1 start
        tangent_end: Option<Point>,   // punkt kierunku dla G1 end
        g1_start: bool,
        g1_end: bool,
    ) -> (Point, f32) { todo!();}


}

