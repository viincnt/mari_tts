mod commands;
mod native_tts;
mod robotize;
mod tts;

use commands::{create_sound, delete_sound, get_sound_audio, list_sounds};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_sounds,
            create_sound,
            delete_sound,
            get_sound_audio
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
