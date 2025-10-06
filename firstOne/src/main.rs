use eframe::{egui, App, Frame, NativeOptions};

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

            if (ui).button("Nie klikaj mnie").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
        });
    }
}