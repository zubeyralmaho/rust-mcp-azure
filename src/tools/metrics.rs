use crate::{
    config::AppState,
    error::AppError,
    util::{parse_u64, round_two, timestamp_utc},
};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::{path::Path, time::Duration};
use tokio::{fs, process::Command, time::sleep};

const TOOL: &str = "safe_system_metrics";

#[derive(Deserialize)]
struct Input {
    sections: Vec<String>,
    #[serde(default)]
    include_top_processes: bool,
    #[serde(default = "default_top_process_limit")]
    top_process_limit: usize,
}

struct CpuTotals {
    idle: u64,
    total: u64,
}

fn default_top_process_limit() -> usize {
    5
}

pub async fn handle(state: &AppState, input: Value) -> Result<Value, AppError> {
    let input: Input = serde_json::from_value(input).map_err(|error| {
        AppError::bad_request(
            TOOL,
            "invalid_argument",
            format!("invalid safe_system_metrics input: {error}"),
        )
    })?;

    if input.sections.is_empty() {
        return Err(AppError::bad_request(
            TOOL,
            "invalid_argument",
            "sections must contain at least one value",
        ));
    }

    if !(1..=5).contains(&input.top_process_limit) {
        return Err(AppError::bad_request(
            TOOL,
            "invalid_argument",
            "top_process_limit must be between 1 and 5",
        ));
    }

    let mut data = Map::new();
    let mut warnings = Vec::new();

    for section in input.sections {
        match section.as_str() {
            "cpu" => {
                data.insert("cpu".to_string(), cpu_metrics().await?);
            }
            "memory" => {
                data.insert("memory".to_string(), memory_metrics().await?);
            }
            "disk" => {
                data.insert("disk".to_string(), disk_metrics(&state.sandbox_root).await?);
            }
            "runtime" => {
                data.insert(
                    "runtime".to_string(),
                    json!({
                        "sandbox_root": state.sandbox_root.display().to_string(),
                        "uptime_seconds": uptime_seconds().await?,
                    }),
                );
            }
            "uptime" => {
                data.insert(
                    "uptime".to_string(),
                    json!({
                        "seconds": uptime_seconds().await?,
                    }),
                );
            }
            other => {
                return Err(AppError::bad_request(
                    TOOL,
                    "invalid_argument",
                    format!(
                        "section must be one of: cpu, memory, disk, runtime, uptime; got {other}"
                    ),
                ));
            }
        }
    }

    if input.include_top_processes {
        if state.enable_process_summary && cfg!(target_os = "linux") {
            let process_summary = top_processes(input.top_process_limit).await?;
            data.insert("top_processes".to_string(), process_summary);
        } else if state.enable_process_summary {
            warnings
                .push("process summaries are currently available only on Linux in v1".to_string());
        } else {
            warnings.push("process summaries are disabled by configuration".to_string());
        }
    }

    Ok(json!({
        "ok": true,
        "tool": TOOL,
        "timestamp_utc": timestamp_utc(),
        "data": Value::Object(data),
        "warnings": warnings,
    }))
}

async fn cpu_metrics() -> Result<Value, AppError> {
    let logical_cores = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    let load_average_1m = load_average_1m().await?;
    let usage_percent = cpu_usage_percent().await?;

    Ok(json!({
        "logical_cores": logical_cores,
        "usage_percent": usage_percent,
        "load_average_1m": load_average_1m,
    }))
}

async fn cpu_usage_percent() -> Result<f64, AppError> {
    if cfg!(target_os = "macos") {
        return macos_cpu_usage_percent().await;
    }

    let first = read_cpu_totals().await?;
    sleep(Duration::from_millis(120)).await;
    let second = read_cpu_totals().await?;

    let idle_delta = second.idle.saturating_sub(first.idle);
    let total_delta = second.total.saturating_sub(first.total);

    if total_delta == 0 {
        return Ok(0.0);
    }

    Ok(round_two(
        (1.0 - idle_delta as f64 / total_delta as f64) * 100.0,
    ))
}

