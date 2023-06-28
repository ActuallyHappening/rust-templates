set -e

./macos-release.sh
trunk build --release
./windows-release.sh
# ./linux-release.sh # What is linux and who would run game on it?
