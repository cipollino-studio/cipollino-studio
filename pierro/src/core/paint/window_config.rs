
use crate::{vec2, Vec2};

pub struct WindowIcon {
    pub(crate) rgba: Vec<u8>,
    pub(crate) width: u32,
    pub(crate) height: u32
}

impl WindowIcon {

    pub fn new(width: u32, height: u32, rgba: Vec<u8>) -> Self {
        assert_eq!(rgba.len(), (width * height * 4) as usize, "invalid pixel data size.");
        Self {
            rgba,
            width,
            height,
        }
    }

}

#[macro_export]
macro_rules! include_icon {
    ($path: expr) => {
        {
            let data = include_bytes!($path);
            let image = image::load_from_memory(data).unwrap().into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            crate::WindowIcon::new(width, height, rgba)
        } 
    };
}

pub struct WindowConfig {
    pub(crate) title: String,
    pub(crate) min_size: Vec2,
    pub(crate) maximize: bool,
    pub(crate) icon: WindowIcon
}

impl Default for WindowConfig {

    fn default() -> Self {
        Self {
            title: "Pierro Application".to_string(),
            min_size: vec2(400.0, 300.0),
            maximize: false,
            icon: include_icon!("../../../res/default_icon.png")
        }
    }

}

impl WindowConfig {
    
    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = min_size;
        self
    }

    pub fn maximize_window(mut self) -> Self {
        self.maximize = true;
        self
    }

    pub fn with_icon(mut self, icon: WindowIcon) -> Self {
        self.icon = icon;
        self
    }

}
