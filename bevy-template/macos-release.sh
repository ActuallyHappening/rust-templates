set -e

TARGET=x86_64-apple-darwin
ENV_BINARY=$(cargo get --name)

cargo +nightly build --release --target $TARGET

mkdir -p $ENV_BINARY.app/Contents/MacOS
cp target/$TARGET/release/$ENV_BINARY $ENV_BINARY.app/Contents/MacOS/
cp -r assets $ENV_BINARY.app/Contents/MacOS/
hdiutil create -fs HFS+ -volname "$ENV_BINARY" -srcfolder $ENV_BINARY.app $ENV_BINARY.dmg
rm -rf $ENV_BINARY.app

mv $ENV_BINARY.dmg "releases/$ENV_BINARY-macos-$(cargo get version --pretty).dmg"
