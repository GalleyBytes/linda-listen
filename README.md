# Linda Listen

macOS dictation app for local transcription, automatic model download, clipboard copy, and auto-paste.

## Run

```sh
npm install
npm run tauri dev
```

## Build

```sh
npm run tauri build
```

## Setup

- The app downloads the default speech model assets into Application Support automatically.
- Use the custom model directory only if you want to override the app-managed cache.
- Leave Gemini cleanup disabled unless you want optional post-processing.
- Leave the default hotkey or change it to match your workflow. Option+Space is the default, and F18 is also bound as a fallback.
- The standalone app requests microphone access on first launch; grant it in System Settings if macOS prompts. Grant Accessibility too if you want auto-paste to work, and allow Linda Listen to control System Events if macOS asks. Auto-paste is skipped while the app window is focused.

## Notes

- Local transcription uses the bundled local speech model pipeline.
- Gemini cleanup is optional and off by default.
- The prototype stores the API key in the macOS Keychain when available.
