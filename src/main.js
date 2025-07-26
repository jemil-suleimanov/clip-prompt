// Tauri 2 global API
const { invoke } = window.__TAURI__.core;

// Global variables
let isEnhancing = false;

/**
 * Initialize the application
 */
async function init() {
    console.log('🚀 Initializing Clip Prompt...');
    
    // Set up event listeners
    setupEventListeners();
    
    // Load saved settings
    await loadSettings();
    
    // Test Ollama connection
    await testOllamaConnection();
    
    // Load available models
    await loadAvailableModels();

    // Check autostart status on startup
    await checkAutostartStatus();
    
    console.log('✅ Clip Prompt initialized successfully');
}

/**
 * Set up all event listeners
 */
function setupEventListeners() {
    // Button event listeners
    document.getElementById('enhanceBtn').addEventListener('click', handleEnhance);
    document.getElementById('clearBtn').addEventListener('click', handleClear);
    document.getElementById('copyBtn').addEventListener('click', handleCopy);
    document.getElementById('quitBtn').addEventListener('click', () => {
        // Hide window instead of quitting (let system tray handle it)
        window.close();
    });
    
    // Settings event listeners
    document.getElementById('modelSelect').addEventListener('change', saveSettings);
    document.getElementById('autostartToggle').addEventListener('change', handleAutostartToggle);
    
    // Keyboard shortcuts
    document.addEventListener('keydown', (e) => {
        if (e.ctrlKey && e.key === 'Enter') {
            e.preventDefault();
            handleEnhance();
        } else if (e.ctrlKey && e.shiftKey && e.key === 'C') {
            e.preventDefault();
            handleCopy();
        } else if (e.key === 'Escape') {
            e.preventDefault();
            handleClear();
        }
    });
    
    console.log('✅ Event listeners set up');
}

/**
 * Handle text enhancement
 */
async function handleEnhance() {
    if (isEnhancing) return;
    
    const inputText = document.getElementById('inputText').value.trim();
    const outputTextarea = document.getElementById('outputText');
    const enhanceBtn = document.getElementById('enhanceBtn');
    
    if (!inputText) {
        updateStatus('error', 'Please enter some text to enhance');
        return;
    }
    
    isEnhancing = true;
    enhanceBtn.textContent = 'Enhancing...';
    enhanceBtn.disabled = true;
    outputTextarea.value = 'Processing...';
    
    updateStatus('connecting', 'Enhancing text...');
    
    try {
        console.log('🤖 Enhancing text:', inputText);
        const enhanced = await invoke('enhance_prompt', { prompt: inputText });
        
        outputTextarea.value = enhanced;
        updateStatus('connected', 'Text enhanced successfully');
        
        console.log('✅ Enhancement complete');
    } catch (error) {
        console.error('❌ Enhancement failed:', error);
        outputTextarea.value = '';
        updateStatus('error', `Enhancement failed: ${error}`);
    } finally {
        isEnhancing = false;
        enhanceBtn.textContent = 'Enhance';
        enhanceBtn.disabled = false;
    }
}

/**
 * Handle clear action
 */
function handleClear() {
    document.getElementById('inputText').value = '';
    document.getElementById('outputText').value = '';
    updateStatus('connected', 'Ready');
    console.log('🧹 Text fields cleared');
}

/**
 * Handle copy to clipboard
 */
async function handleCopy() {
    const outputText = document.getElementById('outputText').value;
    
    if (!outputText || outputText === 'Processing...') {
        updateStatus('error', 'No enhanced text to copy');
        return;
    }
    
    try {
        await navigator.clipboard.writeText(outputText);
        updateStatus('connected', 'Text copied to clipboard');
        console.log('📋 Text copied to clipboard');
    } catch (error) {
        console.error('❌ Copy failed:', error);
        updateStatus('error', 'Failed to copy text');
    }
}

/**
 * Handle autostart toggle
 */
async function handleAutostartToggle() {
    const toggle = document.getElementById('autostartToggle');
    const isEnabled = toggle.checked;
    
    try {
        console.log('🔄 Setting autostart:', isEnabled ? 'enabled' : 'disabled');
        
        if (isEnabled) {
            await invoke('enable_autostart');
            updateStatus('connected', 'Autostart enabled');
        } else {
            await invoke('disable_autostart');
            updateStatus('connected', 'Autostart disabled');
        }
        
        // Save setting
        localStorage.setItem('autostart', isEnabled.toString());
        
        console.log('✅ Autostart', isEnabled ? 'enabled' : 'disabled');
    } catch (error) {
        console.error('❌ Autostart toggle failed:', error);
        toggle.checked = !isEnabled; // Revert on error
        updateStatus('error', `Failed to ${isEnabled ? 'enable' : 'disable'} autostart: ${error}`);
    }
}

