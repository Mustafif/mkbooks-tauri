#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowUrl};

fn main() {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let filemenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
  let max = CustomMenuItem::new("max".to_string(), "Maximize");
  let min = CustomMenuItem::new("min".to_string(), "Minimize");
  let opsmenu = Submenu::new(
    "Options",
    Menu::new().add_item(max).add_item(min),
  );
  let menu = Menu::new().add_submenu(filemenu).add_submenu(opsmenu);
  tauri::Builder::default()
    .menu(menu)
    .on_menu_event(|event| match event.menu_item_id() {
      "quit" => {
        std::process::exit(0);
      }
      "close" => {
        event.window().close().unwrap();
      }
      "max" => {
        event.window().maximize().unwrap();
      }
      "min" => {
        event.window().minimize().unwrap();
      }
      _ => std::process::exit(1),
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
