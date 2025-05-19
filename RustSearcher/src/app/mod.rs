pub mod config;
pub mod model;
pub mod ui;

pub use model::FileSearchApp;
pub use ui::{show_top_panel, show_exclude_window, show_main_content};

impl eframe::App for FileSearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(files) = self.indexed_files.lock() {
            self.indexed_count = files.len();
            if self.is_indexing && !files.is_empty() {
                self.is_indexing = false;
            }
        }
        
        let should_reindex = if self.show_exclude_window {
            show_exclude_window(self, ctx)
        } else {
            false
        };
        
        if should_reindex {
            self.start_indexing();
        }

        show_top_panel(self, ctx);
        show_main_content(self, ctx);
    }

    fn on_exit(&mut self, _gl_ctx: Option<&eframe::glow::Context>) {
        self.save_config();
    }
}