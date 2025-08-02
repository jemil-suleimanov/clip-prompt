# Clip Prompt

**Clip Prompt** is a powerful cross-platform desktop application that enhances your text prompts using local AI models. It runs quietly in your system tray and provides instant text enhancement through a global hotkey, making it perfect for writers, developers, students, and anyone who wants to improve their text with AI assistance.

## ✨ Features

### 🚀 Core Functionality
- **Global Hotkey Enhancement**: Press `Cmd+Shift+E` (or `Ctrl+Shift+E`) anywhere to enhance text
- **Smart Clipboard Integration**: Automatically reads, enhances, and replaces clipboard content
- **Local AI Processing**: Uses Ollama for completely offline, private text enhancement
- **System Tray Operation**: Runs quietly in the background with easy tray access
- **Cross-Platform Support**: Works seamlessly on macOS, Windows, and Linux

### ⚙️ Advanced Features
- **Custom System Prompts**: Fully customizable AI behavior and enhancement style
- **Model Selection**: Choose from any Ollama-compatible AI model
- **Autostart Support**: Configure the app to start automatically with your system
- **Real-time Status Monitoring**: Live connection status for Ollama and model availability
- **Visual Notifications**: Native system notifications for process feedback
- **Settings Persistence**: All preferences saved automatically

### 🎯 User Experience
- **One-Click Enhancement**: Simple copy → hotkey → paste workflow
- **Smart Error Handling**: Helpful notifications and instructions for common issues
- **Ollama Integration Guide**: Built-in installation instructions for different platforms
- **Reset to Default**: Easy restoration of default settings
- **Hide to Tray**: Minimize to system tray without closing

## 🚀 Quick Start

### 1. Install Ollama
**Clip Prompt requires Ollama to be installed and running.**

#### macOS Installation:
```bash
# Using Homebrew (recommended)
brew install ollama

# Or download from website
# Visit https://ollama.ai/download
```

