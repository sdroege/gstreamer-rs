// Take a look at the license at the top of the repository in the LICENSE file.

mod player_video_renderer;

pub mod prelude {
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;
    #[doc(hidden)]
    pub use gst::subclass::prelude::*;

    pub use super::player_video_renderer::PlayerVideoRendererImpl;
}
