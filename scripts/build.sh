# Changing directory.
cd ..

# Adding all Mac OSX architectures.
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin

# Adding the most important Linux architectures.
rustup target add aarch64-unknown-linux-musl
rustup target add i686-unknown-linux-musl
rustup target add x86_64-unknown-linux-musl

# Adding all Windows architectures.
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu

# Adding all Android architectures.
rustup target add arm-linux-androideabi
rustup target add aarch64-linux-android
rustup target add i686-linux-android

# Compiling added Mac OSX targets.
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Compiling added Linux targets.
cargo build --release --target aarch64-unknown-linux-musl
cargo build --release --target i686-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl

# Compiling added Windows targets.
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target add i686-pc-windows-gnu

# Compiling added Android targets.
cargo build --release --target arm-linux-androideabi
cargo build --release --target aarch64-linux-android
cargo build --release --target i686-linux-android

mkdir mandy-release

# Move binaries to "mandy-release".

# Mac OSX binaries.
mv target/aarch64-apple-darwin/release/mandy mandy-release/mandy-aarch64-apple-darwin.bin
mv target/x86_64-apple-darwin/release/mandy mandy-release/mandy-x86_64-apple-darwin.bin

# Linux binaries.
mv target/aarch64-unknown-linux-musl/release/mandy mandy-release/mandy-aarch64-unknown-linux-musl.bin
mv target/i686-unknown-linux-musl/release/mandy mandy-release/mandy-i686-unknown-linux-musl.bin
mv target/x86_64-unknown-linux-musl/release/mandy mandy-release/mandy-x86_64-unknown-linux-musl.bin

# Windows binaries.
mv target/x86_64-pc-windows-gnu/release/mandy mandy-release/mandy-x86_64-pc-windows-gnu.exe
mv target/i686-pc-windows-gnu/release/mandy mandy-release/mandy-i686-pc-windows-gnu.exe

# Android binaries.
mv target/arm-linux-androideabi/release/mandy mandy-release/mandy-arm-linux-androideabi.bin
mv target/aarch64-linux-android/release/mandy mandy-release/mandy-aarch64-linux-android.bin
mv target/i686-linux-android/release/mandy mandy-release/mandy-i686-linux-android.bin
