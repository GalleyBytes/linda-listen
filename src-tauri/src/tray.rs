use tauri::image::Image;
use tauri::tray::TrayIconId;
#[allow(unused_imports)]
use tauri::Manager;

use crate::error::AppError;
use crate::state::AppStatus;

const ICON_BYTES: &[u8] = include_bytes!("../icons/tray-icon.png");

pub const TRAY_ID: &str = "main-tray";

struct Color(u8, u8, u8);

// Apple HIG system colors
const RED: Color = Color(255, 59, 48);
const YELLOW: Color = Color(255, 204, 0);
const GREEN: Color = Color(52, 199, 89);
const ORANGE: Color = Color(255, 149, 0);

pub struct TrayIcons {
    width: u32,
    height: u32,
    idle: Vec<u8>,
    red: Vec<u8>,
    yellow: Vec<u8>,
    green: Vec<u8>,
    orange: Vec<u8>,
}

impl TrayIcons {
    pub fn load() -> crate::error::AppResult<Self> {
        let base = Image::from_bytes(ICON_BYTES)
            .map_err(|e| AppError::Message(format!("failed to decode tray icon: {e}")))?;
        let width = base.width();
        let height = base.height();
        let rgba = base.rgba().to_vec();

        Ok(Self {
            width,
            height,
            idle: rgba.clone(),
            red: colorize(&rgba, &RED),
            yellow: colorize(&rgba, &YELLOW),
            green: colorize(&rgba, &GREEN),
            orange: colorize(&rgba, &ORANGE),
        })
    }

    /// Returns (icon image, is_template) for the given status.
    pub fn for_status(&self, status: AppStatus) -> (Image<'_>, bool) {
        let (rgba, is_template) = match status {
            AppStatus::Idle | AppStatus::Ready => (&self.idle, true),
            AppStatus::Recording => (&self.red, false),
            AppStatus::Downloading | AppStatus::Transcribing => (&self.yellow, false),
            AppStatus::Rewriting => (&self.orange, false),
            AppStatus::Copying | AppStatus::Pasting => (&self.green, false),
            AppStatus::Error => (&self.red, false),
        };
        (Image::new(rgba, self.width, self.height), is_template)
    }

    pub fn idle_icon(&self) -> Image<'_> {
        Image::new(&self.idle, self.width, self.height)
    }
}

/// Update the system tray icon to reflect the current app status.
pub fn update_tray(app: &tauri::AppHandle, icons: &TrayIcons, status: AppStatus) {
    let id: TrayIconId = TRAY_ID.into();
    if let Some(tray) = app.tray_by_id(&id) {
        let (icon, is_template) = icons.for_status(status);
        let _ = tray.set_icon(Some(icon));
        let _ = tray.set_icon_as_template(is_template);
    }
}

/// Reset the tray icon back to the idle template appearance.
pub fn reset_tray_idle(app: &tauri::AppHandle, icons: &TrayIcons) {
    let id: TrayIconId = TRAY_ID.into();
    if let Some(tray) = app.tray_by_id(&id) {
        let _ = tray.set_icon(Some(icons.idle_icon()));
        let _ = tray.set_icon_as_template(true);
    }
}

/// Recolor all non-transparent pixels to the given color, preserving alpha.
fn colorize(rgba: &[u8], color: &Color) -> Vec<u8> {
    let mut out = rgba.to_vec();
    for chunk in out.chunks_exact_mut(4) {
        if chunk[3] > 0 {
            chunk[0] = color.0;
            chunk[1] = color.1;
            chunk[2] = color.2;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colorize_preserves_alpha() {
        // 2 pixels: one opaque black, one fully transparent
        let rgba = vec![0, 0, 0, 255, 0, 0, 0, 0];
        let result = colorize(&rgba, &Color(255, 0, 0));
        assert_eq!(result, vec![255, 0, 0, 255, 0, 0, 0, 0]);
    }

    #[test]
    fn colorize_preserves_partial_alpha() {
        let rgba = vec![0, 0, 0, 128];
        let result = colorize(&rgba, &GREEN);
        assert_eq!(result, vec![52, 199, 89, 128]);
    }

    #[test]
    fn tray_icons_load_succeeds() {
        let icons = TrayIcons::load().unwrap();
        assert!(icons.width > 0);
        assert!(icons.height > 0);
        // Idle and colored variants should differ
        assert_ne!(icons.idle, icons.red);
        assert_ne!(icons.red, icons.green);
    }

    #[test]
    fn status_mapping_returns_template_only_for_idle_and_ready() {
        let icons = TrayIcons::load().unwrap();
        let (_, is_template) = icons.for_status(AppStatus::Idle);
        assert!(is_template);

        let (_, is_template) = icons.for_status(AppStatus::Ready);
        assert!(is_template);

        let (_, is_template) = icons.for_status(AppStatus::Recording);
        assert!(!is_template);
    }
}
