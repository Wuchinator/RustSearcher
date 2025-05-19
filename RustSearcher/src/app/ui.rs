use super::model::FileSearchApp;
use eframe::egui;

pub fn show_top_panel(app: &mut FileSearchApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Управление исключениями").clicked() {
                app.show_exclude_window = true;
            }
            ui.label(format!("Проиндексировано файлов: {}", app.indexed_count));
        });
    });
}

pub fn show_exclude_window(app: &mut FileSearchApp, ctx: &egui::Context) -> bool {
    let mut should_reindex = false;
    let mut to_remove = None;
    let new_exclude_path = app.new_exclude_path.clone();
    
    egui::Window::new("Исключенные пути")
        .open(&mut app.show_exclude_window)
        .show(ctx, |ui| {
            ui.label("Добавить путь для исключения:");
            ui.text_edit_singleline(&mut app.new_exclude_path);
            
            if ui.button("Добавить").clicked() && !new_exclude_path.is_empty() {
                let mut path = new_exclude_path.trim().to_string();
                if !path.starts_with('/') {
                    path = format!("/{}", path);
                }
                app.excluded_paths.push(path.to_lowercase());
                app.new_exclude_path.clear();
                should_reindex = true;
            }
            
            ui.separator();
            
            ui.label("Текущие исключения:");
            for (i, path) in app.excluded_paths.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(path);
                    if ui.button("❌").clicked() {
                        to_remove = Some(i);
                    }
                });
            }
        });
    
    if let Some(i) = to_remove {
        app.excluded_paths.remove(i);
        should_reindex = true;
    }
    
    should_reindex
}

pub fn show_main_content(app: &mut FileSearchApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        if app.is_indexing {
            ui.spinner();
            ui.label("Идет индексация файлов...");
            return;
        }

        let search_changed = ui.text_edit_singleline(&mut app.search_term).changed();
        
        if search_changed && !app.search_term.is_empty() {
            let search_term = app.search_term.to_lowercase();
            let indexed_files = app.indexed_files.clone();
            let results = app.results.clone();
            
            std::thread::spawn(move || {
                let found: Vec<_> = indexed_files.lock()
                    .unwrap()
                    .iter()
                    .filter(|path| {
                        path.file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| n.to_lowercase().contains(&search_term))
                            .unwrap_or(false)
                    })
                    .take(1000)
                    .cloned()
                    .collect();
                
                *results.lock().unwrap() = found;
            });
        }

        ui.separator();

        if let Ok(results) = app.results.try_lock() {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if app.search_term.is_empty() {
                    ui.label("Введите поисковый запрос");
                } else if results.is_empty() {
                    ui.label("Ничего не найдено");
                } else {
                    ui.label(format!("Найдено: {} файлов", results.len()));
                    for result in results.iter().take(100) {
                        ui.label(result.display().to_string());
                    }
                }
            });
        }
    });
}