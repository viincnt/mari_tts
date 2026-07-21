use crate::tts;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

fn default_lang() -> String {
    "en".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sound {
    pub id: String,
    pub label: String,
    pub text: String,
    #[serde(default = "default_lang")]
    pub lang: String,
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path().app_data_dir().map_err(|e| e.to_string())
}

fn sounds_json_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_data_dir(app)?.join("sounds.json"))
}

fn sounds_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app_data_dir(app)?.join("sounds");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn read_sounds(app: &AppHandle) -> Result<Vec<Sound>, String> {
    let path = sounds_json_path(app)?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

fn write_sounds(app: &AppHandle, sounds: &[Sound]) -> Result<(), String> {
    let path = sounds_json_path(app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(sounds).map_err(|e| e.to_string())?;
    fs::write(&path, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_sounds(app: AppHandle) -> Result<Vec<Sound>, String> {
    read_sounds(&app)
}

#[tauri::command]
pub fn create_sound(
    app: AppHandle,
    label: String,
    text: String,
    lang: String,
) -> Result<Sound, String> {
    let label = label.trim();
    let text = text.trim();
    if label.is_empty() || text.is_empty() {
        return Err("Label and text are required".into());
    }
    if !tts::SUPPORTED_LANGUAGES.contains(&lang.as_str()) {
        return Err(format!("unsupported language: {lang}"));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let wav = tts::synthesize_wav(&lang, text)?;
    let wav_path = sounds_dir(&app)?.join(format!("{id}.wav"));
    fs::write(&wav_path, wav).map_err(|e| e.to_string())?;

    let sound = Sound {
        id,
        label: label.to_string(),
        text: text.to_string(),
        lang,
    };

    let mut sounds = read_sounds(&app)?;
    sounds.push(sound.clone());
    write_sounds(&app, &sounds)?;

    Ok(sound)
}

#[tauri::command]
pub fn delete_sound(app: AppHandle, id: String) -> Result<(), String> {
    let mut sounds = read_sounds(&app)?;
    sounds.retain(|s| s.id != id);
    write_sounds(&app, &sounds)?;

    let wav_path = sounds_dir(&app)?.join(format!("{id}.wav"));
    if wav_path.exists() {
        fs::remove_file(&wav_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn get_sound_audio(app: AppHandle, id: String) -> Result<Vec<u8>, String> {
    let wav_path = sounds_dir(&app)?.join(format!("{id}.wav"));
    fs::read(&wav_path).map_err(|e| e.to_string())
}
