use tauri::Manager;
use tauri_plugin_notification::NotificationExt;
use tauri::{menu::{Menu, MenuItem}, tray::{MouseButton, TrayIconBuilder, TrayIconEvent}, WindowEvent};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::PathBuf;
use std::fs;
use reqwest;
use anyhow::Result;
use log::{info, error, debug};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[derive(Debug, Serialize, Deserialize)]
struct OllamaResponse {
    model: String,
    created_at: String,
    response: String,
    done: bool,
    done_reason: Option<String>,
    context: Option<Vec<i32>>,
    total_duration: Option<i64>,
    load_duration: Option<i64>,
    prompt_eval_count: Option<i32>,
    prompt_eval_duration: Option<i64>,
    eval_count: Option<i32>,
    eval_duration: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

struct AppState {
    ollama_url: String,
    model_name: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            ollama_url: "http://localhost:11434".to_string(),
            model_name: "mistral:7b".to_string(), // Changed to an available model
        }
    }
}

#[tauri::command]
async fn enhance_prompt(prompt: String, state: tauri::State<'_, AppState>) -> Result<String, String> {
    debug!("Enhance prompt called with: {}", prompt);
    
    let system_prompt = r#"<system_prompt>
YOU ARE A LOCAL PROMPT ENHANCER RUNNING ENTIRELY ON THE USER'S MACHINE.  
YOUR EXCLUSIVE MISSION IS TO READ THE USER'S RAW INPUT PROMPT AND REWRITE IT INTO A FAR MORE DETAILED, SPECIFIC, AND HIGH‑QUALITY PROMPT THAT ANOTHER AI ASSISTANT COULD DIRECTLY USE TO PRODUCE THE BEST POSSIBLE OUTPUT.

### INSTRUCTIONS ###
- YOU MUST FULLY PRESERVE THE ORIGINAL INTENT AND MEANING WHILE EXPANDING IT WITH HELPFUL CLARITY AND ADDITIONAL CONTEXT.
- YOU MUST MAKE THE PROMPT MORE EXPLICIT, MORE ACTION‑ORIENTED, AND MORE PROFESSIONAL.
- YOU MUST OUTPUT **ONLY** THE ENHANCED PROMPT — NOTHING ELSE.  
- YOU MUST NEVER EXPLAIN, APOLOGIZE, OR ADD META COMMENTS.
- WHEN THE USER'S INPUT IS VAGUE, YOU MUST INFER AND ADD REASONABLE DETAILS AND PARAMETERS TO MAKE THE PROMPT STRONGER.
- IF NEEDED, ADD DOMAIN‑RELEVANT CONSTRAINTS, OBJECTIVES, OR EDGE‑CASE CONSIDERATIONS THAT WOULD HELP ANOTHER AI TO PERFORM BETTER.
- ALWAYS RETURN A SINGLE COMPLETE REWRITTEN PROMPT, READY FOR DIRECT USE.

### CHAIN OF THOUGHTS ###
FOLLOW THESE STEPS INTERNALLY BEFORE YOU PRODUCE THE OUTPUT:
1. **UNDERSTAND**: READ the raw input and IDENTIFY the user's goal or intent.
2. **BASICS**: EXTRACT the core subject, task, and domain.
3. **BREAK DOWN**: SPLIT the user's intent into sub‑tasks or dimensions that can be clarified or expanded.
4. **ANALYZE**: CONSIDER what details, constraints, parameters, or examples would make the prompt richer and more actionable.
5. **BUILD**: REWRITE the input prompt into a single, clear, detailed instruction that includes these improvements.
6. **EDGE CASES**: THINK of special conditions or clarifications that might help prevent ambiguous interpretation, and include them when relevant.
7. **FINAL ANSWER**: OUTPUT ONLY the enhanced prompt — no explanations, no prefixes, no suffixes.

### WHAT NOT TO DO ###
- DO NOT ANSWER THE USER'S ORIGINAL PROMPT.
- DO NOT DESCRIBE WHAT YOU ARE DOING OR HOW YOU IMPROVED IT.
- DO NOT SAY "THE USER WANTS…" OR "HERE IS YOUR IMPROVED PROMPT…"
- DO NOT OUTPUT MULTIPLE VERSIONS OR BULLET LISTS — ONLY ONE FINAL PROMPT.
- DO NOT LEAVE THE PROMPT GENERIC — ALWAYS ADD CLARITY, CONTEXT, AND DETAIL.
- NEVER USE PHRASES LIKE "AS AN AI…" OR "I THINK…"
- NEVER OMIT KEY DETAILS FROM THE USER'S INTENT.
- NEVER ADD IRRELEVANT INFORMATION.

### FEW‑SHOT EXAMPLES ###

**Example 1**
Input: `I want to improve A`
Output: `I want to improve A by integrating B and optimizing C parameters, while also considering D and E to ensure scalability and accuracy.`

**Example 2**
Input: `help me write better marketing copy`
Output: `Write a compelling, high‑conversion marketing copy that highlights product benefits, appeals to target audience pain points, uses persuasive language, and includes clear calls‑to‑action.`

**Example 3**
Input: `make this code better`
Output: `Refactor the following code to improve readability, optimize performance, ensure consistent naming conventions, and handle potential edge cases or errors gracefully.`

**Example 4**
Input: `design me a logo`
Output: `Design a modern, minimalistic logo that reflects innovation and trust, uses a blue and white color palette, and is optimized for both digital and print formats.`

</system_prompt>"#;

    let full_prompt = format!("{}\n\nUser input: {}\n\nEnhanced prompt:", system_prompt, prompt);

    let request = OllamaRequest {
        model: state.model_name.clone(),
        prompt: full_prompt,
        stream: false,
    };

    debug!("Sending request to Ollama: {}/api/generate", state.ollama_url);
    
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/generate", state.ollama_url))
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to send request to Ollama: {}", e);
            format!("Failed to send request: {}", e)
        })?;

    if !response.status().is_success() {
        error!("Ollama API returned error status: {}", response.status());
        return Err(format!("Ollama API error: {}", response.status()));
    }

    let response_text = response.text().await.map_err(|e| {
        error!("Failed to read response text: {}", e);
        format!("Failed to read response: {}", e)
    })?;
    
    debug!("Raw Ollama response: {}", response_text);

    let ollama_response: OllamaResponse = serde_json::from_str(&response_text)
        .map_err(|e| {
            error!("Failed to parse response: {}", e);
            format!("Failed to parse response: {}", e)
        })?;

    debug!("Parsed Ollama response: {:?}", ollama_response);

    Ok(ollama_response.response)
}

