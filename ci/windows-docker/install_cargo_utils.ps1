$env:ErrorActionPreference='Stop'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

rustup --version
rustc --version
cargo --version

if ("$env:RUST_VERSION" -eq "1.80.1") {
    cargo install --locked cargo-c --version 0.10.5+cargo-0.93
} else {
    cargo install --locked cargo-c --version 0.10.11+cargo-0.86
}

if (!$?) {
  Write-Host "Failed to install cargo-c"
  Exit 1
}

if ("$env:RUST_VERSION" -eq "1.80.1") {
    cargo install --locked cargo-nextest@0.9.85
} else {
    cargo install --locked cargo-nextest
}

if (!$?) {
  Write-Host "Failed to install cargo-nextest"
  Exit 1
}

cargo-cbuild --version
cargo nextest --version
