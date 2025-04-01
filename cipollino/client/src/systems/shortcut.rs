
use super::{AppSystems, UserPref};

pub trait Shortcut: Sized {

    const NAME: &'static str;

    fn default() -> pierro::KeyboardShortcut;

    fn used_globally(ui: &mut pierro::UI, systems: &mut AppSystems) -> bool {
        systems.prefs.get::<Self>().used_globally(ui)
    }

}

impl<T: Shortcut> UserPref for T {

    type Type = pierro::KeyboardShortcut;

    fn default() -> Self::Type {
        Self::default()
    }

    fn name() -> &'static str {
        Self::NAME
    }

}

#[macro_export]
macro_rules! keyboard_shortcut {
    ($name: ident, $default_key: ident, $default_modifiers: expr) => {

        paste::paste! {

            pub enum $name {}

            impl crate::Shortcut for $name {

                const NAME: &'static str = stringify!([< $name:snake >]);

                fn default() -> pierro::KeyboardShortcut {
                    pierro::KeyboardShortcut::new($default_modifiers, pierro::Key::$default_key)
                }

            }

        }

    };
}