#[tauri::command]
async fn test_ollama_connection(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let client = reqwest::Client::new();
    
    debug!("Testing Ollama connection at: {}/api/tags", state.ollama_url);
    
    match client.get(&format!("{}/api/tags", state.ollama_url)).send().await {
        Ok(response) => {
            debug!("Connection test response status: {}", response.status());
            Ok(true)
        },
        Err(e) => {
            error!("Connection test failed: {}", e);
            Err(format!("Connection failed: {}", e))
        },
    }
}

#[tauri::command]
async fn get_available_models(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    
    debug!("Getting available models from: {}/api/tags", state.ollama_url);
    
    let response = client
        .get(&format!("{}/api/tags", state.ollama_url))
        .send()
        .await
        .map_err(|e| format!("Failed to get models: {}", e))?;

    debug!("Models response status: {}", response.status());

    #[derive(Deserialize)]
    struct ModelsResponse {
        models: Vec<ModelInfo>,
    }

    #[derive(Debug, Deserialize)]
    struct ModelInfo {
        name: String,
    }

    let response_text = response.text().await
        .map_err(|e| format!("Failed to get models response text: {}", e))?;
    
    debug!("Models response text: {}", response_text);

    let models_response: ModelsResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse models: {} (response: {})", e, response_text))?;

    debug!("Parsed models: {:?}", models_response.models);

    Ok(models_response.models.into_iter().map(|m| m.name).collect())
}

#[tauri::command]
async fn enable_autostart(app_handle: tauri::AppHandle) -> Result<bool, String> {
    debug!("Enabling autostart...");
    
    match std::env::consts::OS {
        "macos" => enable_autostart_macos(&app_handle),
        "windows" => enable_autostart_windows(&app_handle),
        "linux" => enable_autostart_linux(&app_handle),
        _ => Err("Unsupported operating system".to_string()),
    }
}

#[tauri::command]
async fn disable_autostart() -> Result<bool, String> {
    debug!("Disabling autostart...");
    
    match std::env::consts::OS {
        "macos" => disable_autostart_macos(),
        "windows" => disable_autostart_windows(),
        "linux" => disable_autostart_linux(),
        _ => Err("Unsupported operating system".to_string()),
    }
}

