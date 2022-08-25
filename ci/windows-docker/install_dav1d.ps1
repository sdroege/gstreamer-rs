[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12;

# Download gstreamer and all its subprojects
git clone -b 1.0.0 --depth 1 https://code.videolan.org/videolan/dav1d.git C:\dav1d
if (!$?) {
  Write-Host "Failed to clone dav1d"
  Exit 1
}

Set-Location C:\dav1d

# This is fine, we are not going to use the GtkMedia* apis
$env:MESON_ARGS = "--prefix=C:\gst-install\"

Write-Output "Building dav1d"
cmd.exe /C "C:\BuildTools\Common7\Tools\VsDevCmd.bat -host_arch=amd64 -arch=amd64 && meson _build $env:MESON_ARGS && meson compile -C _build && ninja -C _build install"

if (!$?) {
  Write-Host "Failed to build and install dav1d"
  Exit 1
}

cd C:\
cmd /c rmdir /s /q  C:\dav1d
if (!$?) {
  Write-Host "Failed to remove dav1d checkout"
  Exit 1
}
