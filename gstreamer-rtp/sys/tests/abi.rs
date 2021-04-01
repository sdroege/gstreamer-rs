// Generated by gir (https://github.com/gtk-rs/gir @ 28780fd)
// from gir-files (https://github.com/gtk-rs/gir-files @ 6088bb6)
// from gst-gir-files (https://gitlab.freedesktop.org/gstreamer/gir-files-rs.git @ 208138a)
// DO NOT EDIT

use gstreamer_rtp_sys::*;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::mem::{align_of, size_of};
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["gstreamer-rtp-1.0"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Compiler, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Compiler { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}", &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    (
        "GstRTCPBuffer",
        Layout {
            size: size_of::<GstRTCPBuffer>(),
            alignment: align_of::<GstRTCPBuffer>(),
        },
    ),
    (
        "GstRTCPFBType",
        Layout {
            size: size_of::<GstRTCPFBType>(),
            alignment: align_of::<GstRTCPFBType>(),
        },
    ),
    (
        "GstRTCPPacket",
        Layout {
            size: size_of::<GstRTCPPacket>(),
            alignment: align_of::<GstRTCPPacket>(),
        },
    ),
    (
        "GstRTCPSDESType",
        Layout {
            size: size_of::<GstRTCPSDESType>(),
            alignment: align_of::<GstRTCPSDESType>(),
        },
    ),
    (
        "GstRTCPType",
        Layout {
            size: size_of::<GstRTCPType>(),
            alignment: align_of::<GstRTCPType>(),
        },
    ),
    (
        "GstRTCPXRType",
        Layout {
            size: size_of::<GstRTCPXRType>(),
            alignment: align_of::<GstRTCPXRType>(),
        },
    ),
    (
        "GstRTPBaseAudioPayload",
        Layout {
            size: size_of::<GstRTPBaseAudioPayload>(),
            alignment: align_of::<GstRTPBaseAudioPayload>(),
        },
    ),
    (
        "GstRTPBaseAudioPayloadClass",
        Layout {
            size: size_of::<GstRTPBaseAudioPayloadClass>(),
            alignment: align_of::<GstRTPBaseAudioPayloadClass>(),
        },
    ),
    (
        "GstRTPBaseDepayload",
        Layout {
            size: size_of::<GstRTPBaseDepayload>(),
            alignment: align_of::<GstRTPBaseDepayload>(),
        },
    ),
    (
        "GstRTPBaseDepayloadClass",
        Layout {
            size: size_of::<GstRTPBaseDepayloadClass>(),
            alignment: align_of::<GstRTPBaseDepayloadClass>(),
        },
    ),
    (
        "GstRTPBasePayload",
        Layout {
            size: size_of::<GstRTPBasePayload>(),
            alignment: align_of::<GstRTPBasePayload>(),
        },
    ),
    (
        "GstRTPBasePayloadClass",
        Layout {
            size: size_of::<GstRTPBasePayloadClass>(),
            alignment: align_of::<GstRTPBasePayloadClass>(),
        },
    ),
    (
        "GstRTPBuffer",
        Layout {
            size: size_of::<GstRTPBuffer>(),
            alignment: align_of::<GstRTPBuffer>(),
        },
    ),
    (
        "GstRTPBufferFlags",
        Layout {
            size: size_of::<GstRTPBufferFlags>(),
            alignment: align_of::<GstRTPBufferFlags>(),
        },
    ),
    (
        "GstRTPBufferMapFlags",
        Layout {
            size: size_of::<GstRTPBufferMapFlags>(),
            alignment: align_of::<GstRTPBufferMapFlags>(),
        },
    ),
    (
        "GstRTPPayload",
        Layout {
            size: size_of::<GstRTPPayload>(),
            alignment: align_of::<GstRTPPayload>(),
        },
    ),
    (
        "GstRTPPayloadInfo",
        Layout {
            size: size_of::<GstRTPPayloadInfo>(),
            alignment: align_of::<GstRTPPayloadInfo>(),
        },
    ),
    (
        "GstRTPProfile",
        Layout {
            size: size_of::<GstRTPProfile>(),
            alignment: align_of::<GstRTPProfile>(),
        },
    ),
    (
        "GstRTPSourceMeta",
        Layout {
            size: size_of::<GstRTPSourceMeta>(),
            alignment: align_of::<GstRTPSourceMeta>(),
        },
    ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(gint) GST_RTCP_FB_TYPE_INVALID", "0"),
    ("GST_RTCP_MAX_BYE_SSRC_COUNT", "31"),
    ("GST_RTCP_MAX_RB_COUNT", "31"),
    ("GST_RTCP_MAX_SDES", "255"),
    ("GST_RTCP_MAX_SDES_ITEM_COUNT", "31"),
    ("(gint) GST_RTCP_PSFB_TYPE_AFB", "15"),
    ("(gint) GST_RTCP_PSFB_TYPE_FIR", "4"),
    ("(gint) GST_RTCP_PSFB_TYPE_PLI", "1"),
    ("(gint) GST_RTCP_PSFB_TYPE_RPSI", "3"),
    ("(gint) GST_RTCP_PSFB_TYPE_SLI", "2"),
    ("(gint) GST_RTCP_PSFB_TYPE_TSTN", "6"),
    ("(gint) GST_RTCP_PSFB_TYPE_TSTR", "5"),
    ("(gint) GST_RTCP_PSFB_TYPE_VBCN", "7"),
    ("GST_RTCP_REDUCED_SIZE_VALID_MASK", "57592"),
    ("(gint) GST_RTCP_RTPFB_TYPE_NACK", "1"),
    ("(gint) GST_RTCP_RTPFB_TYPE_RTCP_SR_REQ", "5"),
    ("(gint) GST_RTCP_RTPFB_TYPE_TMMBN", "4"),
    ("(gint) GST_RTCP_RTPFB_TYPE_TMMBR", "3"),
    ("(gint) GST_RTCP_RTPFB_TYPE_TWCC", "15"),
    ("(gint) GST_RTCP_SDES_CNAME", "1"),
    ("(gint) GST_RTCP_SDES_EMAIL", "3"),
    ("(gint) GST_RTCP_SDES_END", "0"),
    ("(gint) GST_RTCP_SDES_INVALID", "-1"),
    ("(gint) GST_RTCP_SDES_LOC", "5"),
    ("(gint) GST_RTCP_SDES_NAME", "2"),
    ("(gint) GST_RTCP_SDES_NOTE", "7"),
    ("(gint) GST_RTCP_SDES_PHONE", "4"),
    ("(gint) GST_RTCP_SDES_PRIV", "8"),
    ("(gint) GST_RTCP_SDES_TOOL", "6"),
    ("(gint) GST_RTCP_TYPE_APP", "204"),
    ("(gint) GST_RTCP_TYPE_BYE", "203"),
    ("(gint) GST_RTCP_TYPE_INVALID", "0"),
    ("(gint) GST_RTCP_TYPE_PSFB", "206"),
    ("(gint) GST_RTCP_TYPE_RR", "201"),
    ("(gint) GST_RTCP_TYPE_RTPFB", "205"),
    ("(gint) GST_RTCP_TYPE_SDES", "202"),
    ("(gint) GST_RTCP_TYPE_SR", "200"),
    ("(gint) GST_RTCP_TYPE_XR", "207"),
    ("GST_RTCP_VALID_MASK", "57598"),
    ("GST_RTCP_VALID_VALUE", "200"),
    ("GST_RTCP_VERSION", "2"),
    ("(gint) GST_RTCP_XR_TYPE_DLRR", "5"),
    ("(gint) GST_RTCP_XR_TYPE_DRLE", "2"),
    ("(gint) GST_RTCP_XR_TYPE_INVALID", "-1"),
    ("(gint) GST_RTCP_XR_TYPE_LRLE", "1"),
    ("(gint) GST_RTCP_XR_TYPE_PRT", "3"),
    ("(gint) GST_RTCP_XR_TYPE_RRT", "4"),
    ("(gint) GST_RTCP_XR_TYPE_SSUMM", "6"),
    ("(gint) GST_RTCP_XR_TYPE_VOIP_METRICS", "7"),
    ("(guint) GST_RTP_BUFFER_FLAG_LAST", "268435456"),
    ("(guint) GST_RTP_BUFFER_FLAG_REDUNDANT", "2097152"),
    ("(guint) GST_RTP_BUFFER_FLAG_RETRANSMISSION", "1048576"),
    ("(guint) GST_RTP_BUFFER_MAP_FLAG_LAST", "16777216"),
    ("(guint) GST_RTP_BUFFER_MAP_FLAG_SKIP_PADDING", "65536"),
    ("GST_RTP_HDREXT_BASE", "urn:ietf:params:rtp-hdrext:"),
    ("GST_RTP_HDREXT_NTP_56", "ntp-56"),
    ("GST_RTP_HDREXT_NTP_56_SIZE", "7"),
    ("GST_RTP_HDREXT_NTP_64", "ntp-64"),
    ("GST_RTP_HDREXT_NTP_64_SIZE", "8"),
    ("(gint) GST_RTP_PAYLOAD_1016", "1"),
    ("GST_RTP_PAYLOAD_1016_STRING", "1"),
    ("(gint) GST_RTP_PAYLOAD_CELLB", "25"),
    ("GST_RTP_PAYLOAD_CELLB_STRING", "25"),
    ("(gint) GST_RTP_PAYLOAD_CN", "13"),
    ("GST_RTP_PAYLOAD_CN_STRING", "13"),
    ("(gint) GST_RTP_PAYLOAD_DVI4_11025", "16"),
    ("GST_RTP_PAYLOAD_DVI4_11025_STRING", "16"),
    ("(gint) GST_RTP_PAYLOAD_DVI4_16000", "6"),
    ("GST_RTP_PAYLOAD_DVI4_16000_STRING", "6"),
    ("(gint) GST_RTP_PAYLOAD_DVI4_22050", "17"),
    ("GST_RTP_PAYLOAD_DVI4_22050_STRING", "17"),
    ("(gint) GST_RTP_PAYLOAD_DVI4_8000", "5"),
    ("GST_RTP_PAYLOAD_DVI4_8000_STRING", "5"),
    ("GST_RTP_PAYLOAD_DYNAMIC_STRING", "[96, 127]"),
    ("(gint) GST_RTP_PAYLOAD_G721", "2"),
    ("GST_RTP_PAYLOAD_G721_STRING", "2"),
    ("(gint) GST_RTP_PAYLOAD_G722", "9"),
    ("GST_RTP_PAYLOAD_G722_STRING", "9"),
    ("(gint) GST_RTP_PAYLOAD_G723", "4"),
    ("GST_RTP_PAYLOAD_G723_53", "17"),
    ("GST_RTP_PAYLOAD_G723_53_STRING", "17"),
    ("GST_RTP_PAYLOAD_G723_63", "16"),
    ("GST_RTP_PAYLOAD_G723_63_STRING", "16"),
    ("GST_RTP_PAYLOAD_G723_STRING", "4"),
    ("(gint) GST_RTP_PAYLOAD_G728", "15"),
    ("GST_RTP_PAYLOAD_G728_STRING", "15"),
    ("(gint) GST_RTP_PAYLOAD_G729", "18"),
    ("GST_RTP_PAYLOAD_G729_STRING", "18"),
    ("(gint) GST_RTP_PAYLOAD_GSM", "3"),
    ("GST_RTP_PAYLOAD_GSM_STRING", "3"),
    ("(gint) GST_RTP_PAYLOAD_H261", "31"),
    ("GST_RTP_PAYLOAD_H261_STRING", "31"),
    ("(gint) GST_RTP_PAYLOAD_H263", "34"),
    ("GST_RTP_PAYLOAD_H263_STRING", "34"),
    ("(gint) GST_RTP_PAYLOAD_JPEG", "26"),
    ("GST_RTP_PAYLOAD_JPEG_STRING", "26"),
    ("(gint) GST_RTP_PAYLOAD_L16_MONO", "11"),
    ("GST_RTP_PAYLOAD_L16_MONO_STRING", "11"),
    ("(gint) GST_RTP_PAYLOAD_L16_STEREO", "10"),
    ("GST_RTP_PAYLOAD_L16_STEREO_STRING", "10"),
    ("(gint) GST_RTP_PAYLOAD_LPC", "7"),
    ("GST_RTP_PAYLOAD_LPC_STRING", "7"),
    ("(gint) GST_RTP_PAYLOAD_MP2T", "33"),
    ("GST_RTP_PAYLOAD_MP2T_STRING", "33"),
    ("(gint) GST_RTP_PAYLOAD_MPA", "14"),
    ("GST_RTP_PAYLOAD_MPA_STRING", "14"),
    ("(gint) GST_RTP_PAYLOAD_MPV", "32"),
    ("GST_RTP_PAYLOAD_MPV_STRING", "32"),
    ("(gint) GST_RTP_PAYLOAD_NV", "28"),
    ("GST_RTP_PAYLOAD_NV_STRING", "28"),
    ("(gint) GST_RTP_PAYLOAD_PCMA", "8"),
    ("GST_RTP_PAYLOAD_PCMA_STRING", "8"),
    ("(gint) GST_RTP_PAYLOAD_PCMU", "0"),
    ("GST_RTP_PAYLOAD_PCMU_STRING", "0"),
    ("(gint) GST_RTP_PAYLOAD_QCELP", "12"),
    ("GST_RTP_PAYLOAD_QCELP_STRING", "12"),
    ("GST_RTP_PAYLOAD_TS41", "19"),
    ("GST_RTP_PAYLOAD_TS41_STRING", "19"),
    ("GST_RTP_PAYLOAD_TS48", "18"),
    ("GST_RTP_PAYLOAD_TS48_STRING", "18"),
    ("(gint) GST_RTP_PROFILE_AVP", "1"),
    ("(gint) GST_RTP_PROFILE_AVPF", "3"),
    ("(gint) GST_RTP_PROFILE_SAVP", "2"),
    ("(gint) GST_RTP_PROFILE_SAVPF", "4"),
    ("(gint) GST_RTP_PROFILE_UNKNOWN", "0"),
    ("GST_RTP_SOURCE_META_MAX_CSRC_COUNT", "15"),
    ("GST_RTP_VERSION", "2"),
];