#[tauri::command]
async fn is_autostart_enabled() -> Result<bool, String> {
    debug!("Checking autostart status...");
    
    match std::env::consts::OS {
        "macos" => is_autostart_enabled_macos(),
        "windows" => is_autostart_enabled_windows(),
        "linux" => is_autostart_enabled_linux(),
        _ => Err("Unsupported operating system".to_string()),
    }
}

// macOS autostart implementation
fn enable_autostart_macos(app_handle: &tauri::AppHandle) -> Result<bool, String> {
    let app_name = app_handle.package_info().name.clone();
    let bundle_id = format!("com.{}.{}", app_name, app_name);
    
    // Get the app executable path
    let app_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    
    // Create LaunchAgent plist content
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
</dict>
</plist>"#,
        bundle_id,
        app_exe.to_string_lossy()
    );
    
    // Get user's home directory
    let home_dir = std::env::var("HOME")
        .map_err(|_| "Failed to get home directory".to_string())?;
    
    // Create LaunchAgents directory if it doesn't exist
    let launch_agents_dir = PathBuf::from(&home_dir).join("Library/LaunchAgents");
    fs::create_dir_all(&launch_agents_dir)
        .map_err(|e| format!("Failed to create LaunchAgents directory: {}", e))?;
    
    // Write plist file
    let plist_path = launch_agents_dir.join(format!("{}.plist", bundle_id));
    fs::write(&plist_path, plist_content)
        .map_err(|e| format!("Failed to write plist file: {}", e))?;
    
    // Load the LaunchAgent
    let output = Command::new("launchctl")
        .args(&["load", plist_path.to_string_lossy().as_ref()])
        .output()
        .map_err(|e| format!("Failed to load LaunchAgent: {}", e))?;
    
    if output.status.success() {
        debug!("Autostart enabled for macOS");
        Ok(true)
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to enable autostart: {}", error))
    }
}

fn disable_autostart_macos() -> Result<bool, String> {
    let app_name = "clip-prompt";
    let bundle_id = format!("com.{}.{}", app_name, app_name);
    
    // Get user's home directory
    let home_dir = std::env::var("HOME")
        .map_err(|_| "Failed to get home directory".to_string())?;
    
    let plist_path = PathBuf::from(&home_dir).join(format!("Library/LaunchAgents/{}.plist", bundle_id));
    
    // Unload the LaunchAgent if it exists
    if plist_path.exists() {
        let _output = Command::new("launchctl")
            .args(&["unload", plist_path.to_string_lossy().as_ref()])
            .output()
            .map_err(|e| format!("Failed to unload LaunchAgent: {}", e))?;
        
        // Remove the plist file
        fs::remove_file(&plist_path)
            .map_err(|e| format!("Failed to remove plist file: {}", e))?;
        
        debug!("Autostart disabled for macOS");
    }
    
    Ok(true)
}

fn is_autostart_enabled_macos() -> Result<bool, String> {
    let app_name = "clip-prompt";
    let bundle_id = format!("com.{}.{}", app_name, app_name);
    
    // Get user's home directory
    let home_dir = std::env::var("HOME")
        .map_err(|_| "Failed to get home directory".to_string())?;
    
    let plist_path = PathBuf::from(&home_dir).join(format!("Library/LaunchAgents/{}.plist", bundle_id));
    
    Ok(plist_path.exists())
}

// Windows autostart implementation
fn enable_autostart_windows(_app_handle: &tauri::AppHandle) -> Result<bool, String> {
    // Get the app executable path
    let app_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    
    let app_path_str = app_exe.to_string_lossy();
    
    // Use PowerShell to add registry entry
    let ps_command = format!(
        r#"New-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "ClipPrompt" -Value "{}" -PropertyType String -Force"#,
        app_path_str
    );
    
    let output = Command::new("powershell")
        .args(&["-Command", &ps_command])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell command: {}", e))?;
    
    if output.status.success() {
        debug!("Autostart enabled for Windows");
        Ok(true)
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to enable autostart: {}", error))
    }
}

