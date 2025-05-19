
cargo build --bin cipollino-studio --release --target aarch64-apple-darwin

rm -Rf ../build/macos_aarch64
DIR=../build/macos_aarch64/Cipollino.app
rm -Rf $DIR
mkdir -p $DIR/Contents/MacOS
mkdir -p $DIR/Contents/Resources

cp ../target/aarch64-apple-darwin/release/cipollino-studio $DIR/Contents/MacOS/Cipollino
cp client/res/macos/icon.icns $DIR/Contents/Resources/icon.icns
cp client/res/macos/Info.plist $DIR/Contents/

cp client/libs/ffmpeg/macos_arm64/ffmpeg $DIR/Contents/

codesign -s - --deep --force $DIR

create-dmg --icon-size 50 --app-drop-link 200 100 --icon "Cipollino.app" 0 100 ../build/macos_aarch64/Cipollino.dmg ../build/macos_aarch64