 Project name is Clip Prompt
 
 LEM (Local Enhanced Model) is a cross‑platform desktop tray application that improves user‑written prompts.
A user highlights any text anywhere (web browser, editor, chat app), presses a global hotkey, and LEM instantly rewrites the highlighted text into a clearer, more explicit prompt using a locally running language model.
Privacy is preserved because all processing happens offline on the user’s machine.
 
 ┌─────────────────────────────────────────────────────┐
 │                  User Environment                    │
 │ ┌───────────────┐      ┌──────────────────────────┐ │
 │ │ Any App (Web, │ Ctrl+C │ Highlighted text copied │ │
 │ │ Editor, etc.) │──────▶│ to Clipboard            │ │
 │ └───────────────┘      └──────────────────────────┘ │
 └─────────────────────────────────────────────────────┘
                        │
                        ▼ (hotkey triggers)
 ┌─────────────────────────────────────────────────────┐
 │                   LEM Tray App                      │
 │  (Tauri + Rust)                                     │
 │                                                     │
 │  - Global Hotkey Listener                           │
 │  - Clipboard Handler                                │
 │  - REST Client to Local Model                       │
 │  - Minimal Settings UI (tray menu)                  │
 │                                                     │
 │  [Backend Rust Commands]                            │
 │     │                                               │
 │     ▼                                               │
 │  Call Ollama API (localhost) with prompt text       │
 │     │                                               │
 │     ▼                                               │
 │  Receive enhanced prompt                            │
 │     │                                               │
 │     ▼                                               │
 │  Replace clipboard content with enhanced text       │
 └─────────────────────────────────────────────────────┘
                        │
                        ▼
 ┌─────────────────────────────────────────────────────┐
 │                 Local Model Server                  │
 │      (Ollama or llama.cpp with GGUF model)          │
 │                                                     │
 │  - Runs in background on CPU/GPU                    │
 │  - Hosts quantized model (3B–7B for speed)          │
 │  - Processes text and returns improved prompt       │
 └─────────────────────────────────────────────────────┘

## Key Components
  ### Frontend/UI:

    - Tauri tray window with minimal settings (optional: select model, autostart toggle).

  ### Backend Service:

    - Rust commands handle clipboard I/O, hotkey events, and model requests.

  ### Model Serving:

    - Ollama or llama.cpp backend serving a small, quantized model (GGUF).

### OS Integration:

    - Global hotkey registration.

    - Autostart on login.

    - Cross‑platform packaging (Tauri Bundler → .msi, .dmg, .AppImage).
  

====

## Workflow
    - User highlights text.
    - Presses hotkey (e.g., Ctrl+Shift+E).
    - LEM copies text → sends to local model → gets enhanced version.
    - Enhanced prompt is instantly placed back into clipboard, ready to paste.