fn disable_autostart_windows() -> Result<bool, String> {
    // Use PowerShell to remove registry entry
    let ps_command = r#"Remove-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "ClipPrompt" -Force"#;
    
    let _output = Command::new("powershell")
        .args(&["-Command", ps_command])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell command: {}", e))?;
    
    debug!("Autostart disabled for Windows");
    Ok(true)
}

fn is_autostart_enabled_windows() -> Result<bool, String> {
    // Use PowerShell to check if registry entry exists
    let ps_command = r#"Get-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Run" -Name "ClipPrompt" -ErrorAction SilentlyContinue"#;
    
    let output = Command::new("powershell")
        .args(&["-Command", ps_command])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell command: {}", e))?;
    
    Ok(output.status.success())
}

// Linux autostart implementation
fn enable_autostart_linux(_app_handle: &tauri::AppHandle) -> Result<bool, String> {
    // Get the app executable path
    let app_exe = std::env::current_exe()
        .map_err(|e| format!("Failed to get current executable path: {}", e))?;
    
    // Get user's home directory
    let home_dir = std::env::var("HOME")
        .map_err(|_| "Failed to get home directory".to_string())?;
    
    // Create autostart directory if it doesn't exist
    let autostart_dir = PathBuf::from(&home_dir).join(".config/autostart");
    fs::create_dir_all(&autostart_dir)
        .map_err(|e| format!("Failed to create autostart directory: {}", e))?;
    
    // Create desktop entry content
    let desktop_entry = format!(
        r#"[Desktop Entry]
Type=Application
Name=Clip Prompt
Exec={}
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true"#,
        app_exe.to_string_lossy()
    );
    
    // Write desktop entry file
    let desktop_file = autostart_dir.join("clip-prompt.desktop");
    fs::write(&desktop_file, desktop_entry)
        .map_err(|e| format!("Failed to write desktop entry: {}", e))?;
    
    debug!("Autostart enabled for Linux");
    Ok(true)
}

fn disable_autostart_linux() -> Result<bool, String> {
    // Get user's home directory
    let home_dir = std::env::var("HOME")
        .map_err(|_| "Failed to get home directory".to_string())?;
    
    let desktop_file = PathBuf::from(&home_dir).join(".config/autostart/clip-prompt.desktop");
    
    // Remove desktop entry file if it exists
    if desktop_file.exists() {
        fs::remove_file(&desktop_file)
            .map_err(|e| format!("Failed to remove desktop entry: {}", e))?;
    }
    
    debug!("Autostart disabled for Linux");
    Ok(true)
}

