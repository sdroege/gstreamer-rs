[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

rustup --version
rustc --version
cargo --version

if ("$RUST_VERSION" -eq "nightly") {
    cargo install --locked cargo-c --version 0.10.3+cargo-0.81
} else {
    cargo install --locked cargo-c --version 0.9.26+cargo-0.74
}

if (!$?) {
  Write-Host "Failed to install cargo-c"
  Exit 1
}

cargo-cbuild --version
