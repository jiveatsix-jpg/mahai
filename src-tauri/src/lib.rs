use std::process::Command;

const ALLOWED_EXE: [&str; 11] = [
    "NioBiologic.exe",
    "CaptainsLog.exe",
    "PixelProcessor.exe",
    "AntiCloud.exe",
    "Nyomcraft.exe",
    "RedOps.exe",
    "Breathe.exe",
    "ColtDrill.exe",
    "MediaOps.exe",
    "AeterCalendar.exe",
    "RetroDarts.exe",
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

// Lanza el desinstalador de Inno Setup (unins000.exe, el mismo binario que usan
// el acceso directo "Desinstalar Ma'hai" del Start Menu y Agregar/quitar programas)
// y cierra el hub enseguida — un exe en ejecución no puede autodestruirse, así
// que Ma'hai tiene que salir para que el desinstalador pueda borrar sus archivos.
// El asistente de Inno ya pide su propia confirmación antes de borrar nada.
#[tauri::command]
fn launch_uninstaller(app: tauri::AppHandle) -> Result<(), String> {
    let current_exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let dir = current_exe
        .parent()
        .ok_or_else(|| "No se pudo resolver la carpeta de instalación".to_string())?;
    let uninstaller = dir.join("unins000.exe");

    if !uninstaller.exists() {
        return Err("No se encontró el desinstalador (unins000.exe)".to_string());
    }

    Command::new(&uninstaller)
        .current_dir(dir)
        .spawn()
        .map_err(|e| e.to_string())?;

    app.exit(0);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch_app, launch_uninstaller])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
