//! 共通ヘルパーマクロ集
#![allow(unused_macros)]

// `once_cell` と `regex` をマクロ内で絶対パス指定して使用するため、ここでは use しない。

/// stderr に含まれる "@@HEURS_*" マーカーを抽出するマクロ。
/// 戻り値は `(score: i64, time_ms: u32)` のタプル。
///
/// 例:
/// ```
/// use heurs_core::extract_heurs_markers;
/// let stderr_str = "@@HEURS_SCORE=1\n@@HEURS_TIME_MS=2";
/// let (score, time_ms) = extract_heurs_markers!(stderr_str);
/// assert_eq!(score, 1);
/// assert_eq!(time_ms, 2);
/// ```
#[macro_export]
macro_rules! extract_heurs_markers {
    ($s:expr) => {{
        static RE: ::once_cell::sync::Lazy<::regex::Regex> =
            ::once_cell::sync::Lazy::new(|| ::regex::Regex::new(r"^@@HEURS_(\w+)=(\d+)$").unwrap());
        let mut score: i64 = 0;
        let mut time_ms: u32 = 0;
        for line in $s.lines() {
            if let Some(cap) = RE.captures(line.trim()) {
                match &cap[1] {
                    "SCORE" => score = cap[2].parse::<i64>().unwrap_or(0),
                    "TIME_MS" => time_ms = cap[2].parse::<u32>().unwrap_or(0),
                    _ => {}
                }
            }
        }
        (score, time_ms)
    }};
}