async fn read_cpu_totals() -> Result<CpuTotals, AppError> {
    let contents = read_text_file("/proc/stat").await?;
    let cpu_line = contents
        .lines()
        .find(|line| line.starts_with("cpu "))
        .ok_or_else(|| AppError::internal(TOOL, "missing cpu line in /proc/stat"))?;

    let values = cpu_line
        .split_whitespace()
        .skip(1)
        .map(|value| {
            value.parse::<u64>().map_err(|error| {
                AppError::internal(TOOL, format!("failed to parse /proc/stat value: {error}"))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    if values.len() < 5 {
        return Err(AppError::internal(TOOL, "unexpected /proc/stat cpu layout"));
    }

    let idle = values[3] + values.get(4).copied().unwrap_or(0);
    let total = values.iter().sum();

    Ok(CpuTotals { idle, total })
}

async fn memory_metrics() -> Result<Value, AppError> {
    if cfg!(target_os = "macos") {
        return macos_memory_metrics().await;
    }

    let contents = read_text_file("/proc/meminfo").await?;
    let mut total_kb = None;
    let mut available_kb = None;

    for line in contents.lines() {
        if let Some(value) = parse_meminfo_value(line, "MemTotal:")? {
            total_kb = Some(value);
        }
        if let Some(value) = parse_meminfo_value(line, "MemAvailable:")? {
            available_kb = Some(value);
        }
    }

    let total_kb =
        total_kb.ok_or_else(|| AppError::internal(TOOL, "MemTotal not found in /proc/meminfo"))?;
    let available_kb = available_kb
        .ok_or_else(|| AppError::internal(TOOL, "MemAvailable not found in /proc/meminfo"))?;
    let used_kb = total_kb.saturating_sub(available_kb);

    Ok(json!({
        "total_mb": kilobytes_to_megabytes(total_kb),
        "used_mb": kilobytes_to_megabytes(used_kb),
        "available_mb": kilobytes_to_megabytes(available_kb),
        "usage_percent": round_two((used_kb as f64 / total_kb as f64) * 100.0),
    }))
}

fn parse_meminfo_value(line: &str, prefix: &str) -> Result<Option<u64>, AppError> {
    if !line.starts_with(prefix) {
        return Ok(None);
    }

    let value = line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| AppError::internal(TOOL, "unexpected /proc/meminfo layout"))?
        .parse::<u64>()
        .map_err(|error| {
            AppError::internal(TOOL, format!("failed to parse /proc/meminfo: {error}"))
        })?;

    Ok(Some(value))
}

async fn disk_metrics(sandbox_root: &Path) -> Result<Value, AppError> {
    let sandbox_root = sandbox_root.display().to_string();
    let output = Command::new("df")
        .args(["-kP", sandbox_root.as_str()])
        .output()
        .await
        .map_err(|error| AppError::internal(TOOL, format!("failed to execute df: {error}")))?;

    if !output.status.success() {
        return Err(AppError::internal(
            TOOL,
            format!("df exited with status {}", output.status),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout
        .lines()
        .nth(1)
        .ok_or_else(|| AppError::internal(TOOL, "unexpected df output layout"))?;
    let fields = line.split_whitespace().collect::<Vec<_>>();

    if fields.len() < 6 {
        return Err(AppError::internal(TOOL, "unexpected df output field count"));
    }

    let total_kb = parse_u64(fields[1], TOOL, "df total blocks")?;
    let used_kb = parse_u64(fields[2], TOOL, "df used blocks")?;
    let free_kb = parse_u64(fields[3], TOOL, "df free blocks")?;
    let usage_percent = fields[4]
        .trim_end_matches('%')
        .parse::<f64>()
        .map_err(|error| {
            AppError::internal(TOOL, format!("failed to parse df usage percent: {error}"))
        })?;

    Ok(json!({
        "mount_path": fields[5],
        "total_gb": kilobytes_to_gigabytes(total_kb),
        "used_gb": kilobytes_to_gigabytes(used_kb),
        "free_gb": kilobytes_to_gigabytes(free_kb),
        "usage_percent": round_two(usage_percent),
    }))
}

async fn uptime_seconds() -> Result<f64, AppError> {
    if cfg!(target_os = "macos") {
        return macos_uptime_seconds().await;
    }

    let contents = read_text_file("/proc/uptime").await?;
    let value = contents
        .split_whitespace()
        .next()
        .ok_or_else(|| AppError::internal(TOOL, "unexpected /proc/uptime layout"))?
        .parse::<f64>()
        .map_err(|error| {
            AppError::internal(TOOL, format!("failed to parse /proc/uptime: {error}"))
        })?;

    Ok(round_two(value))
}

async fn load_average_1m() -> Result<f64, AppError> {
    if cfg!(target_os = "macos") {
        return macos_load_average_1m().await;
    }

    let contents = read_text_file("/proc/loadavg").await?;
    let value = contents
        .split_whitespace()
        .next()
        .ok_or_else(|| AppError::internal(TOOL, "unexpected /proc/loadavg layout"))?
        .parse::<f64>()
        .map_err(|error| {
            AppError::internal(TOOL, format!("failed to parse /proc/loadavg: {error}"))
        })?;

    Ok(round_two(value))
}

async fn macos_cpu_usage_percent() -> Result<f64, AppError> {
    let output = run_read_only_command("top", &["-l", "1", "-n", "0"]).await?;

    let cpu_line = output
        .lines()
        .find(|line| line.contains("CPU usage:"))
        .ok_or_else(|| AppError::internal(TOOL, "missing CPU usage line in top output"))?;

    let usage = cpu_line
        .split(':')
        .nth(1)
        .ok_or_else(|| AppError::internal(TOOL, "unexpected top CPU layout"))?
        .split(',')
        .take(2)
        .map(parse_percent_fragment)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<f64>();

    Ok(round_two(usage))
}

async fn macos_memory_metrics() -> Result<Value, AppError> {
    let memsize_output = run_read_only_command("sysctl", &["-n", "hw.memsize"]).await?;
    let total_bytes = memsize_output.trim().parse::<u64>().map_err(|error| {
        AppError::internal(TOOL, format!("failed to parse hw.memsize: {error}"))
    })?;

    let vm_stat_output = run_read_only_command("vm_stat", &[]).await?;
    let page_size = parse_macos_page_size(&vm_stat_output)?;
    let free_pages = parse_macos_vm_stat(&vm_stat_output, "Pages free")?;
    let inactive_pages = parse_macos_vm_stat(&vm_stat_output, "Pages inactive")?;
    let speculative_pages = parse_macos_vm_stat(&vm_stat_output, "Pages speculative")?;

    let available_bytes = (free_pages + inactive_pages + speculative_pages) * page_size;
    let used_bytes = total_bytes.saturating_sub(available_bytes);

    Ok(json!({
        "total_mb": bytes_to_megabytes(total_bytes),
        "used_mb": bytes_to_megabytes(used_bytes),
        "available_mb": bytes_to_megabytes(available_bytes),
        "usage_percent": round_two((used_bytes as f64 / total_bytes as f64) * 100.0),
    }))
}

async fn macos_uptime_seconds() -> Result<f64, AppError> {
    let output = run_read_only_command("sysctl", &["-n", "kern.boottime"]).await?;
    let boot_seconds = output
        .split("sec = ")
        .nth(1)
        .and_then(|value| value.split(',').next())
        .ok_or_else(|| AppError::internal(TOOL, "unexpected kern.boottime layout"))?
        .trim()
        .parse::<u64>()
        .map_err(|error| {
            AppError::internal(TOOL, format!("failed to parse kern.boottime: {error}"))
        })?;

    let now = Utc::now().timestamp();
    let uptime = now.saturating_sub(boot_seconds as i64) as f64;

    Ok(round_two(uptime))
}

async fn macos_load_average_1m() -> Result<f64, AppError> {
    let output = run_read_only_command("sysctl", &["-n", "vm.loadavg"]).await?;
    let value = output
        .trim()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .split_whitespace()
        .next()
        .ok_or_else(|| AppError::internal(TOOL, "unexpected vm.loadavg layout"))?
        .parse::<f64>()
        .map_err(|error| {
            AppError::internal(TOOL, format!("failed to parse vm.loadavg: {error}"))
        })?;

    Ok(round_two(value))
}

async fn top_processes(limit: usize) -> Result<Value, AppError> {
    let output = Command::new("top")
        .args(["-b", "-n", "1"])
        .output()
        .await
        .map_err(|error| AppError::internal(TOOL, format!("failed to execute top: {error}")))?;

    if !output.status.success() {
        return Err(AppError::internal(
            TOOL,
            format!("top exited with status {}", output.status),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let processes = stdout
        .lines()
        .filter(|line| {
            line.trim_start()
                .chars()
                .next()
                .map(|character| character.is_ascii_digit())
                .unwrap_or(false)
        })
        .filter_map(parse_top_line)
        .take(limit)
        .collect::<Vec<_>>();

    Ok(Value::Array(processes))
}

fn parse_top_line(line: &str) -> Option<Value> {
    let fields = line.split_whitespace().collect::<Vec<_>>();

    if fields.len() < 12 {
        return None;
    }

    Some(json!({
        "pid": fields[0].parse::<u64>().ok()?,
        "cpu_percent": fields[8].parse::<f64>().ok().map(round_two)?,
        "memory_percent": fields[9].parse::<f64>().ok().map(round_two)?,
        "command": fields[11..].join(" "),
    }))
}

async fn read_text_file(path: &str) -> Result<String, AppError> {
    fs::read_to_string(path)
        .await
        .map_err(|error| AppError::internal(TOOL, format!("failed to read {path}: {error}")))
}

async fn run_read_only_command(program: &str, args: &[&str]) -> Result<String, AppError> {
    let output = Command::new(program)
        .args(args)
        .output()
        .await
        .map_err(|error| {
            AppError::internal(TOOL, format!("failed to execute {program}: {error}"))
        })?;

    if !output.status.success() {
        return Err(AppError::internal(
            TOOL,
            format!("{program} exited with status {}", output.status),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn parse_percent_fragment(fragment: &str) -> Result<f64, AppError> {
    fragment
        .trim()
        .split('%')
        .next()
        .ok_or_else(|| AppError::internal(TOOL, "unexpected percent fragment layout"))?
        .trim()
        .parse::<f64>()
        .map_err(|error| {
            AppError::internal(TOOL, format!("failed to parse percent fragment: {error}"))
        })
}

fn parse_macos_page_size(output: &str) -> Result<u64, AppError> {
    let header = output
        .lines()
        .next()
        .ok_or_else(|| AppError::internal(TOOL, "vm_stat output was empty"))?;

    let page_size = header
        .split("page size of ")
        .nth(1)
        .and_then(|value| value.split(" bytes").next())
        .ok_or_else(|| AppError::internal(TOOL, "unexpected vm_stat header layout"))?
        .trim()
        .parse::<u64>()
        .map_err(|error| {
            AppError::internal(TOOL, format!("failed to parse vm_stat page size: {error}"))
        })?;

    Ok(page_size)
}

fn parse_macos_vm_stat(output: &str, key: &str) -> Result<u64, AppError> {
    let line = output
        .lines()
        .find(|line| line.starts_with(key))
        .ok_or_else(|| AppError::internal(TOOL, format!("missing {key} in vm_stat")))?;

    let value = line
        .split(':')
        .nth(1)
        .ok_or_else(|| AppError::internal(TOOL, format!("unexpected {key} layout")))?
        .trim()
        .trim_end_matches('.')
        .replace('.', "")
        .parse::<u64>()
        .map_err(|error| {
            AppError::internal(TOOL, format!("failed to parse {key} from vm_stat: {error}"))
        })?;

    Ok(value)
}

fn bytes_to_megabytes(value: u64) -> u64 {
    value / 1024 / 1024
}

fn kilobytes_to_megabytes(value: u64) -> u64 {
    value / 1024
}

fn kilobytes_to_gigabytes(value: u64) -> f64 {
    round_two(value as f64 / 1024.0 / 1024.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppState;
    use serde_json::json;
    use std::path::PathBuf;

    fn test_state() -> AppState {
        AppState {
            api_key: "test".to_string(),
            sandbox_root: PathBuf::from("."),
            enable_process_summary: false,
        }
    }

    #[tokio::test]
    async fn handle_rejects_empty_sections() {
        let state = test_state();
        let error = handle(&state, json!({ "sections": [] })).await.unwrap_err();
        assert_eq!(error.code, "invalid_argument");
        assert!(error.message.contains("at least one"));
    }

    #[tokio::test]
    async fn handle_rejects_missing_sections_field() {
        let state = test_state();
        let error = handle(&state, json!({})).await.unwrap_err();
        assert_eq!(error.code, "invalid_argument");
    }

    #[tokio::test]
    async fn handle_rejects_unknown_section() {
        let state = test_state();
        let error = handle(&state, json!({ "sections": ["network"] }))
            .await
            .unwrap_err();
        assert_eq!(error.code, "invalid_argument");
        assert!(error.message.contains("network"));
    }

    #[tokio::test]
    async fn handle_rejects_top_process_limit_above_five() {
        let state = test_state();
        let error = handle(
            &state,
            json!({
                "sections": ["cpu"],
                "include_top_processes": true,
                "top_process_limit": 99
            }),
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "invalid_argument");
        assert!(error.message.contains("between 1 and 5"));
    }

    #[tokio::test]
    async fn handle_rejects_top_process_limit_zero() {
        let state = test_state();
        let error = handle(
            &state,
            json!({
                "sections": ["cpu"],
                "include_top_processes": true,
                "top_process_limit": 0
            }),
        )
        .await
        .unwrap_err();
        assert_eq!(error.code, "invalid_argument");
    }

    #[test]
    fn parse_meminfo_value_extracts_kilobytes() {
        let value = parse_meminfo_value("MemTotal:       16384000 kB", "MemTotal:")
            .unwrap()
            .unwrap();
        assert_eq!(value, 16_384_000);
    }

    #[test]
    fn parse_meminfo_value_returns_none_for_other_prefix() {
        let value = parse_meminfo_value("Buffers:        12345 kB", "MemTotal:").unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn parse_macos_page_size_reads_header() {
        let header = "Mach Virtual Memory Statistics: (page size of 16384 bytes)";
        assert_eq!(parse_macos_page_size(header).unwrap(), 16384);
    }

    #[test]
    fn parse_macos_page_size_fails_when_missing() {
        let header = "Mach Virtual Memory Statistics:\n";
        assert!(parse_macos_page_size(header).is_err());
    }

    #[test]
    fn parse_macos_vm_stat_strips_dots() {
        let output = "Mach Virtual Memory Statistics: (page size of 16384 bytes)\nPages free:                               12345.\nPages active:                             67890.\n";
        assert_eq!(parse_macos_vm_stat(output, "Pages free").unwrap(), 12345);
        assert_eq!(parse_macos_vm_stat(output, "Pages active").unwrap(), 67890);
    }

    #[test]
    fn parse_macos_vm_stat_fails_for_missing_key() {
        let output = "Pages free: 1.\n";
        assert!(parse_macos_vm_stat(output, "Pages active").is_err());
    }

    #[test]
    fn parse_percent_fragment_handles_trailing_percent() {
        assert_eq!(parse_percent_fragment("12.34% user").unwrap(), 12.34);
        assert_eq!(parse_percent_fragment(" 5.5% sys").unwrap(), 5.5);
    }

    #[test]
    fn parse_top_line_extracts_summary_fields() {
        let line = "1234 user      20   0  100m  10m   5m S  12.3  4.5   0:01.23 my_proc arg1";
        let value = parse_top_line(line).unwrap();
        assert_eq!(value["pid"], 1234);
        assert_eq!(value["cpu_percent"], 12.3);
        assert_eq!(value["memory_percent"], 4.5);
        assert_eq!(value["command"], "my_proc arg1");
    }

    #[test]
    fn parse_top_line_returns_none_for_short_line() {
        assert!(parse_top_line("1234 only-three fields").is_none());
    }

    #[test]
    fn unit_conversions_are_consistent() {
        assert_eq!(bytes_to_megabytes(1024 * 1024), 1);
        assert_eq!(bytes_to_megabytes(0), 0);
        assert_eq!(kilobytes_to_megabytes(1024), 1);
        assert_eq!(kilobytes_to_gigabytes(1024 * 1024), 1.0);
    }

    #[test]
    fn default_top_process_limit_is_five() {
        assert_eq!(default_top_process_limit(), 5);
    }
}
