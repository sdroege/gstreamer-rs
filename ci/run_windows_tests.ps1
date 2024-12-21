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
    'gstreamer-mpegts',
    'gstreamer-mpegts/sys',
    'gstreamer-net',
    'gstreamer-pbutils',
    'gstreamer-player',
    'gstreamer-rtp',
    'gstreamer-rtsp',
    'gstreamer-rtsp-server',
    'gstreamer-sdp',
    'gstreamer-tag',
    'gstreamer-tag/sys',
    'gstreamer-video',
    'gstreamer-webrtc',
    'tutorials',
    'examples'
)

# "" is the default build, no flags appended
[string[]] $features_matrix = @(
    # "--no-default-features",
    # "--features=v1_18",
    # "--features=v1_20",
    "",
    "--all-features"
)

if ($env:FDO_CI_CONCURRENT)
{
    $ncpus = $env:FDO_CI_CONCURRENT
}
else
{
    $ncpus = (Get-WmiObject -Class Win32_ComputerSystem).NumberOfLogicalProcessors
}
Write-Host "Build Jobs: $ncpus"
$cargo_opts = @("--color=always", "--jobs=$ncpus")
$cargo_nextest_opts=@("--profile=ci", "--no-fail-fast", "--no-tests=pass")

function Move-Junit {
    param (
        $Features
    )

    if ($env:CI_PROJECT_DIR) {
        $parent = $env:CI_PROJECT_DIR
    } else {
        $parent = $PWD.path
    }
    Write-Host "Parent directory: $parent"

    $new_report_dir = "$parent/junit_reports/$crate/"
    If(!(test-path -PathType container $new_report_dir))
    {
        New-Item -Path "$new_report_dir" -ItemType "directory"
        if (!$?) {
            Write-Host "Failed to create directory: $new_report_dir"
            Exit 1
        }
    }

    if ($Features -eq "--all-features") {
        $suffix = "all"
    } elseif ($Features -eq "--no-default-features") {
        $suffix = "no-default"
    } else {
        $suffix = "default"
    }

    Move-Item "$parent/target/nextest/ci/junit.xml" "$new_report_dir/junit-$suffix.xml"
    if (!$?) {
        Write-Host "Failed to move junit file"
        Exit 1
    }
}

foreach($features in $features_matrix) {
    foreach($crate in $crates)
    {
        Write-Host "Building crate: $crate"
        Write-Host "Features: $features"
        $env:LocalFeatures = $features

        # Don't append feature flags if the string is null/empty
        # Or when we want to build without default features
        if ($env:LocalFeatures -and ($env:LocalFeatures -ne '--no-default-features')) {
            if ($crate -eq 'examples') {
                # FIXME: We can do --all-features for examples once we have gtk3 installed in the image
                $env:LocalFeatures = "--features=rtsp-server,rtsp-server-record,pango-cairo,overlay-composition,gst-play,gst-player,ges,image,cairo-rs,gst-video/v1_18,windows,gl"
            }

            if ($crate -eq 'tutorials') {
                $env:LocalFeatures = ''
            }
        }

        Write-Host "with features: $env:LocalFeatures"
        cargo build $cargo_opts --manifest-path $crate/Cargo.toml --all-targets $env:LocalFeatures

        if (!$?) {
            Write-Host "Failed to build crate: $crate"
            Exit 1
        }

        if (($crate -eq "gstreamer-tag/sys") -or ($crate -eq "gstreamer-mpegts/sys")) {
            Write-Host "Skipping tests for $crate"
            continue
        }

        $env:G_DEBUG="fatal_warnings"
        $env:RUST_BACKTRACE="1"
        cargo nextest run $cargo_nextest_opts $cargo_opts --manifest-path $crate/Cargo.toml $env:LocalFeatures
        if (!$?) {
            Write-Host "Tests failed to for crate: $crate"
            Exit 1
        }

        Move-Junit -Features $features
    }
}
