
#[cfg(not(target_arch = "wasm32"))]
pub mod native;
#[cfg(target_arch = "wasm32")]
pub mod web;

pub trait UserPref {
    type Type: serde::Serialize + for<'a> serde::Deserialize<'a>;

    fn default() -> Self::Type;
    fn name() -> &'static str;

}

pub struct UserPrefs {
    #[cfg(not(target_arch = "wasm32"))]
    prefs_path: std::path::PathBuf,
    #[cfg(not(target_arch = "wasm32"))]
    prefs: serde_json::Map<String, serde_json::Value>,

    #[cfg(target_arch = "wasm32")]
    storage: web_sys::Storage
}