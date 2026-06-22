mod model;
mod db_model;
mod db;
mod import;
mod export;

#[tauri::command]
async fn init_database() -> Result<(), String> {
    db::init_database()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_packages() -> Result<Vec<db_model::DbPackage>, String> {
    db::list_packages()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_entries() -> Result<Vec<db_model::DbEntry>, String> {
    db::list_entries()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_entries(query: String) -> Result<Vec<db_model::DbEntry>, String> {
    db::search_entries(&query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_relations() -> Result<Vec<db_model::DbRelation>, String> {
    db::list_relations()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn import_package(path: String) -> Result<(), String> {
    import::import_package(&path)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn export_package(package_id: String, path: String) -> Result<(), String> {
    export::export_package(&package_id, &path)
        .await
        .map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            init_database,
            list_packages,
            list_entries,
            search_entries,
            list_relations,
            import_package,
            export_package
        ])
        .run(tauri::generate_context!())
        .expect("error while running ORKB Editor");
}
