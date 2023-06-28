# set -e

# TARGET=x86_64-unknown-linux-gnu
# ENV_BINARY=$(cargo get --name)

# cross build --release --target $TARGET

# mkdir linux
# cp target/$TARGET/release/$ENV_BINARY linux/
# cp -r assets linux/

# # cd linux
# zip --recurse-paths ../$ENV_BINARY.zip linux/
# # cd ..
