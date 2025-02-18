
use crate::{Color, Stroke, TextStyle, UI};

/// The color scheme used throughout the UI. Defaults to a dark theme.
pub struct Theme {
    pub bg_dark: Color,
    pub bg_light: Color,
    pub bg_popup: Color,
    pub bg_button: Color,
    pub bg_text_field: Color,

    pub stroke: Color,
    
    pub text: Color,
    pub text_active: Color,
    pub link: Color,

    pub accent: Color,

    pub label_font_size: f32,

    pub widget_margin: f32,
    pub widget_rounding: f32,
    pub widget_stroke_width: f32,

    pub window_margin: f32,

    pub color_transition_animation_rate: f32
}

impl Default for Theme {

    fn default() -> Self {
        Self {
            bg_dark: Color::hex(0x2D2D31FF),
            bg_light: Color::hex(0x363739FF),
            bg_popup: Color::hex(0x373A3BFF),
            bg_button: Color::hex(0x55585AFF),
            bg_text_field: Color::hex(0x242428FF),

            stroke: Color::hex(0x1E1E1EFF),

            text: Color::hex(0xB9BDC1FF),
            text_active: Color::hex(0xE8ECEFFF),
            link: Color::hex(0x3d98ffFF),

            accent: Color::hex(0x6AC3C1FF),

            label_font_size: 14.0,

            widget_margin: 5.0,
            widget_rounding: 5.0,
            widget_stroke_width: 1.0,

            window_margin: 7.5,

            color_transition_animation_rate: 0.3
        }
    }

}

impl Theme {

    pub fn hovered_color(&self, base: Color) -> Color {
        base.darken(0.15)
    }

    pub fn pressed_color(&self, base: Color) -> Color {
        base.darken(0.3)
    }

    pub fn widget_stroke(&self) -> Stroke {
        Stroke::new(self.stroke, self.widget_stroke_width)
    }

}

pub fn label_text_style(ui: &mut UI) -> TextStyle {
    let theme = ui.style::<Theme>(); 
    TextStyle {
        color: theme.text,
        font_size: theme.label_font_size,
        line_height: 1.0,
        font: ui.text_font(),
    }
}