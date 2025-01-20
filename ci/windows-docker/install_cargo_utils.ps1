$env:ErrorActionPreference='Stop'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

rustup --version
rustc --version
cargo --version

if ("$env:RUST_VERSION" -eq "1.71.1") {
    cargo install --locked cargo-c --version 0.9.26+cargo-0.74
} else {
    cargo install --locked cargo-c --version 0.10.9+cargo-0.85
}

if (!$?) {
  Write-Host "Failed to install cargo-c"
  Exit 1
}

# Multiple dependencies of cargo-nextest require 1.74/1.75 nowadays
if ("$env:RUST_VERSION" -eq "1.71.1") {
    cargo install --locked cargo-nextest@0.9.67
} else {
    cargo install --locked cargo-nextest
}

if (!$?) {
  Write-Host "Failed to install cargo-nextest"
  Exit 1
}

cargo-cbuild --version
cargo nextest --version
