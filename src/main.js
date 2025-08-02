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
    const connectionSuccess = await testOllamaConnection();
    
    // If connection failed, don't try to load models
    if (!connectionSuccess) {
        console.log('❌ Skipping model loading due to connection failure');
        return;
    }
    
    // Load available models and set initial model
    await loadAvailableModels();

    // Load platform-specific autostart instructions
    await loadPlatformInstructions();
    
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
    
    // Settings event listeners
    document.getElementById('modelSelect').addEventListener('change', handleModelChange);
    document.getElementById('toggleInstallInstructions').addEventListener('click', toggleInstallInstructions);
    document.getElementById('resetSystemPrompt').addEventListener('click', handleResetSystemPrompt);
    document.getElementById('systemPrompt').addEventListener('input', handleSystemPromptChange);
    
    // Add click handlers for download links
    document.querySelectorAll('a[href*="ollama.ai"]').forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            // Use Tauri's shell plugin to open the URL
            window.__TAURI__.shell.open(link.href);
        });
    });
    
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
 * Handle model change
 */
async function handleModelChange() {
    const modelSelect = document.getElementById('modelSelect');
    const selectedModel = modelSelect.value;
    
    try {
        console.log('🔄 Changing model to:', selectedModel);
        updateModelStatus('connecting', `Switching to ${selectedModel}...`);
        
        // Save the setting
        localStorage.setItem('selectedModel', selectedModel);
        
        // Update the model in the backend
        await invoke('update_model', { model: selectedModel });
        
        // Test the new model with a simple request
        await invoke('test_ollama_connection');
        
        updateModelStatus('connected', selectedModel);
        console.log('✅ Model changed successfully');
    } catch (error) {
        console.error('❌ Model change failed:', error);
        updateModelStatus('error', `Failed to switch to ${selectedModel}`);
    }
}

/**
 * Handle text enhancement
 */
