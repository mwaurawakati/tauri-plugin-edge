use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Edge;
#[cfg(mobile)]
use mobile::Edge;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the edge APIs.
pub trait EdgeExt<R: Runtime> {
  fn edge(&self) -> &Edge<R>;
}

impl<R: Runtime, T: Manager<R>> crate::EdgeExt<R> for T {
  fn edge(&self) -> &Edge<R> {
    self.state::<Edge<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("edge")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let edge = mobile::init(app, api)?;
      #[cfg(desktop)]
      let edge = desktop::init(app, api)?;
      app.manage(edge);
      Ok(())
    })
    .build()
}
