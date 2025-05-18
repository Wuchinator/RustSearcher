use eframe::egui;
use std::path::{PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use walkdir::WalkDir;

struct FileSearchApp {
    search_term: String,
    results: Arc<Mutex<Vec<PathBuf>>>,
    indexed_files: Arc<Mutex<Vec<PathBuf>>>,
    is_indexing: bool,
    indexed_count: usize,
}

impl Default for FileSearchApp {
    fn default() -> Self {
        let indexed_files = Arc::new(Mutex::new(Vec::new()));
        let results = Arc::new(Mutex::new(Vec::new()));
        
        let files_clone = indexed_files.clone();
        thread::spawn(move || {
            let home_dir = dirs::home_dir().expect("Не удалось найти домашнюю директорию");
            let skip_dirs = [
                "/proc", "/sys", "/dev", "/run", "/var",
                "/.cache", "/.npm", "/.gradle", "/.m2", "/.ivy2",
                "/.rustup", "/.cargo/registry",
                "/.local/share/Trash",
            ];

            for entry in WalkDir::new(&home_dir)
                .into_iter()
                .filter_entry(|e| {
                    let path = e.path();
                    !skip_dirs.iter().any(|dir| path.starts_with(dir)) &&
                    !path.to_string_lossy().contains("/Cache/") &&
                    !path.to_string_lossy().contains("/.cache/")
                })
                .filter_map(|e| e.ok())
            {
                if let Ok(mut files) = files_clone.lock() {
                    files.push(entry.path().to_path_buf());
                }
            }
        });

        Self {
            search_term: String::new(),
            results,
            indexed_files,
            is_indexing: true,
            indexed_count: 0,
        }
    }
}

impl eframe::App for FileSearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(files) = self.indexed_files.lock() {
            self.indexed_count = files.len();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if self.is_indexing {
                ui.spinner();
                ui.label(format!("Индексация файлов... (найдено {})", self.indexed_count));
                if self.indexed_count > 0 {
                    self.is_indexing = false;
                }
                return;
            }

            let search_changed = ui.text_edit_singleline(&mut self.search_term).changed();
            
            if search_changed && !self.search_term.is_empty() {
                let search_term = self.search_term.to_lowercase();
                let indexed_files = self.indexed_files.clone();
                let results = self.results.clone();
                
                thread::spawn(move || {
                    let found: Vec<PathBuf> = if let Ok(files) = indexed_files.lock() {
                        files.iter()
                            .filter(|path| {
                                path.file_name()
                                    .and_then(|n| n.to_str())
                                    .map(|n| n.to_lowercase().contains(&search_term))
                                    .unwrap_or(false)
                            })
                            .take(1000)
                            .cloned()
                            .collect()
                    } else {
                        Vec::new()
                    };
                    if let Ok(mut res) = results.lock() {
                        *res = found;
                    }
                });
            }

            ui.separator();

            if let Ok(results) = self.results.try_lock() {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    if self.search_term.is_empty() {
                        ui.label(format!(
                            "Готов к поиску ({} файлов проиндексировано)", 
                            self.indexed_count
                        ));
                    } else if results.is_empty() {
                        ui.label("Ничего не найдено");
                    } else {
                        ui.label(format!("Найдено: {} файлов (показано {})", results.len(), results.len().min(100)));
                        for result in results.iter().take(100) {
                            ui.label(result.display().to_string());
                        }
                    }
                });
            } else {
                ui.spinner();
                ui.label("Processing...");
            }
        });

        ctx.request_repaint();
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    
    if let Err(e) = eframe::run_native(
        "Searcher",
        options,
        Box::new(|_cc| Box::new(FileSearchApp::default())),
    ) {
        eprintln!("Ошибка при запуске приложения: {}", e);
        std::process::exit(1);
    }
}