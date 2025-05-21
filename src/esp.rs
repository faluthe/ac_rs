use crate::gl;

use anyhow::Result;

pub unsafe fn run() -> Result<()> {
    // Example draw
    gl::draw_rect(100.0, 100.0, 500.0, 500.0, 1.0, 0.0, 0.0);

    Ok(())
}
