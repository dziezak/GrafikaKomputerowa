use crate::view::IPolygonDrawer::IPolygonDrawer;
use std::thread::sleep;
use eframe::{egui, App};
use crate::geometry::polygon::{Polygon, ConstraintType};
use crate::geometry::point::Point;
use crate::editor::selection::Selection;
use crate::view::{PolygonDrawer};

pub struct PolygonApp {
    polygon: Polygon,
    selection: Selection,
    drawer: Box<dyn IPolygonDrawer>,
    show_context_menu: bool,
    context_pos: egui::Pos2,
    clicked_vertex: Option<usize>,
    clicked_edge: Option<usize>,
    show_constraint_submenu: bool,
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
            drawer: Box::new(PolygonDrawer::new()),
            show_context_menu: false,
            context_pos: egui::pos2(0.0, 0.0),
            clicked_vertex: None,
            clicked_edge: None,
            show_constraint_submenu: false,
        }
    }
}

impl App for PolygonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx,|ui| {
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
                            self.polygon.apply_constraints();
                        }
                    }
                } else {
                    self.selection.selected_vertex = None;
                }

                if response.clicked_by(egui::PointerButton::Secondary){
                    if let Some(pos) = response.interact_pointer_pos() {
                        let mouse_point = Point { x: pos.x, y: pos.y };

                        self.clicked_vertex = self.selection.select_vertex(&self.polygon, mouse_point, 10.0);
                        self.clicked_edge = self.selection.select_edge(&self.polygon, &mouse_point, 10.0);
                        self.context_pos = pos;
                        self.show_context_menu = true;
                        self.show_constraint_submenu = false;
                    }
                }

            self.drawer.draw(&painter, &mut self.polygon);


            if self.show_context_menu {
                egui::Area::new(egui::Id::new("context_menu"))
                    .fixed_pos(self.context_pos)
                    .show(ctx, |ui| {
                        egui::Frame::popup(&ctx.style()).show(ui, |ui| {
                            ui.set_width(220.0);
                            ui.vertical_centered(|ui| {
                                ui.heading("opcje");
                                ui.separator();

                                if let Some(v_idx) = self.clicked_vertex {
                                    if ui.button("usun wierzcholek").clicked(){
                                        self.polygon.remove_vertex(v_idx);
                                        self.show_context_menu = false;
                                        self.polygon.apply_constraints();
                                    }
                                    if ui.button("Ustaw caiglasc w wierzcholku").clicked(){
                                        todo!();
                                        self.show_context_menu = false;
                                        self.polygon.apply_constraints();
                                    }
                                }else if let Some(e_idx) = self.clicked_edge {
                                    if ui.button("dodaj wierzcholek").clicked(){
                                        self.polygon.add_vertex_mid_edge(e_idx, e_idx+1); //TODO tutaj jest problem bo nie da sie dodac wiierzcholka za ostatnia krawedzia
                                        self.show_context_menu = false;
                                        self.polygon.apply_constraints();
                                    }
                                    if ui.button("dodaj ograniczenie").clicked(){
                                        self.show_constraint_submenu = !self.show_constraint_submenu;
                                        self.polygon.apply_constraints();
                                        //self.show_context_menu = false;
                                    }
                                    if ui.button("usun ograniczenie").clicked(){
                                        if e_idx < self.polygon.constraints.len(){
                                            self.polygon.constraints[e_idx] = None;
                                        }
                                        self.show_context_menu = false;
                                        self.polygon.apply_constraints();
                                    }
                                    if ui.button("uzyj antyaliasingu").clicked(){
                                        //todo!();
                                        eprintln!("egui robi to automatycznie!");
                                        self.polygon.apply_constraints();
                                        self.show_context_menu = false;
                                    }
                                }
                                if ui.button("Anuluj").clicked() {
                                    self.show_context_menu = false;
                                }
                            });

                            if self.show_constraint_submenu {
                                ui.separator();
                                ui.label("Dodaj ograniczenie:");
                                if let Some(e_idx) = self.clicked_edge {
                                    if ui.button("Pozioma (H)").clicked(){
                                        self.polygon.constraints[e_idx] = Some(ConstraintType::Horizontal);
                                        self.show_context_menu = false;
                                        self.polygon.apply_constraints();
                                    }
                                    if ui.button("Skosna (D)").clicked(){
                                        self.polygon.constraints[e_idx] = Some(ConstraintType::Diagonal45);
                                        self.show_context_menu = false;
                                        self.polygon.apply_constraints();
                                    }
                                    if ui.button("Dlugosc stala").clicked(){
                                        let start = &self.polygon.vertices[e_idx];
                                        let end = &self.polygon.vertices[e_idx+1 % self.polygon.vertices.len()];
                                        let dx = end.x - start.x;
                                        let dy = end.y - start.y;
                                        let length = (dx * dx + dy * dy).sqrt();
                                        self.polygon.constraints[e_idx] = Some(ConstraintType::FixedLength(length as f64)); // tutaj mega jest ten jezyk!
                                        self.show_context_menu = false;
                                        self.polygon.apply_constraints();
                                    }
                                }
                            }
                        });
                    });

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
