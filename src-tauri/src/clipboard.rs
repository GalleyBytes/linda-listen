use crate::error::{AppError, AppResult};
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::{process::Command, thread, time::Duration};
use tauri::AppHandle;

pub fn copy_text(text: &str) -> AppResult<()> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(text.to_owned())?;
    Ok(())
}

fn paste_clipboard_impl() -> AppResult<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|err| AppError::Automation(err.to_string()))?;
    enigo
        .key(Key::Meta, Direction::Press)
        .map_err(|err| AppError::Automation(err.to_string()))?;
    enigo
        .key(Key::Unicode('v'), Direction::Click)
        .map_err(|err| AppError::Automation(err.to_string()))?;
    enigo
        .key(Key::Meta, Direction::Release)
        .map_err(|err| AppError::Automation(err.to_string()))?;

    Ok(())
}

fn is_simulate_input_permission_error(message: &str) -> bool {
    let message = message.to_ascii_lowercase();
    message.contains("permission to simulate input")
}

#[cfg(target_os = "macos")]
fn paste_clipboard_with_osascript() -> AppResult<()> {
    let output = Command::new("osascript")
        .args([
            "-e",
            r#"tell application "System Events" to keystroke "v" using command down"#,
        ])
        .output()?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let detail = if !stderr.is_empty() {
        stderr
    } else if !stdout.is_empty() {
        stdout
    } else {
        format!("exit status {}", output.status)
    };

    Err(AppError::Automation(format!("AppleScript paste failed: {detail}")))
}

#[cfg(not(target_os = "macos"))]
fn paste_clipboard_with_osascript() -> AppResult<()> {
    Err(AppError::Automation(
        "AppleScript paste fallback is only available on macOS".to_owned(),
    ))
}

pub async fn paste_clipboard_on_main_thread(app: &AppHandle) -> AppResult<()> {
    thread::sleep(Duration::from_millis(80));

    let (tx, rx) = tokio::sync::oneshot::channel::<Result<(), String>>();
    app.run_on_main_thread(move || {
        let result = paste_clipboard_impl().map_err(|err| err.to_string());
        let _ = tx.send(result);
    })
    .map_err(|err| AppError::Message(err.to_string()))?;

    match rx
        .await
        .map_err(|_| AppError::Message("failed to receive paste result".to_owned()))?
    {
        Ok(()) => Ok(()),
        Err(message) if is_simulate_input_permission_error(&message) => paste_clipboard_with_osascript(),
        Err(message) => Err(AppError::Message(message)),
    }
}

#[cfg(test)]
mod tests {
    use super::is_simulate_input_permission_error;

    #[test]
    fn detects_macos_input_simulation_permission_errors() {
        assert!(is_simulate_input_permission_error(
            "The application does not have the permission to simulate input!"
        ));
        assert!(!is_simulate_input_permission_error(
            "something unrelated happened"
        ));
    }
}
