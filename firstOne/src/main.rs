use eframe::{egui, App, Frame, NativeOptions};
use eframe::egui::{Color32, Stroke, Pos2};
use eframe::egui::accesskit::Point;

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Edytor wielokątów",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

#[derive(Default)]
struct MyApp {
    counter: i32,
    points: Vec<Pos2>,
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Witaj w edytorze wielokątów!");
            ui.label("To jest podstawowe okno aplikacji.");

            if ui.button("Kliknij mnie!").clicked() {
                self.counter += 1;
            }

            ui.label(format!("Kliknięcia: {}", self.counter));

            ui.separator();

            let canvas_size = egui::Vec2::new(600.0, 400.0);
            let(rect, response) = ui.allocate_exact_size(canvas_size, egui::Sense::click_and_drag());

            let painter = ui.painter();

            painter.rect_stroke(rect,0.0, Stroke::new(2.0, Color32::LIGHT_BLUE));

            if response.dragged(){
                if let Some(pos) = response.interact_pointer_pos(){
                    if rect.contains(pos){
                        self.points.push(pos);
                    }
                }
            }

            for &pos in &self.points {
                painter.circle_filled(pos, 3.0, Color32::RED);
            }

        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.separator();
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.button("Zamknij aplikacje").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                })
            });

            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    if ui.button("Resetuj obrazek").clicked(){
                        self.points.clear();
                    }
                })
            });
        });
    }
}