#### Windows Installation:
- Download from [ollama.ai/download](https://ollama.ai/download)
- Run the installer and follow the setup wizard

#### Linux Installation:
```bash
# Using curl
curl -fsSL https://ollama.ai/install.sh | sh

# Or download from website
# Visit https://ollama.ai/download
```

### 2. Install AI Models
```bash
# Start Ollama
ollama serve

# Install a model (in a new terminal)
ollama pull mistral:7b    # Good balance of speed and quality
# or
ollama pull qwen2.5:7b    # Fast and efficient
# or
ollama pull llama2:7b     # Classic option
```

### 3. Install Clip Prompt

#### Option 1: Download from GitHub Releases
Download the latest release for your platform from the [GitHub Releases page](https://github.com/jemil-suleimanov/clip-prompt/releases/latest).

**Note for macOS users**: Since we don't have Apple Developer certificates, you may see security warnings when installing. This is normal for unsigned apps. You can still run the app by right-clicking and selecting "Open" or going to System Preferences → Security & Privacy → General and clicking "Open Anyway".

#### Option 2: Build from Source
```bash
# Clone the repository
git clone https://github.com/your-username/clip-prompt.git
cd clip-prompt

# Install dependencies
npm install

# Build for your platform
npm run tauri build
```

**Note for macOS users**: Since we don't have Apple Developer certificates, you may see security warnings when installing. This is normal for unsigned apps. You can still run the app by right-clicking and selecting "Open" or going to System Preferences → Security & Privacy → General and clicking "Open Anyway".

### 4. Start Using
1. **Select text** in any application
2. **Copy it** (`Cmd+C` / `Ctrl+C`)
3. **Press hotkey** (`Cmd+Shift+E` / `Ctrl+Shift+E`)
4. **Paste enhanced text** (`Cmd+V` / `Ctrl+V`)

## 📖 Detailed Usage Guide

### Basic Workflow
The core workflow is simple and consistent:

1. **Select & Copy**: Highlight any text in any application and copy it
2. **Enhance**: Press the global hotkey (`Cmd+Shift+E` / `Ctrl+Shift+E`)
3. **Paste**: The enhanced text is automatically placed in your clipboard

**Important**: You must manually copy the text first using `Cmd+C` / `Ctrl+C` before pressing the hotkey.

### System Tray Interface
- **Left-click tray icon**: Opens the main settings window
- **Right-click tray icon**: Shows context menu with options:
  - "Show Window" - Open the main interface
  - "Quit" - Exit the application

### Main Application Window

#### Quick Start Guide
The top section shows your current shortcut and provides a clear 4-step workflow:
1. Select the text you want to enhance
2. Copy it to clipboard (`Cmd+C` / `Ctrl+C`)
3. Press the global shortcut (`Cmd+Shift+E` / `Ctrl+Shift+E`)
4. Paste the enhanced text (`Cmd+V` / `Ctrl+V`)

#### Status Overview
Real-time status indicators for:
- **Ollama Connection**: Shows if Ollama is running and accessible
- **Autostart Status**: Indicates if the app starts automatically with your system
- **Model Status**: Shows which AI model is currently selected and available

#### Settings Panel

##### AI Model Selection
- **Dropdown menu**: Choose from available Ollama models
- **Auto-detection**: App automatically detects and lists available models
- **Dynamic loading**: Models are loaded on startup and can be refreshed
- **Fallback handling**: Graceful handling when no models are available

##### System Prompt Customization
- **Textarea editor**: Modify how the AI enhances your text
- **Default prompt visible**: See the original system prompt as a starting point
- **Real-time saving**: Changes are saved automatically as you type
- **Reset to Default**: One-click restoration of the original prompt
- **Persistent storage**: Custom prompts are saved between sessions

**Example customizations:**
- **More formal tone**: "You are a professional business consultant..."
- **Creative writing**: "You are a creative writing assistant..."
- **Technical focus**: "You are a software development expert..."
- **Educational style**: "You are a patient teacher explaining concepts..."

##### Autostart Configuration
Platform-specific instructions for enabling/disabling autostart:

**macOS:**
- System Preferences → Users & Groups → Login Items
- Add/remove Clip Prompt from the list

**Windows:**
- Task Manager → Startup tab
- Enable/disable Clip Prompt

**Linux:**
- Desktop environment settings → Startup Applications
- Add/remove Clip Prompt

#### Manual Enhancement
For testing and direct enhancement:
- **Input field**: Type or paste text directly
- **Enhance button**: Process the text immediately
- **Output display**: See the enhanced result
- **Copy button**: Copy enhanced text to clipboard
- **Clear button**: Reset both input and output fields

### Notifications
The app provides helpful feedback through system notifications:
- 🤖 "Enhancing your text..." when processing starts
- ✅ "Text enhanced! Press Cmd+V to paste" when complete
- 📋 "Please copy text (Cmd+C), then try again" if clipboard is empty
- ❌ Error notifications with specific guidance if something goes wrong
- 🚫 "Ollama Not Installed" with installation instructions if needed

## 🔧 Configuration

### Model Management
- **Automatic detection**: App finds available models on startup
- **Dynamic selection**: Change models without restarting
- **Fallback handling**: Uses first available model if preferred isn't found
- **Error recovery**: Clear error messages when models are unavailable

### System Prompt Customization
The system prompt controls how the AI enhances your text. You can:
- **View the default**: See exactly how the original prompt works
- **Modify behavior**: Change the AI's tone, style, or focus
- **Add examples**: Include specific examples for your use case
- **Language support**: Create prompts in different languages
- **Reset anytime**: Return to the default prompt with one click

### Autostart Setup
Configure the app to start automatically with your system:
- **Platform-specific instructions**: Clear guidance for each OS
- **Easy enable/disable**: Simple toggle in the settings
- **Background operation**: Runs silently in the tray

## 🛠️ Troubleshooting

### Common Issues

#### "Ollama Not Installed" Warning
**Solution**: Install Ollama following the built-in instructions
1. Click "Show Instructions" in the warning
2. Follow the platform-specific installation steps
3. Install a model: `ollama pull mistral:7b`
4. Restart Clip Prompt

#### "Failed to connect to Ollama"
**Solutions**:
- Ensure Ollama is running: `ollama serve`
- Check if models are available: `ollama list`
- Verify Ollama is accessible at `http://localhost:11434`
- Restart Ollama if it becomes unresponsive

#### "No models available"
**Solutions**:
- Install a model: `ollama pull mistral:7b`
- Check Ollama is running: `ollama serve`
- Verify model installation: `ollama list`

#### "Hotkey not working"
**Solutions**:
- Check if another app is using the same hotkey
- Try restarting the application
- On macOS, ensure accessibility permissions are granted
- Verify the app is running (check system tray)

#### "Enhancement takes too long"
**Solutions**:
- Try a smaller/faster model: `ollama pull qwen2.5:0.5b`
- Check system resources (CPU, memory)
- Restart Ollama if it becomes unresponsive
- Consider upgrading your hardware

#### "No text in clipboard"
**Solutions**:
- Make sure you copy text before pressing the hotkey
- The app only works with text content (not images or files)
- Try copying the text again and then pressing the hotkey

### Performance Tips
- **Use smaller models** for faster processing (qwen2.5:0.5b, gemma:2b)
- **Close unused applications** to free up system resources
- **Restart Ollama periodically** if it becomes slow
- **Use SSD storage** for better model loading performance

## 🏗️ Development

### Prerequisites
- **Rust**: Install from [rustup.rs](https://rustup.rs)
- **Node.js**: Version 18+ from [nodejs.org](https://nodejs.org)
- **Tauri CLI**: Install with `npm install -g @tauri-apps/cli`

### Building from Source
```bash
# Clone the repository
git clone <repository-url>
cd clip-prompt

# Install dependencies
npm install

# Development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### Architecture
- **Backend**: Rust with Tauri framework
- **Frontend**: HTML, CSS, JavaScript
- **AI Integration**: HTTP communication with Ollama API
- **System Integration**: Global hotkeys, clipboard, notifications, tray

## 📦 Distribution

### Build Artifacts
- **macOS**: DMG installer and App bundle
- **Windows**: MSI installer
- **Linux**: AppImage (portable)

### System Requirements
- **macOS**: 10.13+ (High Sierra)
- **Windows**: Windows 10+
- **Linux**: Most modern distributions
- **RAM**: 4GB+ (8GB+ recommended for larger models)
- **Storage**: 2GB+ free space for models

### Releases and Versioning
We use semantic versioning (MAJOR.MINOR.PATCH) for releases. To create a new release:

1. **Update version** in `package.json` and `src-tauri/tauri.conf.json`
2. **Create a git tag**: `git tag v1.0.0`
3. **Push the tag**: `git push origin v1.0.0`
4. **GitHub Actions** will automatically build and create a release

The workflow builds for all platforms simultaneously and uploads the artifacts to the GitHub release.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🤝 Contributing

See [DEVELOPMENT.md](DEVELOPMENT.md) for detailed development setup and contribution guidelines.

## 🆘 Support

- **Issues**: Report bugs and request features on GitHub
- **Discussions**: Ask questions and share tips in GitHub Discussions
- **Documentation**: Check the built-in help and this README

---

**Clip Prompt** - Enhance your text with local AI power! 🚀
