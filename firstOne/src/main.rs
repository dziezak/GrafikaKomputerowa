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
        });
    }
}