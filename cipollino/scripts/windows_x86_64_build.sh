
cargo build --bin cipollino-studio --release --target x86_64-pc-windows-gnu

DIR=../build/windows_x86_64/Cipollino
rm -Rf $DIR 
mkdir -p $DIR

cp ../target/x86_64-pc-windows-gnu/release/cipollino-studio.exe $DIR/Cipollino.exe
cp client/libs/ffmpeg/windows_x86/ffmpeg.exe $DIR/ffmpeg.exe
