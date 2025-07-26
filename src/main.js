// Import Tauri APIs - Tauri 2 syntax
const { invoke } = window.__TAURI__.core;

// DOM elements
const originalTextArea = document.getElementById('original-text');
const enhancedTextArea = document.getElementById('enhanced-text');
const enhanceBtn = document.getElementById('enhance-btn');
const clearBtn = document.getElementById('clear-btn');
const copyBtn = document.getElementById('copy-btn');
const statusIndicator = document.getElementById('status-indicator');
const statusText = document.querySelector('.status-text');
const statusDot = document.querySelector('.status-dot');
const modelSelect = document.getElementById('model-select');
const ollamaUrlInput = document.getElementById('ollama-url');

// State
let isProcessing = false;

// Initialize the application
async function init() {
    try {
        console.log('Clip Prompt initializing...');
        updateStatus('ready', 'Ready');
        
        // Load settings from localStorage
        loadSettings();
        
        // Set up event listeners
        setupEventListeners();
        
        // Test Ollama connection
        await testOllamaConnection();
        
    } catch (error) {
        console.error('Failed to initialize:', error);
        updateStatus('error', 'Initialization failed');
    }
}

// Set up event listeners
function setupEventListeners() {
    enhanceBtn.addEventListener('click', handleEnhance);
    clearBtn.addEventListener('click', handleClear);
    copyBtn.addEventListener('click', handleCopy);
    
    // Auto-save settings
    modelSelect.addEventListener('change', saveSettings);
    ollamaUrlInput.addEventListener('change', saveSettings);
    
    // Handle paste events
    originalTextArea.addEventListener('paste', handlePaste);
    
    // Handle keyboard shortcuts
    document.addEventListener('keydown', handleKeyboardShortcuts);
}

// Test Ollama connection
async function testOllamaConnection() {
    try {
        updateStatus('processing', 'Testing Ollama connection...');
        const isConnected = await invoke('test_ollama_connection');
        
        if (isConnected) {
            updateStatus('ready', 'Ollama connected');
            await loadAvailableModels();
        } else {
            updateStatus('error', 'Ollama not connected');
        }
    } catch (error) {
        console.error('Ollama connection test failed:', error);
        updateStatus('error', 'Ollama connection failed');
    }
}

// Load available models from Ollama
async function loadAvailableModels() {
    try {
        const models = await invoke('get_available_models');
        
        // Clear existing options
        modelSelect.innerHTML = '';
        
        // Add available models
        models.forEach(model => {
            const option = document.createElement('option');
            option.value = model;
            option.textContent = model;
            modelSelect.appendChild(option);
        });
        
        // Select the first model if none is selected
        if (modelSelect.value === '') {
            modelSelect.value = models[0] || 'llama2:7b';
        }
        
    } catch (error) {
        console.error('Failed to load models:', error);
    }
}

// Handle enhance button click
async function handleEnhance() {
    if (isProcessing) return;
    
    const originalText = originalTextArea.value.trim();
    if (!originalText) {
        updateStatus('error', 'No text to enhance');
        setTimeout(() => updateStatus('ready', 'Ready'), 2000);
        return;
    }
    
    try {
        isProcessing = true;
        updateStatus('processing', 'Enhancing prompt...');
        enhanceBtn.disabled = true;
        
        // Call the Rust backend to enhance the prompt
        const enhancedText = await invoke('enhance_prompt', { prompt: originalText });
        
        // Display the enhanced text
        enhancedTextArea.value = enhancedText;
        copyBtn.disabled = false;
        
        updateStatus('ready', 'Enhanced successfully');
        
    } catch (error) {
        console.error('Error enhancing prompt:', error);
        updateStatus('error', 'Enhancement failed');
        setTimeout(() => updateStatus('ready', 'Ready'), 3000);
    } finally {
        isProcessing = false;
        enhanceBtn.disabled = false;
    }
}

// Handle clear button click
function handleClear() {
    originalTextArea.value = '';
    enhancedTextArea.value = '';
    copyBtn.disabled = true;
    updateStatus('ready', 'Ready');
}

// Handle copy button click
async function handleCopy() {
    try {
        const enhancedText = enhancedTextArea.value;
        if (!enhancedText) return;
        
        // Use the browser's clipboard API
        await navigator.clipboard.writeText(enhancedText);
        updateStatus('ready', 'Copied to clipboard');
        setTimeout(() => updateStatus('ready', 'Ready'), 2000);
        
    } catch (error) {
        console.error('Error copying to clipboard:', error);
        updateStatus('error', 'Failed to copy');
        setTimeout(() => updateStatus('ready', 'Ready'), 2000);
    }
}

// Handle paste events
function handlePaste(event) {
    // Auto-enhance after a short delay if text is pasted
    setTimeout(() => {
        if (originalTextArea.value.trim()) {
            // Don't auto-enhance, let user decide
        }
    }, 100);
}

// Handle keyboard shortcuts
function handleKeyboardShortcuts(event) {
    // Ctrl+Enter to enhance
    if (event.ctrlKey && event.key === 'Enter') {
        event.preventDefault();
        handleEnhance();
    }
    
    // Ctrl+Shift+C to copy enhanced text
    if (event.ctrlKey && event.shiftKey && event.key === 'C') {
        event.preventDefault();
        handleCopy();
    }
    
    // Escape to clear
    if (event.key === 'Escape') {
        event.preventDefault();
        handleClear();
    }
}

// Update status indicator
function updateStatus(type, message) {
    statusText.textContent = message;
    
    // Remove all status classes
    statusDot.classList.remove('ready', 'processing', 'error');
    
    // Add the appropriate class
    statusDot.classList.add(type);
}

// Load settings from localStorage
function loadSettings() {
    try {
        const settings = JSON.parse(localStorage.getItem('clipPromptSettings') || '{}');
        
        if (settings.model) {
            modelSelect.value = settings.model;
        }
        
        if (settings.ollamaUrl) {
            ollamaUrlInput.value = settings.ollamaUrl;
        }
        
    } catch (error) {
        console.error('Error loading settings:', error);
    }
}

// Save settings to localStorage
function saveSettings() {
    try {
        const settings = {
            model: modelSelect.value,
            ollamaUrl: ollamaUrlInput.value
        };
        
        localStorage.setItem('clipPromptSettings', JSON.stringify(settings));
        
    } catch (error) {
        console.error('Error saving settings:', error);
    }
}

// Initialize the application when the page loads
document.addEventListener('DOMContentLoaded', init);
