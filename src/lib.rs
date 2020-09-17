use noisy_float::{FloatChecker, NoisyFloat};
use std::io::Cursor;
use num_traits::Float;

/// Like `noisy_float`'s `NumChecker`, except it plays the
/// [Wilhelm Scream](https://en.wikipedia.org/wiki/Wilhelm_scream) instead of panicking.
pub struct ScreamingNanChecker;

impl<F: Float> FloatChecker<F> for ScreamingNanChecker {
    fn check(value: F) -> bool {
        !value.is_nan()
    }

    fn assert(value: F) {
        if !Self::check(value) {
            let scream: &[u8] = include_bytes!("scream.ogg");
            let device = rodio::default_output_device().unwrap();
            let sink = rodio::play_once(&device, Cursor::new(scream)).unwrap();
            sink.play();
            sink.sleep_until_end();
        }
    }
}

/// A floating point number behaving like `f32` that screams when it is constructed from a `NaN`.
pub type S32 = NoisyFloat<f32, ScreamingNanChecker>;

/// A floating point number behaving like `f64` that screams when it is constructed from a `NaN`.
pub type S64 = NoisyFloat<f64, ScreamingNanChecker>;

/// Shorthand for `S32::new(value)`.
pub fn s32(value: f32) -> S32 {
    S32::new(value)
}

/// Shorthand for `S64::new(value)`.
pub fn s64(value: f64) -> S64 {
    S64::new(value)
}
