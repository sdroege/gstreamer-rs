$env:ErrorActionPreference='Stop'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

$rustup_url = 'https://win.rustup.rs/x86_64'

Invoke-WebRequest -Uri $rustup_url -OutFile C:\rustup-init.exe

if (!$?) {
  Write-Host "Failed to download rustup"
  Exit 1
}

C:\rustup-init.exe -y --profile minimal --default-toolchain $env:RUST_VERSION

if (!$?) {
  Write-Host "Failed to install rust"
  Exit 1
}
