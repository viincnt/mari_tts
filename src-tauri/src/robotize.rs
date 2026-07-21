/// Turns a clean speech recording into the flat, crunchy "80s dictaphone
/// robot" character: narrow telephone-band filtering, a stepped/lo-fi
/// sample-and-hold, and heavy bit reduction.
pub fn robotize(samples: &mut [i16], sample_rate: u32) {
    high_pass(samples, sample_rate, 300.0);
    low_pass(samples, sample_rate, 3400.0);
    sample_and_hold(samples, 3);
    bit_crush(samples, 6);
    normalize(samples, 0.85);
}

fn high_pass(samples: &mut [i16], sample_rate: u32, cutoff_hz: f32) {
    let dt = 1.0 / sample_rate as f32;
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff_hz);
    let alpha = rc / (rc + dt);
    let mut prev_in = 0.0f32;
    let mut prev_out = 0.0f32;
    for s in samples.iter_mut() {
        let x = *s as f32;
        let y = alpha * (prev_out + x - prev_in);
        prev_in = x;
        prev_out = y;
        *s = y.clamp(i16::MIN as f32, i16::MAX as f32) as i16;
    }
}

fn low_pass(samples: &mut [i16], sample_rate: u32, cutoff_hz: f32) {
    let dt = 1.0 / sample_rate as f32;
    let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff_hz);
    let alpha = dt / (rc + dt);
    let mut prev_out = 0.0f32;
    for s in samples.iter_mut() {
        let x = *s as f32;
        let y = prev_out + alpha * (x - prev_out);
        prev_out = y;
        *s = y.clamp(i16::MIN as f32, i16::MAX as f32) as i16;
    }
}

/// Holds each sample for `factor` frames, producing the stepped, aliased
/// texture of a cheap lo-fi digitizer.
fn sample_and_hold(samples: &mut [i16], factor: usize) {
    if factor <= 1 {
        return;
    }
    let mut i = 0;
    while i < samples.len() {
        let held = samples[i];
        let end = (i + factor).min(samples.len());
        for s in &mut samples[i..end] {
            *s = held;
        }
        i += factor;
    }
}

fn bit_crush(samples: &mut [i16], bits: u32) {
    let levels = 1i32 << bits;
    let step = (1i32 << 16) / levels;
    for s in samples.iter_mut() {
        let v = *s as i32;
        let q = (v / step) * step;
        *s = q.clamp(i16::MIN as i32, i16::MAX as i32) as i16;
    }
}

/// Filtering/crushing bleeds off amplitude; boost back up (never down) so
/// the result isn't quieter than the source.
fn normalize(samples: &mut [i16], target: f32) {
    let max = samples.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0);
    if max == 0 {
        return;
    }
    let scale = (target * i16::MAX as f32) / max as f32;
    if scale <= 1.0 {
        return;
    }
    for s in samples.iter_mut() {
        let v = (*s as f32 * scale).clamp(i16::MIN as f32, i16::MAX as f32);
        *s = v as i16;
    }
}
