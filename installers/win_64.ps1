# Install the Rust toolchain.
Invoke-WebRequest -URI https://win.rustup.rs/x86_64 -OutFile rustup.exe

# Install Mandy.
cargo install --git https://github.com/angeldollface/mandy