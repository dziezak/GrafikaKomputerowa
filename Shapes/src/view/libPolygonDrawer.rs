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



    fn draw_arc_between_points(
        &self,
        painter: &Painter,
        p1: Pos2,
        p2: Pos2,
        arc_angle: f32, // np. std::f32::consts::PI dla półokręgu
        color: Color32,
        thickness: f32,
    ) {
        // Środek między punktami
        let mid = Pos2::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0);

        // Wektor między punktami
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let chord_length = (dx * dx + dy * dy).sqrt();

        // Promień okręgu
        let radius = chord_length / (2.0 * (arc_angle / 2.0).sin());

        // Kąt kierunku między punktami
        let chord_angle = dy.atan2(dx);

        // Kąt prostopadły do cięciwy
        let perp_angle = chord_angle + std::f32::consts::FRAC_PI_2;

        // Odległość od środka cięciwy do środka okręgu
        let h = (radius * radius - (chord_length / 2.0).powi(2)).sqrt();

        // Środek okręgu
        let center = Pos2::new(mid.x + h * perp_angle.cos(), mid.y + h * perp_angle.sin());

        // Kąty startowy i końcowy względem środka
        let start_angle = (p1.y - center.y).atan2(p1.x - center.x);
        let end_angle = (p2.y - center.y).atan2(p2.x - center.x);

        // Rysowanie łuku jako linii z punktów
        let segments = 100;
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let angle = start_angle + t * (end_angle - start_angle);
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            let pos = Pos2::new(x, y);
            painter.circle_filled(pos, thickness, color);
        }
    }


    fn draw_arc(
        &self,
        painter: &egui::Painter,
        start: crate::geometry::point::Point,
        end: crate::geometry::point::Point,
        center: crate::geometry::point::Point,
        radius: f32,
        stroke: egui::Stroke,
        clockwise: bool,
    ) {
        // liczba segmentów łuku
        let segments = 32;
        let start_angle = (start.y - center.y).atan2(start.x - center.x);
        let end_angle = (end.y - center.y).atan2(end.x - center.x);

        // obliczamy sweep w odpowiednim kierunku
        let mut sweep = end_angle - start_angle;
        if clockwise && sweep > 0.0 {
            sweep -= std::f32::consts::TAU;
        } else if !clockwise && sweep < 0.0 {
            sweep += std::f32::consts::TAU;
        }

        let dt = sweep / segments as f32;
        let mut prev = start;
        for i in 1..=segments {
            let angle = start_angle + dt * i as f32;
            let next = crate::geometry::point::Point {
                x: center.x + radius * angle.cos(),
                y: center.y + radius * angle.sin(),
            };
            painter.line_segment(
                [egui::pos2(prev.x, prev.y), egui::pos2(next.x, next.y)],
                stroke,
            );
            prev = next;
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
                    // 1. Obliczamy center i radius łuku
                    let (center, radius) = polygon.compute_default_arc(*start, *end);

                    // 2. Obliczamy kąty start i end względem środka
                    let start_angle = (*start - center).y.atan2((*start - center).x);
                    let end_angle = (*end - center).y.atan2((*end - center).x);

                    // 3. Rysujemy łuk wywołując algorytm Midpoint + draw_pixel
                    self.draw_arc_between_points(
                        painter,                            // referencja do egui::Painter
                        Pos2::new(start.x, start.y),        // pierwszy punkt łuku
                        Pos2::new(end.x, end.y),            // drugi punkt łuku
                        std::f32::consts::PI,               // kąt łuku w radianach (np. półokrąg)
                        Color32::RED,                       // kolor łuku
                        1.0,                                // grubość punktów (lub linii)
                    );


                    //eprint!("Rysujemy okrag");
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

impl PolygonDrawer {
    fn draw_pixel(painter: &egui::Painter, x: i32, y: i32, color: egui::Color32) {
        let size = 2.0;
        painter.rect_filled(
            egui::Rect::from_min_size(egui::pos2(x as f32, y as f32), egui::vec2(size, size)),
            0.0,
            color,
        );
    }
}