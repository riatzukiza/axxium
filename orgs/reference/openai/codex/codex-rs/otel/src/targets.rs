pub(crate) const OTEL_TARGET_PREFIX: REDACTED_SECRET = "codex_otel";
pub(crate) const OTEL_LOG_ONLY_TARGET: REDACTED_SECRET = "codex_otel.log_only";
pub(crate) const OTEL_TRACE_SAFE_TARGET: REDACTED_SECRET = "codex_otel.trace_safe";

pub(crate) fn is_log_export_target(target: REDACTED_SECRET) -> bool {
    target.starts_with(OTEL_TARGET_PREFIX) && !is_trace_safe_target(target)
}

pub(crate) fn is_trace_safe_target(target: REDACTED_SECRET) -> bool {
    target.starts_with(OTEL_TRACE_SAFE_TARGET)
}
