# Add the precompiled gst binaries to the path
$env:Path += ';C:\bin\'
$env:PKG_CONFIG_PATH = "C:/lib/pkgconfig"

# List of all the crates we want to build
# We need to do this manually to avoid trying
# to build egl,wayland,x11 etc, which can't
# work on windows
[string[]] $crates = @(
    'gstreamer',
    # Unix specific atm
    # 'gstreamer-allocators'
    'gstreamer-app',
    'gstreamer-audio',
    'gstreamer-base',
    'gstreamer-check',
    'gstreamer-controller',
    'gstreamer-editing-services',
    'gstreamer-gl',
    # 'gstreamer-gl/egl',
    # 'gstreamer-gl/wayland',
    # 'gstreamer-gl/x11',
    # only has sys
    # 'gstreamer-mpegts',
    'gstreamer-mpegts/sys',
    'gstreamer-net',
    'gstreamer-pbutils',
    'gstreamer-player',
    'gstreamer-rtp',
    'gstreamer-rtsp',
    'gstreamer-rtsp-server',
    'gstreamer-sdp',
    # only has sys
    # 'gstreamer-tag',
    'gstreamer-tag/sys',
    'gstreamer-video',
    'gstreamer-webrtc',
    'tutorials',
    'examples'
)

foreach($crate in $crates)
{
    Write-Host "Building crate: $crate"
    Write-Host "Features: $env:FEATURES"
    $env:LocalFeatures = $env:FEATURES

    # Don't append feature flags if the string is null/empty
    # Or when we want to build without default features
    if ($env:LocalFeatures -and ($env:LocalFeatures -ne '--no-default-features')) {
        if ($crate -eq 'gstreamer') {
            $env:LocalFeatures += "ser_de,"
        }

        if ($crate -eq 'examples') {
            # FIXME: We can do --all-features for examples once we have gtk installed in the image
            $env:LocalFeatures = "--features=rtsp-server,rtsp-server-record,pango-cairo,overlay-composition"
        }

        if ($crate -eq 'tutorials') {
            $env:LocalFeatures = ''
        }
    }

    Write-Host "with features: $env:LocalFeatures"
    cargo build --color=always --manifest-path $crate/Cargo.toml --all-targets $env:LocalFeatures

    if (!$?) {
        Write-Host "Failed to build crate: $crate"
        Exit 1
    }

    if (($crate -eq "gstreamer-tag/sys") -or ($crate -eq "gstreamer-mpegts/sys")) {
        Write-Host "Skipping tests for $crate"
        continue
    }

    $env:G_DEBUG="fatal_warnings"
    cargo test --no-fail-fast --color=always --manifest-path $crate/Cargo.toml $env:LocalFeatures

    if (!$?) {
        Write-Host "Tests failed to for crate: $crate"
        Exit 1
    }
}