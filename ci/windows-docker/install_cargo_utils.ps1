[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

rustup --version
rustc --version
cargo --version

if ("$env:RUST_VERSION" -eq "1.71.1") {
    cargo install --locked cargo-c --version 0.9.26+cargo-0.74
} else {
    cargo install --locked cargo-c --version 0.10.3+cargo-0.81
}

if (!$?) {
  Write-Host "Failed to install cargo-c"
  Exit 1
}

cargo-cbuild --version
