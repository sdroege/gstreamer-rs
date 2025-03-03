#![allow(clippy::non_send_fields_in_send_ty)]

use anyhow::Result;

#[path = "../glupload.rs"]
mod glupload;
use glupload::*;

#[path = "../examples-common.rs"]
pub mod examples_common;

fn example_main() -> Result<()> {
    let app = App::new(None)?;
    app.run()
}

fn main() -> Result<()> {
    examples_common::run(example_main)
}
