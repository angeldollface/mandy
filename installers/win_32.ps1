# Install the Rust toolchain if it isn't installed.
if(Get-Command cargo){
    Write-Output "Required toolchain installed, proceeding."
}
else {
    Invoke-WebRequest -URI https://win.rustup.rs/i686 -OutFile rustup.exe
    ./rustup.exe
}

# Install Mandy.
cargo install --git https://github.com/angeldollface/mandy