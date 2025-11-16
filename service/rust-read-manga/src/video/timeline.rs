use std::path::PathBuf;

/// Represents a single frame in the video timeline.
#[derive(Debug)]
pub struct Frame {
    pub path: PathBuf,
    pub duration: f32,
}

/// Represents the video timeline, composed of multiple frames.
#[derive(Debug)]
pub struct Timeline {
    pub frames: Vec<Frame>,
}

impl Timeline {
    /// Creates a new, empty `Timeline`.
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    /// Adds a new frame to the timeline.
    ///
    /// # Arguments
    /// * `path` - The path to the image file for this frame.
    /// * `duration` - The display duration of this frame in seconds.
    pub fn add_frame(&mut self, path: PathBuf, duration: f32) {
        self.frames.push(Frame { path, duration });
    }
}