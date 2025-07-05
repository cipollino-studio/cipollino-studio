
#[cfg(target_os = "macos")]
#[cfg(target_arch = "aarch64")]
#[cfg(debug_assertions)]
pub const FFMPEG_PATH: &'static str = "./cipollino/client/libs/ffmpeg/macos_arm64/ffmpeg"; 

#[cfg(target_os = "macos")]
#[cfg(target_arch = "aarch64")]
#[cfg(not(debug_assertions))]
pub const FFMPEG_PATH: &'static str = "../ffmpeg"; 

#[cfg(target_os = "windows")]
#[cfg(target_arch = "x86_64")]
#[cfg(debug_assertions)]
pub const FFMPEG_PATH: &'static str = "./cipollino/client/libs/ffmpeg/windows_x86/ffmpeg.exe"; 

#[cfg(target_os = "windows")]
#[cfg(not(debug_assertions))]
pub const FFMPEG_PATH: &'static str = "./ffmpeg.exe"; 

#[cfg(target_os = "linux")]
#[cfg(target_arch = "x86_64")]
#[cfg(debug_assertions)]
pub const FFMPEG_PATH: &'static str = "./cipollino/client/libs/ffmpeg/linux_x86/ffmpeg"; 

#[cfg(target_os = "linux")]
#[cfg(target_arch = "aarch64")]
#[cfg(debug_assertions)]
pub const FFMPEG_PATH: &'static str = "./cipollino/client/libs/ffmpeg/linux_arm64/ffmpeg"; 

#[cfg(target_os = "linux")]
#[cfg(not(debug_assertions))]
pub const FFMPEG_PATH: &'static str = "./ffmpeg";
