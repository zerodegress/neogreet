use std::path::PathBuf;

use clap::Parser;
use tauri::{Manager, WebviewWindowBuilder};

mod neogreet;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let cli = neogreet::cli::Cli::parse();

  tauri::Builder::default()
    .setup(|app| {
      let _main_window =
        WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App("index.html".into()))
          .data_directory(PathBuf::from("/tmp/neogreetcache"))
          .decorations(false)
          .fullscreen(true)
          .build()
          .expect("create main window failed");
      app.manage(cli);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      neogreet::neogreet_login,
      neogreet::neogreet_defaults,
      neogreet::neogreet_available_desktops,
      neogreet::neogreet_desktops_name_map,
      neogreet::neogreet_available_x11_desktops,
      neogreet::neogreet_available_wayland_desktops,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
