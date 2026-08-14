use std::process::Command;

const ALLOWED_EXE: [&str; 7] = [
    "NioBiologic.exe",
    "CaptainsLog.exe",
    "PixelProcessor.exe",
    "AntiCloud.exe",
    "Nyomcraft.exe",
    "RedOps.exe",
    "Breathe.exe",
];

#[tauri::command]
fn launch_app(exe_name: String) -> Result<(), String> {
    if !ALLOWED_EXE.contains(&exe_name.as_str()) {
        return Err(format!("App desconocida: {exe_name}"));
    }

    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let dir = current_exe
        .parent()
        .ok_or_else(|| "No se pudo resolver la carpeta de instalación".to_string())?;
    let target = dir.join(&exe_name);

    if !target.exists() {
        return Err(format!("No se encontró {exe_name} en {}", dir.display()));
    }

    Command::new(&target)
        .current_dir(dir)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch_app])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
