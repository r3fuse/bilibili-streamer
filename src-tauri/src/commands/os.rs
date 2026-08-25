use serde_json::Value;

#[tauri::command]
pub async fn get_current_os() ->Value {
   let os = std::env::consts::OS;
    return serde_json::json!(
        os
    );
}