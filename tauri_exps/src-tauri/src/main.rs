mod phi3;
mod token_output_stream;
mod utils;

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {

    let mut  model = phi3::phi3::init();
    let out = model.generate("you are helpful ai agent who is going to answer user queries be polite and precise.".to_string());
    format!("Hello, {}! You've been greeted from Rust!, {:?}", name,out )
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
