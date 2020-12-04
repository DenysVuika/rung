# macOS

cross build --target x86_64-apple-darwin --release

rm -rf ./dist/x86_64-apple-darwin && \
mkdir ./dist/x86_64-apple-darwin && \
cp LICENSE ./dist/x86_64-apple-darwin/ && \
cp README.md ./dist/x86_64-apple-darwin/ && \
cp ./target/x86_64-apple-darwin/release/rung ./dist/x86_64-apple-darwin/ && \
(cd ./dist/x86_64-apple-darwin/ && tar -cvzf rung-x86_64-apple-darwin.tar.gz LICENSE README.md rung)

# Linux

cross build --target x86_64-unknown-linux-musl --release

rm -rf ./dist/x86_64-unknown-linux-musl && \
mkdir ./dist/x86_64-unknown-linux-musl && \
cp LICENSE ./dist/x86_64-unknown-linux-musl/ && \
cp README.md ./dist/x86_64-unknown-linux-musl/ && \
cp ./target/x86_64-unknown-linux-musl/release/rung ./dist/x86_64-unknown-linux-musl/ && \
(cd ./dist/x86_64-unknown-linux-musl/ && tar -cvzf rung-x86_64-unknown-linux-musl.tar.gz LICENSE README.md rung)
