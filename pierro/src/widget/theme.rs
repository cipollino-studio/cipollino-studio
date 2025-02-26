
use crate::{Color, Margin, Rounding, Stroke, TextStyle, UI};

macro_rules! style {
    ($name: ident, $t: ty, $default: expr) => {
        pub struct $name;

        impl crate::Style for $name {
            type Value = $t;

            fn default() -> Self::Value {
                $default
            }
        }
    };
}

style!(BgDark, Color, Color::hex(0x2D2D31FF));
style!(BgLight, Color, Color::hex(0x363739FF));
style!(BgPopup, Color, Color::hex(0x373A3BFF));
style!(BgButton, Color, Color::hex(0x55585AFF));
style!(BgTextField, Color, Color::hex(0x242428FF));
style!(TextColor, Color, Color::hex(0xB9BDC1FF));
style!(ActiveTextColor, Color, Color::hex(0xE8ECEFFF));
style!(LinkColor, Color, Color::hex(0x3d98ffFF));
style!(AccentColor, Color, Color::hex(0x6AC3C1FF));

style!(LabelFontSize, f32, 14.0);

style!(WidgetStroke, Stroke, Stroke::new(Color::hex(0x1E1E1EFF), 1.0));
style!(WidgetMargin, Margin, Margin::same(5.0));
style!(WidgetRounding, Rounding, Rounding::same(5.0));

style!(WindowMarin, Margin, Margin::same(7.5));

style!(ColorTransitionRate, f32, 0.3);

style!(DividerLineGap, f32, 5.0);


pub fn hovered_color(base: Color) -> Color {
    base.darken(0.15)
}

pub fn pressed_color(base: Color) -> Color {
    base.darken(0.3)
}

pub fn label_text_style(ui: &mut UI) -> TextStyle {
    TextStyle {
        color: ui.style::<TextColor>(),
        font_size: ui.style::<LabelFontSize>(),
        line_height: 1.0,
        font: ui.text_font(),
    }
}
