
cargo build --bin cipollino-studio --release --target x86_64-unknown-linux-gnu 

DIR=../build/linux_x86/Cipollino
rm -Rf $DIR 
mkdir -p $DIR

cp ../target/x86_64-unknown-linux-gnu/release/cipollino-studio $DIR/cipollino
cp client/libs/ffmpeg/linux_x86/ffmpeg $DIR/ffmpeg
