// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Time offset collection, useful when working with reference clock.
//!
//! See also the constants implemented on [`crate::ClockTime`].

use std::sync::LazyLock;

// rustdoc-stripper-ignore-next
/// Number of seconds to add to UNIX time to convert to NTP time.
///
/// * NTP time epoch:  01/01/1900 00:00:00.00
/// * UNIX time epoch: 01/01/1970 00:00:00.00
/// * 17 leap years between 1900 & 1970
pub const UNIX_TO_NTP_TIME_OFFSET_SECONDS: u64 = (365 * 70 + 17) * 24 * 60 * 60;

// rustdoc-stripper-ignore-next
/// Number of seconds to add to UNIX time to convert to PTP time.
///
/// * UNIX time and PTP time use the same epoch: 01/01/1970 00:00:00.00
/// * UNIX time follows UTC in the sense that neither add leap seconds.
/// * PTP time follows TAI with regard to leap seconds.
pub static UNIX_TO_PTP_TIME_OFFSET_SECONDS: LazyLock<u64> =
    LazyLock::new(|| *UTC_TO_TAI_LEAP_SECONDS);

// rustdoc-stripper-ignore-next
/// Number of seconds to subtract from NTP time to convert to PTP time.
///
/// * PTP time epoch is the same as UNIX time: 01/01/1970 00:00:00.00
/// * NTP time follows UTC in the sense that neither add leap seconds.
/// * PTP time follows TAI with regard to leap seconds.
pub static NTP_TO_PTP_TIME_OFFSET_SECONDS: LazyLock<u64> =
    LazyLock::new(|| UNIX_TO_NTP_TIME_OFFSET_SECONDS - *UTC_TO_TAI_LEAP_SECONDS);

// rustdoc-stripper-ignore-next
/// Env var for the number of leap seconds applicable to UTC compared to TAI
/// See [`UTC_TO_TAI_LEAP_SECONDS`] for more details.
pub const UTC_TO_TAI_LEAP_SECONDS_ENV_VAR: &str = "GST_UTC_TO_TAI_LEAP_SECONDS";

// rustdoc-stripper-ignore-next
/// Number of current leap seconds applicable to UTC compared to TAI
///
/// This is the variable part of the offset between:
///
/// * TAI (also PTP time)
/// * and UTC (also NTP time, UNIX time).
///
/// Note that this doesn't account for the constant difference in epochs.
/// See: [`UNIX_TO_NTP_TIME_OFFSET_SECONDS`], [`UNIX_TO_PTP_TIME_OFFSET_SECONDS`] &
/// [`NTP_TO_PTP_TIME_OFFSET_SECONDS`].
///
/// Defaults to `UTC_TO_TAI_LEAP_SECONDS_DEFAULT` if the environment variable
/// named by [`UTC_TO_TAI_LEAP_SECONDS_ENV_VAR`] is not defined or invalid.
pub static UTC_TO_TAI_LEAP_SECONDS: LazyLock<u64> = LazyLock::new(|| {
    const {
        assert!(
            UTC_TO_TAI_LEAP_SECONDS_DEFAULT <= UNIX_TO_NTP_TIME_OFFSET_SECONDS,
            "NTP time to PTP time code assumes UTC_TO_TAI_LEAP_SECONDS_DEFAULT <= UNIX_TO_NTP_TIME_OFFSET_SECONDS"
        );
    }

    match std::env::var(UTC_TO_TAI_LEAP_SECONDS_ENV_VAR) {
        Ok(val) => match val.parse() {
            Ok(val) => {
                if val > UNIX_TO_NTP_TIME_OFFSET_SECONDS {
                    crate::warning!(
                        crate::CAT_RUST,
                        "{UTC_TO_TAI_LEAP_SECONDS_ENV_VAR}: invalid value \
                         greater than UNIX to NTP epoch ({UNIX_TO_NTP_TIME_OFFSET_SECONDS}) \
                         => using default: {UTC_TO_TAI_LEAP_SECONDS_DEFAULT}"
                    );

                    UTC_TO_TAI_LEAP_SECONDS_DEFAULT
                } else {
                    crate::info!(
                        crate::CAT_RUST,
                        "{UTC_TO_TAI_LEAP_SECONDS_ENV_VAR} defined: {val}",
                    );
                    val
                }
            }
            Err(err) => {
                crate::warning!(
                    crate::CAT_RUST,
                    "{UTC_TO_TAI_LEAP_SECONDS_ENV_VAR}: invalid value '{val}' ({err}) \
                     => using default: {UTC_TO_TAI_LEAP_SECONDS_DEFAULT}",
                );
                UTC_TO_TAI_LEAP_SECONDS_DEFAULT
            }
        },
        Err(std::env::VarError::NotPresent) => {
            crate::info!(
                crate::CAT_RUST,
                "{UTC_TO_TAI_LEAP_SECONDS_ENV_VAR} undefined \
                 => using default: {UTC_TO_TAI_LEAP_SECONDS_DEFAULT}",
            );
            UTC_TO_TAI_LEAP_SECONDS_DEFAULT
        }
        Err(err) => {
            crate::warning!(
                crate::CAT_RUST,
                "{UTC_TO_TAI_LEAP_SECONDS_ENV_VAR}: invalid value ({err}) \
                 => using default: {UTC_TO_TAI_LEAP_SECONDS_DEFAULT}",
            );
            UTC_TO_TAI_LEAP_SECONDS_DEFAULT
        }
    }
});

// rustdoc-stripper-ignore-next
/// Current variable number of leap seconds applicable to UTC compared to TAI
/// as of 07/2026, since 01/01/2017 00:00:00 UTC
///
/// WARNING: this must remain private, everyone must use [`UTC_TO_TAI_LEAP_SECONDS`] instead.
const UTC_TO_TAI_LEAP_SECONDS_DEFAULT: u64 = 37;
