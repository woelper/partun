# ARM6
cross build --target arm-unknown-linux-gnueabihf  --release
# ARM7
cross build --target armv7-unknown-linux-gnueabihf  --release

cp target/arm-unknown-linux-gnueabihf/release/partun release/arm6/
cp target/armv7-unknown-linux-gnueabihf/release/partun release/arm7/
git add release
git commit -m "Cross compile release"
