use std::cmp::PartialEq;
use crate::view::IPolygonDrawer::IPolygonDrawer;
use std::thread::sleep;
use eframe::{egui, App};
use crate::geometry::polygon::{Polygon, ConstraintType};
use crate::geometry::point::Point;
use crate::editor::selection::Selection;
use crate::view::{libPolygonDrawer, PolygonDrawer};

#[derive(PartialEq, Eq)]
pub enum DrawMode {
    Library,
    Bresenham,
}


#[derive()]
pub struct PolygonApp {
    polygon: Polygon,
    selection: Selection,
    drawer: Box<dyn IPolygonDrawer>,
    draw_mode: DrawMode,
    show_context_menu: bool,
    context_pos: egui::Pos2,
    clicked_vertex: Option<usize>,
    clicked_edge: Option<usize>,
    show_constraint_submenu: bool,
    length_input: Option<f32>,
    length_edge_idx: Option<usize>,
    is_dragging_polygon: bool,
    last_mouse_pos: Option<egui::Pos2>,

    show_warning_popup: bool,
    warning_text: String,
    show_help_window: bool,
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
            draw_mode: DrawMode::Library,
            show_context_menu: false,
            context_pos: egui::pos2(0.0, 0.0),
            clicked_vertex: None,
            clicked_edge: None,
            show_constraint_submenu: false,
            length_input: None,
            length_edge_idx: None,
            is_dragging_polygon: false,
            last_mouse_pos: None,

            show_warning_popup: false,
            warning_text: String::new(),

            show_help_window: false,
        }
    }
}

impl App for PolygonApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Tryb rysowania");
                if ui.radio(self.draw_mode == DrawMode::Library, "Bibliotekowa implementacja").clicked() {
                    self.draw_mode = DrawMode::Library;
                    self.drawer = Box::new(PolygonDrawer::new());
                }
                if ui.radio(self.draw_mode == DrawMode::Bresenham, "Moja implementacja").clicked() {
                    self.draw_mode = DrawMode::Bresenham;
                    self.drawer = Box::new(crate::view::myPolygonDrawer::MyPolygonDrawer::new());
                }
                ui.separator();
                if ui.button("Pomoc").clicked(){
                    self.show_help_window = true;
                }
            });
        });

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

                    if self.selection.selected_vertex.is_none() && !self.is_dragging_polygon {
                        if self.selection.select_vertex(&self.polygon, mouse_point, 10.0).is_none() {
                            self.is_dragging_polygon = true;
                            self.last_mouse_pos = Some(pos);
                        }
                    }

                    if let Some(idx) = self.selection.selected_vertex {
                        let dx = pos.x - self.polygon.vertices[idx].x;
                        let dy = pos.y - self.polygon.vertices[idx].y;
                        self.polygon.move_vertex(idx, dx, dy);
                        self.polygon.apply_constraints();
                    }
                    else if self.is_dragging_polygon {
                        if let Some(last_pos) = self.last_mouse_pos {
                            let dx = pos.x - last_pos.x;
                            let dy = pos.y - last_pos.y;
                            for v in &mut self.polygon.vertices {
                                v.x += dx;
                                v.y += dy;
                            }
                            self.last_mouse_pos = Some(pos);
                        }
                    }
                }
            } else {
                self.selection.selected_vertex = None;
                self.is_dragging_polygon = false;
                self.last_mouse_pos = None;
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
                                        let constraint = ConstraintType::Horizontal;
                                        if self.polygon.is_constraint_legal(e_idx, &constraint) {
                                            self.polygon.constraints[e_idx] = Some(ConstraintType::Horizontal);
                                            self.polygon.apply_constraints();
                                        }else{
                                            self.warning_text = "Nie można ustawić: sasiednia krawedz jest juz pozioma".to_string();
                                            self.show_warning_popup = true;
                                        }
                                        self.show_context_menu = false;
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

                                        self.length_input = Some(length);//okienko
                                        self.length_edge_idx = Some(e_idx); //index do okienka

                                        self.show_context_menu = false;
                                        self.polygon.apply_constraints();
                                    }
                                    if ui.button("Łuk").clicked(){
                                        let constraint = ConstraintType::Arc{
                                            g1_start: false,
                                            g1_end: false,
                                        };
                                        self.polygon.constraints[e_idx] = Some(constraint);
                                        self.polygon.apply_constraints();
                                        eprint!("context menu zareagowalo");
                                    }
                                    if let Some(ConstraintType::Arc { ref mut g1_start, ref mut g1_end})=
                                        self.polygon.constraints[e_idx]
                                    {
                                        ui.separator();
                                        ui.label("Ustaw ciaglosc luku:");
                                        if ui.button("Przelacz G1 start").clicked(){
                                            *g1_start = !*g1_start;
                                        }
                                        if ui.button("Przelacz G1 end").clicked() {
                                            *g1_end = !*g1_end;
                                        }
                                    }
                                }
                            }
                        });
                    });

            }

            if let Some(edge_idx) = self.length_edge_idx {
                egui::Window::new("Ustaw dlugosc krawedzi")
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        ui.label(format!("Aktualna dlugosc: {:2}", self.length_input.unwrap_or(0.0)));

                        let mut value = self.length_input.unwrap_or(0.0);
                        if ui.add(egui::DragValue::new(&mut value).speed(1.0)).changed(){
                            self.length_input = Some(value);
                        }

                        if ui.button("Zastosuj").clicked(){
                            self.polygon.constraints[edge_idx] = Some(ConstraintType::FixedLength((self.length_input.unwrap()) as f64));
                            self.length_input = None;
                            self.length_edge_idx = None;

                            self.polygon.apply_constraints();
                        }

                        if ui.button("Anuluj").clicked(){
                            self.length_input = None;
                            self.length_edge_idx = None;
                        }
                    });
            }

            if self.show_warning_popup {
                egui::Window::new("Blad ograniczenia")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.label(&self.warning_text);
                        ui.add_space(10.0);
                        if ui.button("OK").clicked(){
                            self.show_warning_popup = false;
                        }
                    });
            }

            if self.show_help_window {
                egui::Window::new("Pomoc")
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                    .show(ctx, |ui| {
                        ui.label("Pomoc");
                        ui.hyperlink_to("Kliknij po więcej pomocy",
                                        "https://www.youtube.com/watch?v=xvFZjo5PgG0");

                        ui.add_space(10.0);
                        ui.label("Program umożliwia tworzenie i edycję wielokątów z ograniczeniami geometrycznymi (H, V, 45°, długość). Kliknij wierzchołek, aby go przesunąć, lub w krawędź, aby dodać nowy punkt. Kliknięcie prawym przyciskiem myszy otwiera menu z opcjami (dodaj, usuń, nadaj ograniczenie itp.). Dwie sąsiednie krawędzie nie mogą być jednocześnie poziome lub pionowe. Krawędzie mogą być także krzywymi Beziera trzeciego stopnia z punktami kontrolnymi. W wierzchołkach można ustawiać klasę ciągłości (G0, G1, C1) między segmentami. Cały wielokąt można przesuwać przeciągając tło. Po każdej zmianie program automatycznie wymusza zgodność z ograniczeniami.");
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(5.0);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui.button("OK").clicked() {
                                self.show_help_window = false;
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
