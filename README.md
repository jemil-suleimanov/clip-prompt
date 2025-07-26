# 🎯 Clip Prompt (LEM)

**Local Enhanced Model** - A cross-platform desktop tray application that improves user-written prompts using local AI models.

## Features

- 🔒 **Privacy-focused**: All processing happens offline on your machine
- ⚡ **Instant enhancement**: Press `Ctrl+Shift+E` to enhance any highlighted text
- 🖥️ **System tray integration**: Runs quietly in the background
- 🌐 **Cross-platform**: Works on Windows, macOS, and Linux
- 🤖 **Local AI models**: Uses Ollama with quantized models for speed
- 📋 **Clipboard integration**: Seamless copy/paste workflow

## How It Works

1. **Highlight text** anywhere (web browser, editor, chat app)
2. **Press `Ctrl+Shift+E`** (global hotkey)
3. **Get enhanced prompt** instantly in your clipboard
4. **Paste** the improved version anywhere

## Prerequisites

### 1. Install Ollama

First, install Ollama on your system:

**macOS/Linux:**
```bash
curl -fsSL https://ollama.ai/install.sh | sh
```

**Windows:**
Download from [https://ollama.ai/download](https://ollama.ai/download)

### 2. Download a Model

Pull a model to use with Clip Prompt:

```bash
# Recommended for speed and quality
ollama pull llama2:7b

# Alternative models
ollama pull mistral:7b
ollama pull codellama:7b
```

### 3. Start Ollama

```bash
ollama serve
```

## Installation

### Development Setup

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd clip-prompt
   ```

2. **Install dependencies:**
   ```bash
   npm install
   ```

3. **Run in development mode:**
   ```bash
   npm run tauri dev
   ```

### Building for Production

1. **Build the application:**
   ```bash
   npm run tauri build
   ```

2. **Find the installer** in `src-tauri/target/release/bundle/`

## Usage

### Basic Workflow

1. **Start the application** - It will appear in your system tray
2. **Highlight any text** you want to enhance
3. **Press `Ctrl+Shift+E`** - The text will be enhanced automatically
4. **Paste** the enhanced version anywhere

### Manual Enhancement

1. **Open the application window** from the system tray
2. **Paste or type text** in the "Original Text" area
3. **Click "Enhance Prompt"** or press `Ctrl+Enter`
4. **Copy the enhanced text** using the "Copy to Clipboard" button

### Settings

- **Model Selection**: Choose from available Ollama models
- **Ollama URL**: Configure the Ollama server address (default: `http://localhost:11434`)

## Keyboard Shortcuts

- `Ctrl+Shift+E` - Global hotkey to enhance clipboard text
- `Ctrl+Enter` - Enhance text in the application
- `Ctrl+Shift+C` - Copy enhanced text to clipboard
- `Escape` - Clear all text

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                  User Environment                    │
│ ┌───────────────┐      ┌──────────────────────────┐ │
│ │ Any App (Web, │ Ctrl+C │ Highlighted text copied │ │
│ │ Editor, etc.) │──────▶│ to Clipboard            │ │
│ └───────────────┘      └──────────────────────────┘ │
└─────────────────────────────────────────────────────┘
                        │
                        ▼ (Ctrl+Shift+E)
┌─────────────────────────────────────────────────────┐
│                   Clip Prompt                       │
│  (Tauri + Rust)                                     │
│                                                     │
│  - Global Hotkey Listener                           │
│  - Clipboard Handler                                │
│  - REST Client to Ollama                            │
│  - System Tray UI                                   │
└─────────────────────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────┐
│                 Ollama Server                       │
│      (Local AI Model)                               │
│                                                     │
│  - Runs on localhost:11434                          │
│  - Processes text enhancement                       │
│  - Returns improved prompts                         │
└─────────────────────────────────────────────────────┘
```

## Development

### Project Structure

```
clip-prompt/
├── src/                    # Frontend (HTML/CSS/JS)
│   ├── index.html         # Main UI
│   ├── styles.css         # Styling
│   └── main.js           # Frontend logic
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs       # Entry point
│   │   └── lib.rs        # Core functionality
│   ├── Cargo.toml        # Rust dependencies
│   └── tauri.conf.json   # Tauri configuration
└── package.json          # Node.js dependencies
```

### Key Components

- **Global Hotkey Handler**: Listens for `Ctrl+Shift+E` globally
- **Clipboard Manager**: Handles reading/writing clipboard content
- **Ollama Client**: Communicates with local AI model server
- **System Tray**: Provides minimal UI and settings access

### Adding New Features

1. **Backend (Rust)**: Add commands in `src-tauri/src/lib.rs`
2. **Frontend (JavaScript)**: Update UI logic in `src/main.js`
3. **UI (HTML/CSS)**: Modify interface in `src/index.html` and `src/styles.css`

## Troubleshooting

### Common Issues

**"Failed to connect to Ollama"**
- Ensure Ollama is running: `ollama serve`
- Check the URL in settings (default: `http://localhost:11434`)
- Verify the model is downloaded: `ollama list`

**"Global hotkey not working"**
- Check if another application is using the same shortcut
- Restart the application
- On macOS, ensure accessibility permissions are granted

**"Clipboard operations failing"**
- Check system clipboard permissions
- Restart the application
- Try copying/pasting manually first

### Logs

Enable debug logging by setting the environment variable:
```bash
RUST_LOG=debug npm run tauri dev
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Tauri](https://tauri.app/) - Cross-platform desktop framework
- [Ollama](https://ollama.ai/) - Local AI model server
- [Llama2](https://ai.meta.com/llama/) - Open source language model

---

**Made with ❤️ for privacy-conscious AI users**
