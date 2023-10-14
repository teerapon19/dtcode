// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Menu;

fn main() {
    let context = tauri::generate_context!();

    let menu = Menu::os_default(&context.package_info().name);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .menu(menu)
        .run(context)
        .expect("error while running tauri application");
}
