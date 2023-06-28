# Install the Rust toolchain.
Invoke-WebRequest -URI https://win.rustup.rs/i686 -OutFile rustup.exe

# Install Mandy.
cargo install --git https://github.com/angeldollface/mandy