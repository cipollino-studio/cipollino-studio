use crate::UserPref;


pub enum OnionSkinPastColor {}

impl UserPref for OnionSkinPastColor {

    type Type = elic::Color;

    fn default() -> Self::Type {
        pierro::Color::rgb(0.8588, 0.3764, 0.8196)
    }

    fn name() -> &'static str {
        "onion_skin_past_color"
    }

}

pub enum OnionSkinFutureColor {}

impl UserPref for OnionSkinFutureColor {

    type Type = elic::Color;

    fn default() -> Self::Type {
        pierro::Color::rgb(0.4666, 0.8588, 0.3764)
    }

    fn name() -> &'static str {
        "onion_skin_future_color"
    }

}
