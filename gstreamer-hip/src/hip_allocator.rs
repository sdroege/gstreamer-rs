// Take a look at the license at the top of the repository in the LICENSE file.

use crate::HipAllocator;

impl HipAllocator {
    pub fn new() -> Self {
        assert_initialized_main_thread!();
        glib::Object::new()
    }
}

impl Default for HipAllocator {
    fn default() -> Self {
        Self::new()
    }
}
