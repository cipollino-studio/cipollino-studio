
use pierro::Style;

use crate::{color_picker, AppSystems, OnionSkinFutureColor, OnionSkinPastColor, UserPref};

pub enum AccentColor {}

impl UserPref for AccentColor {
    type Type = elic::Color;

    fn default() -> elic::Color {
        pierro::theme::AccentColor::default() 
    }

    fn name() -> &'static str {
        "accent_color"
    }
}

fn color_setting<P: UserPref<Type = elic::Color>>(ui: &mut pierro::UI, systems: &mut AppSystems) {
    let mut color = systems.prefs.get::<P>();
    let prev_color = color;
    color_picker(ui, &mut color);
    if prev_color != color {
        systems.prefs.set::<P>(&color);
    }
}

pub(super) fn appearance(ui: &mut pierro::UI, systems: &mut AppSystems) {

    pierro::key_value_layout(ui, |builder| {
        builder.labeled("Onion Skin Past Color:", |ui| {
            color_setting::<OnionSkinPastColor>(ui, systems);
        });
        builder.labeled("Onion Skin Future Color:", |ui| {
            color_setting::<OnionSkinFutureColor>(ui, systems);
        });
        builder.labeled("Accent Color:", |ui| {
            color_setting::<AccentColor>(ui, systems);
        });
    })

}
