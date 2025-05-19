use super::config::AppConfig;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use walkdir::WalkDir;

pub struct FileSearchApp {
    pub search_term: String,
    pub results: Arc<Mutex<Vec<PathBuf>>>,
    pub indexed_files: Arc<Mutex<Vec<PathBuf>>>,
    pub is_indexing: bool,
    pub indexed_count: usize,
    pub show_exclude_window: bool,
    pub excluded_paths: Vec<String>,
    pub new_exclude_path: String,
}

impl FileSearchApp {
    pub fn start_indexing(&mut self) {
        let excluded_paths = self.excluded_paths.clone();
        let indexed_files = self.indexed_files.clone();
        
        self.indexed_files.lock().unwrap().clear();
        self.is_indexing = true;
        
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
                    let path_str = path.to_string_lossy().to_lowercase();
                    
                    !skip_dirs.iter().any(|dir| path.starts_with(dir)) &&
                    !path_str.contains("/cache/") &&
                    !excluded_paths.iter().any(|excluded| {
                        let excluded = excluded.to_lowercase();
                        path_str.contains(&excluded) || path.starts_with(excluded)
                    })
                })
                .filter_map(|e| e.ok())
            {
                indexed_files.lock().unwrap().push(entry.path().to_path_buf());
            }
        });
    }

    pub fn load_config(&mut self) {
        if let Some(config) = AppConfig::load() {
            self.excluded_paths = config.excluded_paths;
        }
    }
    
    pub fn save_config(&self) {
        let config = AppConfig {
            excluded_paths: self.excluded_paths.clone(),
        };
        let _ = config.save();
    }
}

impl Default for FileSearchApp {
    fn default() -> Self {
        let indexed_files = Arc::new(Mutex::new(Vec::new()));
        let results = Arc::new(Mutex::new(Vec::new()));
        
        let mut app = Self {
            search_term: String::new(),
            results,
            indexed_files,
            is_indexing: true,
            indexed_count: 0,
            show_exclude_window: false,
            excluded_paths: vec![
                "/proc".to_string(),
                "/sys".to_string(),
                "/dev".to_string(),
                "/run".to_string(),
                "/var".to_string(),
            ],
            new_exclude_path: String::new(),
        };
        
        app.load_config();
        app.start_indexing();
        app
    }
}