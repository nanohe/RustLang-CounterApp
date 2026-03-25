mod app;
mod counter;

use app::CounterApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui Counter App",
        options,
        Box::new(|_cc| Ok(Box::new(CounterApp::default()))),
    )
}
