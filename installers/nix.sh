# Install the Rust toolchain.
if ! [ -x "$(command -v cargo)" ]; then 
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
else
    echo "Required toolchain installed, proceeding."
fi

# Install Mandy.
cargo install --git https://github.com/angeldollface/mandy --force