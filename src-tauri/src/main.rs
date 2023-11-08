#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

//use tauri::api::shell;
use tauri::{
  WindowBuilder,
  WindowUrl,
};

fn main() {
  let ctx = tauri::generate_context!();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![])
    .setup(|app| {
      let _window = WindowBuilder::new(app, "main", WindowUrl::default())
        .title("cuttlefish")
        .inner_size(700.0, 700.0)
        .min_inner_size(400.0, 400.0)
        .build()
        .expect("Unable to create window");
      Ok(())
    })
    .run(ctx)
    .expect("error while running tauri application");
}
