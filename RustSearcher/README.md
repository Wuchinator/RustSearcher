# Searcher

A fast and lightweight file search application written in Rust. You can easily exclude unnecessary directories and find exactly the file or type you need

## Features
- Fast file indexing
- Real-time search
- Clean GUI interface

## Installation

##  Структура проекта

```text
RustSearcher/
├── src/
│   ├── app/            
│   │   ├── config.rs
│   │   ├── model.rs
│   │   ├── ui.rs
│   │   └── mod.rs
│   ├── searcher_config.json
│   └── main.rs
└── Cargo.toml
```


##Requires
- Rust 1.82+

### From source
```bash
git clone https://github.com/Wuchinator/RustSearcher.git
cd Searcher
cargo build --release
