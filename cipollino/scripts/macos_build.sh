
cargo build --bin cipollino-studio --release --target aarch64-apple-darwin

mkdir -p ../build/Cipollino.app/Contents/MacOS
mkdir -p ../build/Cipollino.app/Contents/Resources

cp ../target/aarch64-apple-darwin/release/cipollino-studio ../build/Cipollino.app/Contents/MacOS/Cipollino
cp client/res/macos/icon.icns ../build/Cipollino.app/Contents/Resources/icon.icns
cp client/res/macos/Info.plist ../build/Cipollino.app/Contents/

cp client/libs/ffmpeg/macos_arm64/ffmpeg ../build/Cipollino.app/Contents/