async function handleEnhance() {
    if (isEnhancing) return;
    
    const inputText = document.getElementById('inputText').value.trim();
    const outputTextarea = document.getElementById('outputText');
    const enhanceBtn = document.getElementById('enhanceBtn');
    const modelSelect = document.getElementById('modelSelect');
    
    if (!inputText) {
        updateOllamaStatus('error', 'Please enter some text to enhance');
        return;
    }
    
    // Check if a valid model is selected
    if (!modelSelect.value || modelSelect.value === 'No models available' || modelSelect.value === 'Failed to load models') {
        updateOllamaStatus('error', 'No AI model available. Please check your Ollama installation.');
        return;
    }
    
    isEnhancing = true;
    enhanceBtn.textContent = 'Enhancing...';
    enhanceBtn.disabled = true;
    outputTextarea.value = 'Processing...';
    
    updateOllamaStatus('connecting', 'Enhancing text...');
    
    try {
        console.log('🤖 Enhancing text:', inputText);
        
        // Get the currently selected model
        const selectedModel = modelSelect.value;
        
        const enhanced = await invoke('enhance_prompt', { 
            prompt: inputText,
            model: selectedModel
        });
        
        outputTextarea.value = enhanced;
        updateOllamaStatus('connected', 'Text enhanced successfully');
        
        console.log('✅ Enhancement complete');
    } catch (error) {
        console.error('❌ Enhancement failed:', error);
        outputTextarea.value = '';
        updateOllamaStatus('error', `Enhancement failed: ${error}`);
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
    updateOllamaStatus('connected', 'Connected to Ollama');
    console.log('🧹 Text fields cleared');
}

/**
 * Handle copy to clipboard
 */
async function handleCopy() {
    const outputText = document.getElementById('outputText').value;
    
    if (!outputText || outputText === 'Processing...') {
        updateOllamaStatus('error', 'No enhanced text to copy');
        return;
    }
    
    try {
        await navigator.clipboard.writeText(outputText);
        updateOllamaStatus('connected', 'Text copied to clipboard');
        console.log('📋 Text copied to clipboard');
    } catch (error) {
        console.error('❌ Copy failed:', error);
        updateOllamaStatus('error', 'Failed to copy text');
    }
}

/**
 * Load platform-specific autostart instructions
 */
async function loadPlatformInstructions() {
    try {
        console.log('🔄 Loading platform instructions...');
        const platform = await invoke('get_platform');
        
        // Hide all instruction divs first
        document.getElementById('macosInstructions').classList.add('hidden');
        document.getElementById('windowsInstructions').classList.add('hidden');
        document.getElementById('linuxInstructions').classList.add('hidden');
        
        // Show the appropriate instructions based on platform
        switch (platform) {
            case 'macos':
                document.getElementById('macosInstructions').classList.remove('hidden');
                updateAutostartStatus('connected', 'macOS detected');
                break;
            case 'windows':
                document.getElementById('windowsInstructions').classList.remove('hidden');
                updateAutostartStatus('connected', 'Windows detected');
                break;
            case 'linux':
                document.getElementById('linuxInstructions').classList.remove('hidden');
                updateAutostartStatus('connected', 'Linux detected');
                break;
            default:
                updateAutostartStatus('error', 'Unknown platform');
        }
        
        console.log('✅ Platform instructions loaded for:', platform);
    } catch (error) {
        console.error('❌ Failed to load platform instructions:', error);
        updateAutostartStatus('error', 'Failed to detect platform');
    }
}

/**
 * Test Ollama connection
 */
async function testOllamaConnection() {
    console.log('🔍 Testing Ollama connection...');
    updateOllamaStatus('connecting', 'Testing connection...');
    
    try {
        await invoke('test_ollama_connection');
        updateOllamaStatus('connected', 'Connected to Ollama');
        
        // Hide Ollama not installed warning
        document.getElementById('ollamaNotInstalled').classList.add('hidden');
        
        console.log('✅ Ollama connection successful');
        return true;
    } catch (error) {
        console.error('❌ Ollama connection failed:', error);
        updateOllamaStatus('error', `Connection failed: ${error}`);
        
        // Show Ollama not installed warning
        document.getElementById('ollamaNotInstalled').classList.remove('hidden');
        
        return false;
    }
}

/**
 * Toggle install instructions visibility
 */
function toggleInstallInstructions() {
    const instructions = document.getElementById('installInstructions');
    const button = document.getElementById('toggleInstallInstructions');
    
    if (instructions.classList.contains('hidden')) {
        instructions.classList.remove('hidden');
        button.textContent = 'Hide Instructions ▲';
    } else {
        instructions.classList.add('hidden');
        button.textContent = 'Show Instructions ▼';
    }
}

/**
 * Handle system prompt change
 */
async function handleSystemPromptChange() {
    const systemPrompt = document.getElementById('systemPrompt').value;
    
    try {
        await invoke('update_system_prompt', { prompt: systemPrompt });
        console.log('✅ System prompt updated');
        
        // Save to localStorage
        localStorage.setItem('systemPrompt', systemPrompt);
    } catch (error) {
        console.error('❌ Failed to update system prompt:', error);
    }
}

/**
 * Handle reset system prompt to default
 */
async function handleResetSystemPrompt() {
    try {
        await invoke('reset_system_prompt');
        console.log('✅ System prompt reset to default');
        
        // Remove from localStorage
        localStorage.removeItem('systemPrompt');
        
        // Show default prompt in textarea
        try {
            const defaultPrompt = await invoke('get_system_prompt');
            document.getElementById('systemPrompt').value = defaultPrompt;
            console.log('📝 Reset to default system prompt');
        } catch (error) {
            console.error('❌ Failed to load default prompt after reset:', error);
        }
        
        // Show success message
        console.log('✅ System prompt reset to default');
    } catch (error) {
        console.error('❌ Failed to reset system prompt:', error);
    }
}

/**
 * Load available models from Ollama
 */
async function loadAvailableModels() {
    try {
        console.log('📋 Loading available models...');
        updateModelStatus('connecting', 'Loading models...');
        
        const models = await invoke('get_available_models');
        
        const modelSelect = document.getElementById('modelSelect');
        
        // Clear existing options
        modelSelect.innerHTML = '';
        
        if (models.length === 0) {
            // No models available
            updateModelStatus('error', 'No models available');
            console.log('❌ No models available');
            
            // Show warning message
            document.getElementById('modelWarning').classList.remove('hidden');
            
            // Add a placeholder option
            const option = document.createElement('option');
            option.value = '';
            option.textContent = 'No models available';
            option.disabled = true;
            modelSelect.appendChild(option);
            
            return;
        }
        
        // Hide warning messages if models are available
        document.getElementById('modelWarning').classList.add('hidden');
        document.getElementById('ollamaNotInstalled').classList.add('hidden');
        
        // Add models to dropdown (models is an array of strings)
        models.forEach(modelName => {
            const option = document.createElement('option');
            option.value = modelName;
            option.textContent = modelName;
            modelSelect.appendChild(option);
        });
        
        // Restore saved selection or use first available model
        const savedModel = localStorage.getItem('selectedModel');
        let selectedModel;
        
        if (savedModel && models.includes(savedModel)) {
            // Use saved model if it's still available
            selectedModel = savedModel;
        } else {
            // Use first available model
            selectedModel = models[0];
            localStorage.setItem('selectedModel', selectedModel);
        }
        
        modelSelect.value = selectedModel;
        updateModelStatus('connected', selectedModel);
        
        // Set the initial model in the backend
        await invoke('set_initial_model');
        
        console.log(`✅ Loaded ${models.length} models, using: ${selectedModel}`);
    } catch (error) {
        console.error('❌ Failed to load models:', error);
        updateModelStatus('error', 'Failed to load models');
        
        // Add error placeholder
        const modelSelect = document.getElementById('modelSelect');
        modelSelect.innerHTML = '';
        const option = document.createElement('option');
        option.value = '';
        option.textContent = 'Failed to load models';
        option.disabled = true;
        modelSelect.appendChild(option);
        
        // Show warning message
        document.getElementById('modelWarning').classList.remove('hidden');
        
        // Show Ollama not installed warning if it's a connection error
        if (error.includes('Connection failed') || error.includes('Failed to send request')) {
            document.getElementById('ollamaNotInstalled').classList.remove('hidden');
        }
    }
}

/**
 * Update Ollama connection status
 */
function updateOllamaStatus(status, message) {
    const statusDot = document.getElementById('ollamaStatusDot');
    const statusText = document.getElementById('ollamaStatusText');
    
    // Remove existing status classes
    statusDot.className = 'status-dot';
    
    // Add new status class and update text
    switch (status) {
        case 'connected':
            statusDot.classList.add('status-connected');
            statusText.textContent = message || 'Connected';
            statusText.className = 'text-sm text-green-400';
            break;
        case 'connecting':
            statusDot.classList.add('status-connecting');
            statusText.textContent = message || 'Connecting...';
            statusText.className = 'text-sm text-yellow-400';
            break;
        case 'error':
            statusDot.classList.add('status-disconnected');
            statusText.textContent = message || 'Disconnected';
            statusText.className = 'text-sm text-red-400';
            break;
        default:
            statusDot.classList.add('status-connecting');
            statusText.textContent = message || 'Unknown status';
            statusText.className = 'text-sm text-gray-400';
    }
}

/**
 * Update autostart status
 */
function updateAutostartStatus(status, message) {
    const statusDot = document.getElementById('autostartStatusDot');
    const statusText = document.getElementById('autostartStatusText');
    
    // Remove existing status classes
    statusDot.className = 'status-dot';
    
    // Add new status class and update text
    switch (status) {
        case 'connected':
            statusDot.classList.add('status-connected');
            statusText.textContent = message || 'Enabled';
            statusText.className = 'text-sm text-green-400';
            break;
        case 'connecting':
            statusDot.classList.add('status-connecting');
            statusText.textContent = message || 'Configuring...';
            statusText.className = 'text-sm text-yellow-400';
            break;
        case 'error':
            statusDot.classList.add('status-disconnected');
            statusText.textContent = message || 'Error';
            statusText.className = 'text-sm text-red-400';
            break;
        default:
            statusDot.classList.add('status-neutral');
            statusText.textContent = message || 'Disabled';
            statusText.className = 'text-sm text-[var(--text-secondary)]';
    }
}

/**
 * Update model status
 */
function updateModelStatus(status, message) {
    const statusDot = document.getElementById('modelStatusDot');
    const statusText = document.getElementById('modelStatusText');
    
    // Remove existing status classes
    statusDot.className = 'status-dot';
    
    // Add new status class and update text
    switch (status) {
        case 'connected':
            statusDot.classList.add('status-connected');
            statusText.textContent = message || 'Model loaded';
            statusText.className = 'text-sm text-green-400';
            break;
        case 'connecting':
            statusDot.classList.add('status-connecting');
            statusText.textContent = message || 'Loading...';
            statusText.className = 'text-sm text-yellow-400';
            break;
        case 'error':
            statusDot.classList.add('status-disconnected');
            statusText.textContent = message || 'Error';
            statusText.className = 'text-sm text-red-400';
            break;
        default:
            statusDot.classList.add('status-neutral');
            statusText.textContent = message || 'No model';
            statusText.className = 'text-sm text-[var(--text-secondary)]';
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
            
            // Update the backend model to match the saved setting
            await invoke('update_model', { model: savedModel });
        }
        
        // Load system prompt
        const savedSystemPrompt = localStorage.getItem('systemPrompt');
        const systemPromptTextarea = document.getElementById('systemPrompt');
        
        if (savedSystemPrompt) {
            // Use saved custom prompt
            systemPromptTextarea.value = savedSystemPrompt;
            
            // Update the backend system prompt to match the saved setting
            await invoke('update_system_prompt', { prompt: savedSystemPrompt });
        } else {
            // Show default prompt in textarea (but don't save it as custom)
            try {
                const defaultPrompt = await invoke('get_system_prompt');
                systemPromptTextarea.value = defaultPrompt;
                console.log('📝 Loaded default system prompt into textarea');
            } catch (error) {
                console.error('❌ Failed to load default system prompt:', error);
            }
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
        
        console.log('✅ Settings saved');
        updateOllamaStatus('connected', 'Settings saved');
    } catch (error) {
        console.error('❌ Failed to save settings:', error);
        updateOllamaStatus('error', 'Failed to save settings');
    }
}

// Initialize when DOM is loaded
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}
