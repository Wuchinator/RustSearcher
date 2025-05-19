mod app;

use app::FileSearchApp;
use eframe;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Searcher",
        options,
        Box::new(|_cc| Box::<FileSearchApp>::default()),
    ).unwrap();
}