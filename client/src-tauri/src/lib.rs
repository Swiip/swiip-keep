use todo::Todo;

pub mod todo;

#[tauri::command]
async fn load_todos() -> Result<Vec<Todo>, String> {
    let result = todo::load_todos().await;
    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
async fn save_todos(todos: Vec<Todo>) -> Result<String, String> {
    let result = todo::save_todos(&todos).await;
    match result {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![load_todos, save_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