/**
 * Check autostart status on startup
 */
async function checkAutostartStatus() {
    try {
        const isEnabled = await invoke('is_autostart_enabled');
        const toggle = document.getElementById('autostartToggle');
        toggle.checked = isEnabled;
        
        // Update localStorage to match actual system state
        localStorage.setItem('autostart', isEnabled.toString());
        
        console.log('📋 Autostart status:', isEnabled ? 'enabled' : 'disabled');
    } catch (error) {
        console.error('❌ Failed to check autostart status:', error);
    }
}

/**
 * Test Ollama connection
 */
async function testOllamaConnection() {
    console.log('🔍 Testing Ollama connection...');
    updateStatus('connecting', 'Testing connection...');
    
    try {
        await invoke('test_ollama_connection');
        updateStatus('connected', 'Connected to Ollama');
        console.log('✅ Ollama connection successful');
        return true;
    } catch (error) {
        console.error('❌ Ollama connection failed:', error);
        updateStatus('error', `Connection failed: ${error}`);
        return false;
    }
}

/**
 * Load available models from Ollama
 */
async function loadAvailableModels() {
    try {
        console.log('📋 Loading available models...');
        const models = await invoke('get_available_models');
        
        const modelSelect = document.getElementById('modelSelect');
        
        // Clear existing options
        modelSelect.innerHTML = '';
        
        // Add models to dropdown (models is an array of strings)
        models.forEach(modelName => {
            const option = document.createElement('option');
            option.value = modelName;
            option.textContent = modelName;
            modelSelect.appendChild(option);
        });
        
        // Restore saved selection
        const savedModel = localStorage.getItem('selectedModel');
        if (savedModel) {
            modelSelect.value = savedModel;
        }
        
        console.log(`✅ Loaded ${models.length} models`);
    } catch (error) {
        console.error('❌ Failed to load models:', error);
        // Keep default options if loading fails
    }
}

/**
 * Update status indicator
 */
function updateStatus(status, message) {
    const statusDot = document.getElementById('statusDot');
    const statusText = document.getElementById('statusText');
    
    // Remove existing status classes
    statusDot.className = 'status-dot';
    
    // Add new status class and update text
    switch (status) {
        case 'connected':
            statusDot.classList.add('status-connected');
            statusText.textContent = message || 'Connected';
            statusText.className = 'text-green-400';
            break;
        case 'connecting':
            statusDot.classList.add('status-connecting');
            statusText.textContent = message || 'Connecting...';
            statusText.className = 'text-yellow-400';
            break;
        case 'error':
            statusDot.classList.add('status-disconnected');
            statusText.textContent = message || 'Disconnected';
            statusText.className = 'text-red-400';
            break;
        default:
            statusDot.classList.add('status-connecting');
            statusText.textContent = message || 'Unknown status';
            statusText.className = 'text-gray-400';
    }
}

/**
 * Load settings from localStorage
 */
async function loadSettings() {
    console.log('📱 Loading settings...');
    
    try {
        // Load model selection
        const savedModel = localStorage.getItem('selectedModel');
        if (savedModel) {
            const modelSelect = document.getElementById('modelSelect');
            modelSelect.value = savedModel;
        }
        
        // Note: Autostart status is checked from system on startup
        // via checkAutostartStatus() function
        
        console.log('✅ Settings loaded');
    } catch (error) {
        console.error('❌ Failed to load settings:', error);
    }
}

/**
 * Save settings to localStorage
 */
function saveSettings() {
    console.log('💾 Saving settings...');
    
    try {
        // Save model selection
        const modelSelect = document.getElementById('modelSelect');
        localStorage.setItem('selectedModel', modelSelect.value);
        
        // Save autostart setting
        const autostartToggle = document.getElementById('autostartToggle');
        localStorage.setItem('autostart', autostartToggle.checked.toString());
        
        console.log('✅ Settings saved');
        updateStatus('connected', 'Settings saved');
    } catch (error) {
        console.error('❌ Failed to save settings:', error);
        updateStatus('error', 'Failed to save settings');
    }
}

// Initialize when DOM is loaded
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
