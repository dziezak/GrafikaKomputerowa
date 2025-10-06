use eframe::{egui, App};
use crate::geometry::polygon::Polygon;
use crate::geometry::point::Point;
use crate::editor::selection::Selection;

pub struct PolygonApp {
    polygon: Polygon,
    selection: Selection,
}

impl Default for PolygonApp {
    fn default() -> Self {
        let polygon = Polygon::new(vec![
            Point { x: 100.0, y: 100.0 },
            Point { x: 200.0, y: 100.0 },
            Point { x: 200.0, y: 200.0 },
            Point { x: 100.0, y: 200.0 },
        ]);
        Self {
            polygon,
            selection: Selection::new(),
        }
    }
}

impl App for PolygonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(
            ctx,
            |ui| {
                let canvas_size = egui::Vec2::new(600.0, 400.0);
                let (rect, response) = ui.allocate_exact_size(canvas_size, egui::Sense::click_and_drag());
                let painter = ui.painter();
                painter.rect(
                    rect,
                    egui::Rounding::ZERO,
                    egui::Color32::TRANSPARENT,
                    egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
                    egui::StrokeKind::Inside,
                );


                // Obsługa kliknięcia/podciągnięcia wierzchołka
                if response.dragged_by(egui::PointerButton::Primary) {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let mouse_point = Point { x: pos.x, y: pos.y };
                        if self.selection.selected_vertex.is_none() {
                            self.selection.select_vertex(&self.polygon, mouse_point, 10.0);
                        }
                        if let Some(idx) = self.selection.selected_vertex {
                            let dx = pos.x - self.polygon.vertices[idx].x;
                            let dy = pos.y - self.polygon.vertices[idx].y;
                            self.polygon.move_vertex(idx, dx, dy);
                        }
                    }
                } else {
                    self.selection.selected_vertex = None;
                }


                if response.hovered() && response.clicked_by(egui::PointerButton::Secondary) {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let mouse_point = Point { x: pos.x, y: pos.y };

                        self.selection.select_vertex(&self.polygon, mouse_point, 10.0);
                        if let Some(idx) = self.selection.selected_vertex {
                            self.polygon.remove_vertex(idx);
                            self.selection.selected_vertex = None;
                        } else if let Some(edge_idx) = self.selection.select_edge(
                            &self.polygon,
                            &mouse_point,
                            10.0
                        ) {
                            let next_idx = (edge_idx + 1) % self.polygon.vertices.len();
                            self.polygon.add_vertex_mid_edge(edge_idx, next_idx);
                        }
                    }
                }




                // Rysowanie wielokąta
                for window in self.polygon.vertices.windows(2) {
                    painter.line_segment(
                        [egui::pos2(window[0].x, window[0].y), egui::pos2(window[1].x, window[1].y)],
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    );
                }
                // zamknięcie wielokąta
                if self.polygon.vertices.len() > 2 {
                    let first = &self.polygon.vertices[0];
                    let last = &self.polygon.vertices[self.polygon.vertices.len()-1];
                    painter.line_segment(
                        [egui::pos2(first.x, first.y), egui::pos2(last.x, last.y)],
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    );
                }

                // rysowanie wierzchołków
                for v in &self.polygon.vertices {
                    painter.circle_filled(egui::pos2(v.x, v.y), 5.0, egui::Color32::RED);
                }
            }
        );

        // Panel boczny z informacją o wybranym wierzchołku
        egui::SidePanel::right("sidebar").show(ctx, |ui| {
            ui.heading("Wybrany wierzchołek");
            if let Some(idx) = self.selection.selected_vertex {
                let v = &self.polygon.vertices[idx];
                ui.label(format!("Index: {}", idx));
                ui.label(format!("Pozycja: ({:.1}, {:.1})", v.x, v.y));
            } else {
                ui.label("Brak wybranego wierzchołka");
            }
        });
    }
}
