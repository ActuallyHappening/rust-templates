set -e

TARGET=x86_64-pc-windows-gnu
ENV_BINARY=$(cargo get --name)

cargo +nightly b --target $TARGET --release

mkdir -p $ENV_BINARY.zipfolder
cp target/$TARGET/release/$ENV_BINARY.exe $ENV_BINARY.zipfolder/
cp -r assets $ENV_BINARY.zipfolder/
zip -r $ENV_BINARY.zip $ENV_BINARY.zipfolder
rm -rf $ENV_BINARY.zipfolder

mv $ENV_BINARY.zip "releases/$ENV_BINARY-windows-$(cargo get version --pretty).zip"
