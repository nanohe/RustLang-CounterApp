use eframe::egui;

use crate::counter::{load, save, Category, Counter, SAVE_FILE};

#[derive(Default)]
pub struct CounterApp {
    counters: Vec<Counter>,
    new_name: String,
    new_category: Category,
    show_debug: bool,
    status: Option<String>,
}

impl eframe::App for CounterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Counter App");

            // --- Add new counter ---
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_name);

                egui::ComboBox::from_id_salt("category")
                    .selected_text(self.new_category.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.new_category, Category::Daily, Category::Daily.to_string());
                        ui.selectable_value(&mut self.new_category, Category::Weekly, Category::Weekly.to_string());
                        ui.selectable_value(&mut self.new_category, Category::Unlimited, Category::Unlimited.to_string());
                    });

                if ui.button("Add Counter").clicked() && !self.new_name.is_empty() {
                    self.counters.push(Counter::new(self.new_name.clone(), self.new_category));
                    self.new_name.clear();
                }
            });

            ui.separator();

            // --- Render each counter ---
            let mut to_remove: Option<usize> = None;

            for (i, counter) in self.counters.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(counter.to_string());
                    if ui.button("+").clicked() { counter.increment(); }
                    if ui.button("-").clicked() { counter.decrement(); }
                    if ui.button("Remove").clicked() { to_remove = Some(i); }
                });
            }

            if let Some(i) = to_remove {
                self.counters.remove(i);
            }

            ui.separator();

            // --- Save / Load ---
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    self.status = match save(&self.counters) {
                        Ok(()) => Some(format!("Saved to {}", SAVE_FILE)),
                        Err(e) => Some(format!("Save error: {}", e)),
                    };
                }
                if ui.button("Load").clicked() {
                    self.status = match load() {
                        Ok(counters) => {
                            self.counters = counters;
                            Some(format!("Loaded from {}", SAVE_FILE))
                        }
                        Err(e) => Some(format!("Load error: {}", e)),
                    };
                }
            });

            if let Some(msg) = &self.status {
                ui.label(msg);
            }

            ui.separator();

            // --- Debug panel ---
            ui.checkbox(&mut self.show_debug, "Show debug info");
            if self.show_debug {
                for counter in &self.counters {
                    ui.monospace(format!("{:?}", counter));
                }
            }
        });
    }
}
