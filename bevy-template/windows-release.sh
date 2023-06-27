set -e

TARGET_NAME=x86_64-pc-windows-gnu
ENV_BINARY=bevy_solver

cargo +nightly bwindows --release

mkdir -p $ENV_BINARY.zipfolder
cp target/$TARGET_NAME/release/$ENV_BINARY.exe $ENV_BINARY.zipfolder/
cp -r assets $ENV_BINARY.zipfolder/
zip -r $ENV_BINARY.zip $ENV_BINARY.zipfolder
rm -rf $ENV_BINARY.zipfolder

mv $ENV_BINARY.zip "releases/Awesome Duck Man $(cargo get version --pretty).zip"
