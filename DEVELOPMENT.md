# Development Guide - Clip Prompt

This guide will help you set up and develop the Clip Prompt application.

## Prerequisites

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install Node.js (v18 or later)
Download from [https://nodejs.org](https://nodejs.org)

### 3. Install Ollama
```bash
curl -fsSL https://ollama.ai/install.sh | sh
```

### 4. Install Tauri CLI
```bash
npm install -g @tauri-apps/cli
```

## Setup

1. **Clone and install dependencies:**
   ```bash
   git clone <repository-url>
   cd clip-prompt
   npm install
   ```

2. **Start Ollama:**
   ```bash
   ollama serve
   ```

3. **Pull a model:**
   ```bash
   ollama pull llama2:7b
   ```

## Development

### Running in Development Mode
```bash
npm run dev
```

This will:
- Start the Tauri development server
- Open the application window
- Enable hot reloading for both frontend and backend

### Building for Production
```bash
npm run build
```

This creates platform-specific installers in `src-tauri/target/release/bundle/`

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

## Key Features

### Current Implementation
- ✅ Basic Tauri application structure
- ✅ Ollama API integration
- ✅ Prompt enhancement functionality
- ✅ Modern UI with status indicators
- ✅ Settings persistence (localStorage)
- ✅ Model selection from available Ollama models

### Planned Features
- 🔄 System tray integration
- 🔄 Global hotkey support
- 🔄 Clipboard integration
- 🔄 Autostart functionality
- 🔄 Cross-platform packaging

## API Reference

### Backend Commands (Rust)

#### `enhance_prompt(prompt: String) -> Result<String, String>`
Enhances a given prompt using the selected Ollama model.

#### `test_ollama_connection() -> Result<bool, String>`
Tests the connection to the Ollama server.

#### `get_available_models() -> Result<Vec<String>, String>`
Retrieves the list of available models from Ollama.

### Frontend Functions (JavaScript)

#### `handleEnhance()`
Enhances the text in the original textarea.

#### `handleCopy()`
Copies the enhanced text to the clipboard.

#### `testOllamaConnection()`
Tests the connection to Ollama and updates the status.

## Troubleshooting

### Common Issues

**"Failed to connect to Ollama"**
- Ensure Ollama is running: `ollama serve`
- Check if the model is downloaded: `ollama list`
- Verify the URL in settings (default: `http://localhost:11434`)

**"Build fails"**
- Ensure Rust is installed and up to date: `rustup update`
- Check Tauri CLI version: `tauri --version`
- Clear build cache: `cargo clean`

**"Frontend not loading"**
- Check browser console for errors
- Ensure all dependencies are installed: `npm install`
- Try clearing browser cache

### Debug Mode

Enable debug logging:
```bash
RUST_LOG=debug npm run dev
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Test thoroughly
5. Commit your changes: `git commit -m 'Add feature'`
6. Push to the branch: `git push origin feature-name`
7. Submit a pull request

## Next Steps

### Phase 1: Core Functionality ✅
- [x] Basic Tauri setup
- [x] Ollama integration
- [x] UI implementation
- [x] Settings management

### Phase 2: System Integration 🔄
- [ ] System tray implementation
- [ ] Global hotkey registration
- [ ] Clipboard operations
- [ ] Window management

### Phase 3: Advanced Features 📋
- [ ] Autostart functionality
- [ ] Cross-platform packaging
- [ ] Performance optimizations
- [ ] Error handling improvements

### Phase 4: Polish & Distribution 🚀
- [ ] Icon and branding
- [ ] Installer creation
- [ ] Documentation
- [ ] Release management 