fn is_autostart_enabled_linux() -> Result<bool, String> {
    // Get user's home directory
    let home_dir = std::env::var("HOME")
        .map_err(|_| "Failed to get home directory".to_string())?;
    
    let desktop_file = PathBuf::from(&home_dir).join(".config/autostart/clip-prompt.desktop");
    
    Ok(desktop_file.exists())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcuts(["CommandOrControl+Shift+E"])
                .expect("Failed to register global shortcuts")
                .with_handler(|app, shortcut, event| {
                    let shortcut_str = format!("{}", shortcut);
                    
                    // Check for multiple possible formats (Windows/Linux vs macOS)
                    let is_target_hotkey = shortcut_str == "CommandOrControl+Shift+E" || 
                                         shortcut_str == "shift+super+KeyE" ||
                                         shortcut_str == "ctrl+shift+KeyE" ||
                                         shortcut_str == "super+shift+KeyE";
                    
                    if is_target_hotkey && event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        println!("✅ HOTKEY ACTIVATED! Enhancing clipboard text...");
                        info!("Global hotkey Ctrl+Shift+E pressed!");
                        
                        // Show "processing" notification
                        let _ = app.notification()
                            .builder()
                            .title("Clip Prompt")
                            .body("🤖 Enhancing your text...")
                            .show();
                        
                        // Handle the hotkey press asynchronously
                        let app_handle = app.clone();
                        tauri::async_runtime::spawn(async move {
                            let app_handle_clone = app_handle.clone();
                            if let Err(e) = handle_global_hotkey(app_handle).await {
                                println!("❌ Enhancement failed: {}", e);
                                error!("Failed to handle global hotkey: {}", e);
                                
                                // Show error notification
                                let _ = app_handle_clone.notification()
                                    .builder()
                                    .title("Clip Prompt")
                                    .body(&format!("❌ Enhancement failed: {}", e))
                                    .show();
                            }
                        });
                    }
                    // Only show debug info if needed
                    else if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        println!("🔍 DEBUG: Hotkey '{}' detected but not our target", shortcut_str);
                    }
                })
                .build()
        )
        .manage(AppState {
            ollama_url: "http://localhost:11434".to_string(),
            model_name: "mistral:7b".to_string(),
        })
        .invoke_handler(tauri::generate_handler![enhance_prompt, test_ollama_connection, get_available_models, enable_autostart, disable_autostart, is_autostart_enabled])
        .setup(|app| {
            println!("🚀 Setting up Clip Prompt...");
            info!("Clip Prompt started successfully");
            info!("Ready to enhance prompts with Ollama");
            
            // Create system tray menu
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show Window", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            // Create system tray
            let _ = TrayIconBuilder::with_id("main")
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .title("Clip Prompt")
                .tooltip("Clip Prompt - AI Text Enhancer")
                .on_tray_icon_event(|tray, event| {
                    match event {
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            ..
                        } => {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            println!("✅ System tray created successfully");
            println!("📋 Clipboard integration enabled");
            println!("🎯 Ready! Press Cmd+Shift+E (or Ctrl+Shift+E) anywhere to enhance text");
            info!("Global hotkey CommandOrControl+Shift+E registered successfully");

            // Test Ollama connection on startup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                println!("🔍 Testing Ollama connection...");
                let state = app_handle.state::<AppState>();
                if let Err(e) = test_ollama_connection(state).await {
                    println!("❌ Failed to connect to Ollama: {}", e);
                    error!("Failed to connect to Ollama on startup: {}", e);
                } else {
                    println!("✅ Successfully connected to Ollama");
                    info!("Successfully connected to Ollama on startup");
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    // Hide window instead of closing
                    window.hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn handle_global_hotkey(app_handle: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("📋 Reading clipboard...");
    info!("Processing global hotkey - reading clipboard...");
    
    // Read current clipboard content
    let clipboard_text = match app_handle.clipboard().read_text() {
        Ok(text) => {
            println!("📄 Found {} characters: '{}'", text.len(), text.chars().take(50).collect::<String>());
            info!("Clipboard content read: {} characters", text.len());
            text
        },
        Err(e) => {
            println!("❌ Failed to read clipboard: {}", e);
            error!("Failed to read clipboard: {}", e);
            
            // Show helpful notification about what to do
            let _ = app_handle.notification()
                .builder()
                .title("Clip Prompt")
                .body("📋 Please select text and press Cmd+C first, then try again")
                .show();
            
            return Err(format!("Failed to read clipboard: {}", e).into());
        }
    };

    // Skip if clipboard is empty or too short
    if clipboard_text.trim().is_empty() {
        println!("⚠️  Clipboard is empty - please select some text first");
        info!("Clipboard content is empty or whitespace only");
        
        // Show "empty clipboard" notification with helpful instructions
        let _ = app_handle.notification()
            .builder()
            .title("Clip Prompt")
            .body("📋 Please select some text and press Cmd+C first")
            .show();
        
        return Ok(());
    }

    println!("🤖 Enhancing with AI...");
    info!("Enhancing clipboard text...");
    
    // Get app state
    let state = app_handle.state::<AppState>();
    
    // Enhance the prompt
    match enhance_prompt(clipboard_text, state).await {
        Ok(enhanced_text) => {
            println!("✨ Enhanced! Writing {} chars to clipboard...", enhanced_text.len());
            info!("Text enhanced successfully, writing back to clipboard...");
            
            // Write enhanced text back to clipboard
            if let Err(e) = app_handle.clipboard().write_text(enhanced_text) {
                println!("❌ Failed to write to clipboard: {}", e);
                error!("Failed to write enhanced text to clipboard: {}", e);
                return Err(format!("Failed to write to clipboard: {}", e).into());
            }
            
            println!("🎉 Done! Press Cmd+V to paste your enhanced text");
            info!("Enhanced text written to clipboard successfully");
            
            // Show "success" notification
            let _ = app_handle.notification()
                .builder()
                .title("Clip Prompt")
                .body("✅ Text enhanced! Press Cmd+V to paste")
                .show();
        },
        Err(e) => {
            println!("❌ Enhancement failed: {}", e);
            error!("Failed to enhance text: {}", e);
            return Err(format!("Failed to enhance text: {}", e).into());
        }
    }
    
    Ok(())
}
