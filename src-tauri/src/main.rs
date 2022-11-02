#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder,
};

fn main() {
  let tray = SystemTray::new()
    .with_id("main")
    .with_menu(SystemTrayMenu::new().add_item(CustomMenuItem::new("fucking_open", "Open")));

  let app = tauri::Builder::default()
    .system_tray(tray)
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        app.get_window("search").unwrap().show().unwrap();
      }
      // The other events will never trigger, thanks DBusMenu
      _ => {}
    })
    .build(tauri::generate_context!())
    .expect("error while building tauri application");

  let window = WindowBuilder::new(
    &app,
    "search",
    tauri::WindowUrl::External("http://localhost:1420".parse().unwrap()),
  )
  .always_on_top(true)
  .visible(false)
  .decorations(false)
  .skip_taskbar(true)
  .disable_file_drop_handler()
  .focus()
  .resizable(false)
  .fullscreen(false)
  .transparent(true)
  .inner_size(800.0, 480.0)
  .build()
  .unwrap();

  app.run(|_app_handle, event| match event {
    RunEvent::ExitRequested { api, .. } => {
      api.prevent_exit();
    }
    _ => {}
  });
}
