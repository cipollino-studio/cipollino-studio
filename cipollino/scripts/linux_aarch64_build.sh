
cargo build --bin cipollino-studio --release --target aarch64-unknown-linux-gnu

DIR=../build/linux_aarch64/cipollino
rm -Rf $DIR 
mkdir -p $DIR

cp ../target/aarch64-unknown-linux-gnu/release/cipollino-studio $DIR/cipollino
cp client/libs/ffmpeg/linux_arm64/ffmpeg $DIR/ffmpeg
