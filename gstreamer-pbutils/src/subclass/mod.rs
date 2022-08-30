// Take a look at the license at the top of the repository in the LICENSE file.

mod audio_visualizer;
pub use audio_visualizer::AudioVisualizerSetupToken;

pub mod prelude {
    pub use super::audio_visualizer::{AudioVisualizerImpl, AudioVisualizerImplExt};
}
