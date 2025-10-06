use eframe::{NativeOptions, run_native};
use crate::app::PolygonApp;

mod app;
mod geometry;
mod editor;
mod rendering;

fn main()-> eframe::Result<()> {
    let options = NativeOptions::default();
    run_native(
        "Edytor Wielokatow",
        options,
        Box::new(|_cc| Ok(Box::new(PolygonApp::default()))),
    )
}