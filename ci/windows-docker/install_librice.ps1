$env:ErrorActionPreference='Stop'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

git clone --depth=1 https://github.com/ystreet/librice -b v0.3.0
if (!$?) {
  Write-Host "Failed to clone librice"
  Exit 1
}
Set-Location C:\librice
cargo cinstall -p rice-proto --prefix C:\gst-install\ --libdir lib
if (!$?) {
  Write-Host "Failed to build/install librice"
  Exit 1
}
cd C:\
cmd /c rmdir /s /q  C:\librice
if (!$?) {
  Write-Host "Failed to remove librice checkout"
  Exit 1
}
