use todo::Todo;

pub mod todo;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_todos() -> Result<Vec<Todo>, String> {
    match todo::load_todos() {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn save_todos(todos: Vec<Todo>) -> Result<String, String> {
    match todo::save_todos(&todos) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, load_todos, save_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
