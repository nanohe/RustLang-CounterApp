use eframe::egui;

fn main() -> eframe::Result<()> {
    // println!("Hello, world!");
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "egui Counter App",
        options,
        Box::new(|_cc| Ok(Box::new(CounterApp::default()))),
    )

}

#[derive(Default)]
struct CounterApp {
    count: i32,
}

impl eframe::App for CounterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Counter App");
            ui.label(format!("Count: {}", self.count));

            ui.horizontal(|ui| {
                if ui.button("Increment").clicked() {
                    self.count += 1;
                }
                if ui.button("Decrement").clicked() {
                    self.count -= 1;
                }
            });
        });
    }
}