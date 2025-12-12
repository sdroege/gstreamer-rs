$env:ErrorActionPreference='Stop'

[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

git clone https://github.com/fraunhoferhhi/vvdec.git C:\vvdec
if (!$?) {
  Write-Host "Failed to clone vvdec"
  Exit 1
}

Set-Location C:\vvdec
git checkout v3.1.0

# This is fine, we are not going to use the GtkMedia* apis
$env:CMAKE_ARGS = "-GNinja -DCMAKE_INSTALL_PREFIX=C:\gst-install\ -DBUILD_SHARED_LIBS=ON"

Write-Output "Building vvdec"
cmd.exe /C "C:\BuildTools\Common7\Tools\VsDevCmd.bat -host_arch=amd64 -arch=amd64 && cmake -S . -B _build $env:CMAKE_ARGS && ninja -C _build && ninja -C _build install"

if (!$?) {
  Write-Host "Failed to build and install vvdec"
  Exit 1
}

cd C:\
cmd /c rmdir /s /q  C:\vvdec
if (!$?) {
  Write-Host "Failed to remove vvdec checkout"
  Exit 1
}
