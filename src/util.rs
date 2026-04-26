use crate::error::AppError;
use chrono::{SecondsFormat, Utc};

pub fn round_two(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub fn timestamp_utc() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

pub fn parse_u64(value: &str, tool: &'static str, label: &str) -> Result<u64, AppError> {
    value
        .parse::<u64>()
        .map_err(|error| AppError::internal(tool, format!("failed to parse {label}: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_two_truncates_to_two_decimals() {
        assert_eq!(round_two(1.234_567), 1.23);
        assert_eq!(round_two(1.235), 1.24);
        assert_eq!(round_two(0.0), 0.0);
        assert_eq!(round_two(-1.236), -1.24);
    }

    #[test]
    fn parse_u64_succeeds_for_valid_input() {
        let value = parse_u64("42", "demo", "field").unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn parse_u64_returns_internal_error_for_invalid_input() {
        let error = parse_u64("not-a-number", "demo", "field").unwrap_err();
        assert_eq!(error.code, "internal_error");
        assert!(error.message.contains("field"));
    }

    #[test]
    fn timestamp_utc_has_zulu_suffix() {
        let stamp = timestamp_utc();
        assert!(stamp.ends_with('Z'), "expected zulu suffix, got {stamp}");
    }
}
