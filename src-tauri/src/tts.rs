use crate::{native_tts, robotize};

pub const SUPPORTED_LANGUAGES: &[&str] = &["en", "pt"];

/// Synthesizes `text` (in `lang`) to WAV bytes: the OS's native speech
/// engine renders clean, intelligible speech, then `robotize` reprocesses
/// it into the flat, crunchy "80s dictaphone" character.
pub fn synthesize_wav(lang: &str, text: &str) -> Result<Vec<u8>, String> {
    if !SUPPORTED_LANGUAGES.contains(&lang) {
        return Err(format!("unsupported language: {lang}"));
    }

    let raw_path = std::env::temp_dir().join(format!("mari_tts_raw_{}.wav", uuid::Uuid::new_v4()));
    native_tts::synthesize_to_wav(lang, text, &raw_path)?;

    let mut reader = hound::WavReader::open(&raw_path).map_err(|e| e.to_string())?;
    let spec = reader.spec();
    let mut samples: Vec<i16> = reader
        .samples::<i16>()
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(&raw_path);

    robotize::robotize(&mut samples, spec.sample_rate);

    let out_spec = hound::WavSpec {
        channels: 1,
        sample_rate: spec.sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut cursor = std::io::Cursor::new(Vec::new());
    {
        let mut writer = hound::WavWriter::new(&mut cursor, out_spec).map_err(|e| e.to_string())?;
        for sample in samples {
            writer.write_sample(sample).map_err(|e| e.to_string())?;
        }
        writer.finalize().map_err(|e| e.to_string())?;
    }
    Ok(cursor.into_inner())
}
