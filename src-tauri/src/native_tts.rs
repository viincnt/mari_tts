use std::path::Path;
use std::process::Command;

/// Synthesizes `text` (in `lang`) to a 16-bit PCM WAV file at `out_path`
/// using the operating system's own speech engine. These are mature,
/// guaranteed-intelligible voices — the robotic "dictaphone" character is
/// applied afterward as a DSP effect (see `robotize`), rather than relying
/// on a from-scratch synthesizer to sound both correct and robotic.
#[cfg(target_os = "macos")]
pub fn synthesize_to_wav(lang: &str, text: &str, out_path: &Path) -> Result<(), String> {
    let voice = match lang {
        "pt" => "Luciana",
        _ => "Daniel",
    };
    let status = Command::new("say")
        .args([
            "-v",
            voice,
            "-o",
            out_path.to_str().ok_or("invalid output path")?,
            "--data-format=LEI16@22050",
            text,
        ])
        .status()
        .map_err(|e| format!("failed to run `say`: {e}"))?;
    if !status.success() {
        return Err("`say` exited with an error".into());
    }
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn synthesize_to_wav(lang: &str, text: &str, out_path: &Path) -> Result<(), String> {
    let culture = match lang {
        "pt" => "pt-BR",
        _ => "en-US",
    };
    const SCRIPT: &str = r#"
Add-Type -AssemblyName System.Speech
$s = New-Object System.Speech.Synthesis.SpeechSynthesizer
try {
    $culture = New-Object System.Globalization.CultureInfo($env:MARI_TTS_CULTURE)
    $s.SelectVoiceByHints('NotSet', 'NotSet', 0, $culture)
} catch {}
$s.SetOutputToWaveFile($env:MARI_TTS_OUT)
$s.Speak($env:MARI_TTS_TEXT)
$s.Dispose()
"#;
    let status = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", SCRIPT])
        .env("MARI_TTS_CULTURE", culture)
        .env("MARI_TTS_OUT", out_path)
        .env("MARI_TTS_TEXT", text)
        .status()
        .map_err(|e| format!("failed to run PowerShell TTS: {e}"))?;
    if !status.success() {
        return Err("Windows speech synthesis exited with an error".into());
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn synthesize_to_wav(lang: &str, text: &str, out_path: &Path) -> Result<(), String> {
    let voice = match lang {
        "pt" => "pt",
        _ => "en",
    };
    let status = Command::new("espeak-ng")
        .args([
            "-v",
            voice,
            "-w",
            out_path.to_str().ok_or("invalid output path")?,
            text,
        ])
        .status()
        .map_err(|e| {
            format!(
                "failed to run `espeak-ng` (install it, e.g. `sudo apt install espeak-ng`): {e}"
            )
        })?;
    if !status.success() {
        return Err("`espeak-ng` exited with an error".into());
    }
    Ok(())
}
