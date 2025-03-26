
/// The state of the scene edit preview.
/// When drawing/editing the scene, we only queue an operation in the project client
/// when the user is done with their modification(eg they finished drawing a line).
/// This way we don't send a million messages to the server as we draw a line.
/// However, to account for this, we need to preview the user's actions separately.
/// This struct contains the information necessary to render this preview. 
pub struct ScenePreview {
    pub stroke_preview: Option<malvina::StrokeMesh>,

    /// By default, previews should dissapear unless they are explicitly requested.
    /// At the end of the frame, if this flag is false, all previews will be removed.
    pub keep_preview: bool
}

impl ScenePreview {

    pub fn new() -> Self {
        Self {
            stroke_preview: None,
            keep_preview: false,
        }
    }

    pub fn end_frame(&mut self) {
        if !self.keep_preview {
            self.stroke_preview = None;
        }
        self.keep_preview = false;
    }

}
