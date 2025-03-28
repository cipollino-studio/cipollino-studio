
/// The state of the scene edit preview.
/// When drawing/editing the scene, we only queue an operation in the project client
/// when the user is done with their modification(eg they finished drawing a line).
/// This way we don't send a million messages to the server as we draw a line.
/// However, to account for this, we need to preview the user's actions separately.
/// This struct contains the information necessary to render this preview. 
pub struct ScenePreview {

    /// Preview of a stroke being drawn.
    /// Gets rendered on the currently active layer
    pub stroke_preview: Option<malvina::StrokeMesh>,

    /// Transform the selected objects in the scene with some matrix
    pub selection_transform: malvina::Mat4,

    /// By default, previews should dissapear unless they are explicitly requested.
    /// At the end of the frame, if this flag is false, all previews will be removed.
    pub keep_preview: bool
}

impl ScenePreview {

    pub fn new() -> Self {
        Self {
            selection_transform: malvina::Mat4::IDENTITY,
            stroke_preview: None,
            keep_preview: false,
        }
    }

    pub fn end_frame(&mut self) {
        if !self.keep_preview {
            self.stroke_preview = None;
            self.selection_transform = elic::Mat4::IDENTITY;
        }
        self.keep_preview = false;
    }

}
