// Take a look at the license at the top of the repository in the LICENSE file.

mod play_video_renderer;

pub mod prelude {
    #[doc(hidden)]
    pub use gst::subclass::prelude::*;

    pub use super::play_video_renderer::{PlayVideoRendererImpl, PlayVideoRendererImplExt};
}
