// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg(mobile)]

use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, PluginHandle, TauriPlugin},
    Manager, Runtime,
};
use serde_json::Value;
/// The identifier for the Android plugin.
#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.edge";

/// Safe Area configuration options.
#[derive(Debug, Deserialize, Serialize)]
pub struct SafeAreaConfig {
    /// Offset for the safe area.
    #[serde(default)]
    pub offset: i32,
    /// Additional appearance configurations.
    #[serde(flatten)]
    pub appearance: AppearanceConfig,
}



/// Configuration for appearance-related settings.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppearanceConfig {
    /// Whether custom colors should be applied to system bars.
    #[serde(default = "default_custom_colors_for_system_bars")]
    pub custom_colors_for_system_bars: bool,
    /// The color of the status bar (default: `#000000`).
    #[serde(default = "default_status_bar_color")]
    pub status_bar_color: String,
    /// The content style of the status bar (e.g., `light` or `dark`).
    #[serde(default = "default_status_bar_content")]
    pub status_bar_content: String,
    /// The color of the navigation bar (default: `#000000`).
    #[serde(default = "default_navigation_bar_color")]
    pub navigation_bar_color: String,
    /// The content style of the navigation bar (e.g., `light` or `dark`).
    #[serde(default = "default_navigation_bar_content")]
    pub navigation_bar_content: String,
}

impl AppearanceConfig {
    /// Creates a new `AppearanceConfig` from an optional `serde_json::Value`.
    pub fn from_json(value: Option<&Value>) -> Self {
        if let Some(json) = value {
            serde_json::from_value(json.clone()).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            custom_colors_for_system_bars: default_custom_colors_for_system_bars(),
            status_bar_color: default_status_bar_color(),
            status_bar_content: default_status_bar_content(),
            navigation_bar_color: default_navigation_bar_color(),
            navigation_bar_content: default_navigation_bar_content(),
        }
    }
}

/// Default values for fields
fn default_custom_colors_for_system_bars() -> bool {
    true
}

fn default_status_bar_color() -> String {
    "#000000".to_string()
}

fn default_status_bar_content() -> String {
    "light".to_string()
}

fn default_navigation_bar_color() -> String {
    "#000000".to_string()
}

fn default_navigation_bar_content() -> String {
    "light".to_string()
}


/// Represents the SafeArea plugin interface.
pub struct SafeArea<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> SafeArea<R> {
    /// Enable the safe area configuration.
    pub fn enable(&self, config: SafeAreaConfig) -> tauri::Result<()> {
        self.0.run_mobile_plugin("enable", config)?;
        Ok(())
    }

    /// Disable the safe area configuration.
    pub fn disable(&self, config: AppearanceConfig) -> tauri::Result<()> {
        self.0.run_mobile_plugin("disable", config)?;
        Ok(())
    }
}

/// Extension trait for accessing the `SafeArea` API from the Tauri app context.
pub trait SafeAreaExt<R: Runtime> {
    fn safe_area(&self) -> &SafeArea<R>;
}

impl<R: Runtime, T: Manager<R>> SafeAreaExt<R> for T {
    fn safe_area(&self) -> &SafeArea<R> {
        self.state::<SafeArea<R>>().inner()
    }
}

/// Initializes the Safe Area plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("safearea")
        .setup(|app, api| {
            #[cfg(target_os = "android")]
            let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "SafeAreaPlugin")?;
            #[cfg(target_os = "ios")]
            let handle = api.register_ios_plugin(init_plugin_safe_area)?; // Add iOS binding if applicable.
            app.manage(SafeArea(handle));
            Ok(())
        })
        .build()
}
