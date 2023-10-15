// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod tabs;

use std::{str::FromStr, sync::Mutex};

use tabs::{tab::Tab, tab_content::Content, tab_manager::Manager};
use tauri::Menu;

struct TabState {
    tab_manager: Mutex<Manager>,
}

#[tauri::command]
fn create_new_tab(
    state: tauri::State<TabState>,
    title: String,
    content: String,
) -> Result<usize, ()> {
    Ok(state
        .tab_manager
        .lock()
        .unwrap()
        .create_tab(title, Content::from_str(&content).unwrap()))
}

#[tauri::command]
fn close_tab(state: tauri::State<TabState>, id: usize) {
    state.tab_manager.lock().unwrap().close_tab_by_id(id);
}

#[tauri::command]
fn get_tabs(state: tauri::State<TabState>) -> Result<Vec<Tab>, ()> {
    let tabs = &state.tab_manager.lock().unwrap().tabs;
    Ok(tabs.to_vec())
}

fn main() {
    let context = tauri::generate_context!();

    let menu = Menu::os_default(&context.package_info().name);

    tauri::Builder::default()
        .manage(TabState {
            tab_manager: Manager::new().into(),
        })
        .invoke_handler(tauri::generate_handler![
            create_new_tab,
            close_tab,
            get_tabs
        ])
        .menu(menu)
        .run(context)
        .expect("error while running tauri application");
}
