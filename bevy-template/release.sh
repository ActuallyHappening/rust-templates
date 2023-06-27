set -e

./macos-release.sh
trunk build --release
./windows-release.sh
