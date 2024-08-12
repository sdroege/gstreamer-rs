[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

rustup --version
rustc --version
cargo --version

cargo install --locked cargo-c --version 0.9.26+cargo-0.72

if (!$?) {
  Write-Host "Failed to install cargo-c"
  Exit 1
}

cargo-cbuild --version
