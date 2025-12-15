$env:ErrorActionPreference='Stop'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

rustup --version
rustc --version
cargo --version

if ("$env:RUST_VERSION" -eq "1.83.0") {
    cargo install --locked cargo-c --version 0.10.11+cargo-0.86.0
} else {
    cargo install --locked cargo-c --version 0.10.19+cargo-0.93.0
}

if (!$?) {
  Write-Host "Failed to install cargo-c"
  Exit 1
}

if ("$env:RUST_VERSION" -eq "1.83.0") {
    cargo install --locked cargo-nextest@0.9.94
} else {
    cargo install --locked cargo-nextest
}

if (!$?) {
  Write-Host "Failed to install cargo-nextest"
  Exit 1
}

cargo-cbuild --version
cargo nextest --version

# Rust-based CLI unpacker
cargo install --locked ouch@0.5.1
if (!$?) {
  Write-Host "Failed to install ouch"
  Exit 1
}

# libclang for bindgen-cli (x64)
$libclang_url = 'https://gstreamer.freedesktop.org/data/src/mirror/libclang-20.1.2.tar.xz'
Invoke-WebRequest -Uri $libclang_url -Outfile "$env:TEMP\libclang-20.1.2.tar.xz"
ouch decompress -d $env:TEMP "$env:TEMP\libclang-20.1.2.tar.xz"
cp "$env:TEMP\libclang-20.1.2\x64\bin\libclang.dll" "$env:USERPROFILE\.cargo\bin"
Remove-Item -Recurse "$env:TEMP\libclang-20.1.2"
Remove-Item "$env:TEMP\libclang-20.1.2.tar.xz"

cargo install --locked bindgen-cli
if (!$?) {
  Write-Host "Failed to install bindgen"
  Exit 1
}
bindgen --version
