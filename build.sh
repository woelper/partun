# Note: This needs docker

cargo install --force cargo-strip
cargo install cross
# ARM6
cross build --target arm-unknown-linux-gnueabihf  --release
cargo strip --target arm-unknown-linux-gnueabihf
# ARM7
cross build --target armv7-unknown-linux-gnueabihf  --release
cargo strip --target armv7-unknown-linux-gnueabihf

cp target/arm-unknown-linux-gnueabihf/release/partun release/arm6/
cp target/armv7-unknown-linux-gnueabihf/release/partun release/arm7/
#git add release
#git commit -m "Cross compile release"
