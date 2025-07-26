# Clip Prompt

LEM (Local Enhanced Model) is a cross-platform desktop tray application that improves user-written prompts using a locally running language model.

## Features

- **🚀 Global Hotkey**: Press `Cmd+Shift+E` (or `Ctrl+Shift+E`) anywhere to enhance text
- **📋 Clipboard Integration**: Automatically reads, enhances, and replaces clipboard content
- **🔔 Visual Notifications**: Native system notifications for process feedback
- **🎯 System Tray**: Runs quietly in the background with tray access
- **🤖 Local AI**: Uses Ollama for completely offline text enhancement
- **⚡ Cross-Platform**: Works on macOS, Windows, and Linux

## How It Works

1. **Select & Copy**: Highlight any text in any application and copy it (`Cmd+C` / `Ctrl+C`)
2. **Enhance**: Press the global hotkey (`Cmd+Shift+E` / `Ctrl+Shift+E`) 
3. **Paste**: The enhanced text is automatically placed in your clipboard - just paste (`Cmd+V` / `Ctrl+V`)

**Important**: You must manually copy the text first using `Cmd+C` / `Ctrl+C` before pressing the hotkey.

The app provides visual feedback through system notifications:
- 🤖 "Enhancing your text..." when processing starts
- ✅ "Text enhanced! Press Cmd+V to paste" when complete
- 📋 "Please select some text and press Cmd+C first" if clipboard is empty
- ❌ Error notifications if something goes wrong

## Prerequisites

### Ollama Setup
1. **Install Ollama**: Download from [ollama.ai](https://ollama.ai)
2. **Install a model**: Run `ollama pull mistral:7b` (or any preferred model)
3. **Start Ollama**: Run `ollama serve` or ensure it's running as a service

### Development Prerequisites
- **Rust**: Install from [rustup.rs](https://rustup.rs)
- **Node.js**: Version 18+ from [nodejs.org](https://nodejs.org)
- **Tauri CLI**: Install with `npm install -g @tauri-apps/cli`

## Installation

### Option 1: Download Pre-built Release (Recommended)
1. Download the latest release for your platform:
   - **macOS**: Download the `.dmg` file
   - **Windows**: Download the `.msi` installer  
   - **Linux**: Download the `.AppImage` file

2. **macOS Installation**:
   - Open the `.dmg` file
   - Drag "Clip Prompt" to your Applications folder
   - Launch from Applications or Spotlight

3. **Windows Installation**:
   - Run the `.msi` installer
   - Follow the installation wizard
   - Launch from Start Menu or Desktop shortcut

4. **Linux Installation**:
   - Make the `.AppImage` executable: `chmod +x ClipPrompt.AppImage`
   - Run directly: `./ClipPrompt.AppImage`

### Option 2: Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd clip-prompt

# Install dependencies
npm install

# Build for production
npm run tauri build
```

**Build artifacts will be created in**:
- **macOS**: `src-tauri/target/release/bundle/dmg/` (DMG installer) and `src-tauri/target/release/bundle/macos/` (App bundle)
- **Windows**: `src-tauri/target/release/bundle/msi/` (MSI installer)
- **Linux**: `src-tauri/target/release/bundle/appimage/` (AppImage)

## Usage

### First Launch
1. **Start the app** - it will appear in your system tray
2. **Check Ollama connection** - the app will test connectivity on startup
3. **Configure settings** (optional) - click the tray icon to open settings

### Daily Usage
1. **Select text** in any application (web browser, text editor, chat app, etc.)
2. **Copy it** with `Cmd+C` / `Ctrl+C`
3. **Press the hotkey** `Cmd+Shift+E` / `Ctrl+Shift+E`
4. **Wait for notification** - you'll see "Enhancing your text..."
5. **Paste enhanced text** with `Cmd+V` / `Ctrl+V` when you see "Text enhanced!"

### System Tray
- **Left-click** the tray icon to show/hide the main window
- **Right-click** for context menu with options:
  - "Show Window" - Open the main interface
  - "Quit" - Exit the application

## Keyboard Shortcuts

### Global (System-wide)
- `Cmd+Shift+E` / `Ctrl+Shift+E` - **Enhance clipboard text**

### Application Window
- `Ctrl+Enter` - Enhance the text in the input field
- `Ctrl+Shift+C` - Copy enhanced text to clipboard
- `Escape` - Clear input and output fields

## Distribution & Packaging

### Build Information
- **Binary Size**: ~15MB (release build)
- **DMG Package Size**: ~5.2MB (compressed)
- **Minimum macOS**: 10.13 (High Sierra)
- **Bundle Identifier**: `com.clip-prompt.app`

### Installation Locations
- **macOS**: `/Applications/Clip Prompt.app`
- **Windows**: `C:\Program Files\Clip Prompt\`
- **Linux**: User-defined location (portable AppImage)

### Supported Platforms
- macOS (Apple Silicon & Intel)
- Windows (x64)
- Linux (x64, AppImage format)

## Architecture

### Backend (Rust/Tauri)
- **Global Hotkey Management**: Cross-platform hotkey registration
- **Clipboard Operations**: Read/write system clipboard
- **HTTP Client**: Communication with Ollama API
- **System Tray**: Background operation with minimal UI
- **Notifications**: Native system notification display

### Frontend (HTML/CSS/JavaScript)
- **Settings Interface**: Model selection and configuration
- **Manual Enhancement**: Direct text input/output for testing
- **Status Display**: Connection status and operation feedback

### External Dependencies
- **Ollama**: Local AI model server (user-installed)
- **AI Models**: Any Ollama-compatible model (mistral:7b recommended)

## Troubleshooting

### Common Issues

**"Failed to connect to Ollama"**
- Ensure Ollama is installed and running: `ollama serve`
- Check if models are available: `ollama list`
- Verify Ollama is accessible at `http://localhost:11434`

**"Hotkey not working"**
- Check if another app is using the same hotkey
- Try restarting the application
- On macOS, ensure accessibility permissions are granted

**"No text in clipboard"**
- Make sure you copy text before pressing the hotkey
- The app only works with text content (not images or files)

**"Enhancement takes too long"**
- Try a smaller/faster model like `qwen2.5:0.5b`
- Check if your system has sufficient resources
- Restart Ollama if it becomes unresponsive

### Logs & Debugging
- **Development**: Run with `npm run dev` to see console output
- **Production**: Check system logs or run from terminal to see output

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development setup and contribution guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.
