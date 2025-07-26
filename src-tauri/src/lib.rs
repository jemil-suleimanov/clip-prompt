use std::sync::Arc;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use log::{info, error, debug};

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
async fn enhance_prompt(prompt: String, state: tauri::State<'_, Arc<AppState>>) -> Result<String, String> {
    let client = reqwest::Client::new();
    
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

    debug!("Sending request to Ollama: {:?}", request);
    debug!("Request URL: {}/api/generate", state.ollama_url);

    let response = client
        .post(&format!("{}/api/generate", state.ollama_url))
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    debug!("Response status: {}", response.status());
    
    // Get the raw response text first
    let response_text = response.text().await
        .map_err(|e| format!("Failed to get response text: {}", e))?;
    
    debug!("Raw response: {}", response_text);

    // Try to parse as JSON
    let ollama_response: OllamaResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse response: {} (response: {})", e, response_text))?;

    debug!("Parsed response: {:?}", ollama_response);

    Ok(ollama_response.response)
}

#[tauri::command]
async fn test_ollama_connection(state: tauri::State<'_, Arc<AppState>>) -> Result<bool, String> {
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
async fn get_available_models(state: tauri::State<'_, Arc<AppState>>) -> Result<Vec<String>, String> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    
    let app_state = Arc::new(AppState::default());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .setup(|_app| {
            info!("Clip Prompt started successfully");
            info!("Ready to enhance prompts with Ollama");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            enhance_prompt,
            test_ollama_connection,
            get_available_models
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
