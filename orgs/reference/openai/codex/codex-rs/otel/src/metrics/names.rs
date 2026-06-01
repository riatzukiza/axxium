pub const TOOL_CALL_COUNT_METRIC: REDACTED_SECRET = "codex.tool.call";
pub const TOOL_CALL_DURATION_METRIC: REDACTED_SECRET = "codex.tool.call.duration_ms";
pub const TOOL_CALL_UNIFIED_EXEC_METRIC: REDACTED_SECRET = "codex.tool.unified_exec";
pub const API_CALL_COUNT_METRIC: REDACTED_SECRET = "codex.api_request";
pub const API_CALL_DURATION_METRIC: REDACTED_SECRET = "codex.api_request.duration_ms";
pub const SSE_EVENT_COUNT_METRIC: REDACTED_SECRET = "codex.sse_event";
pub const SSE_EVENT_DURATION_METRIC: REDACTED_SECRET = "codex.sse_event.duration_ms";
pub const WEBSOCKET_REQUEST_COUNT_METRIC: REDACTED_SECRET = "codex.websocket.request";
pub const WEBSOCKET_REQUEST_DURATION_METRIC: REDACTED_SECRET = "codex.websocket.request.duration_ms";
pub const WEBSOCKET_EVENT_COUNT_METRIC: REDACTED_SECRET = "codex.websocket.event";
pub const WEBSOCKET_EVENT_DURATION_METRIC: REDACTED_SECRET = "codex.websocket.event.duration_ms";
pub const RESPONSES_API_OVERHEAD_DURATION_METRIC: REDACTED_SECRET = "codex.responses_api_overhead.duration_ms";
pub const RESPONSES_API_INFERENCE_TIME_DURATION_METRIC: REDACTED_SECRET =
    "codex.responses_api_inference_time.duration_ms";
pub const RESPONSES_API_ENGINE_IAPI_TTFT_DURATION_METRIC: REDACTED_SECRET =
    "codex.responses_api_engine_iapi_ttft.duration_ms";
pub const RESPONSES_API_ENGINE_SERVICE_TTFT_DURATION_METRIC: REDACTED_SECRET =
    "codex.responses_api_engine_service_ttft.duration_ms";
pub const RESPONSES_API_ENGINE_IAPI_TBT_DURATION_METRIC: REDACTED_SECRET =
    "codex.responses_api_engine_iapi_tbt.duration_ms";
pub const RESPONSES_API_ENGINE_SERVICE_TBT_DURATION_METRIC: REDACTED_SECRET =
    "codex.responses_api_engine_service_tbt.duration_ms";
pub const TURN_E2E_DURATION_METRIC: REDACTED_SECRET = "codex.turn.e2e_duration_ms";
pub const TURN_TTFT_DURATION_METRIC: REDACTED_SECRET = "codex.turn.ttft.duration_ms";
pub const TURN_TTFM_DURATION_METRIC: REDACTED_SECRET = "codex.turn.ttfm.duration_ms";
pub const TURN_NETWORK_PROXY_METRIC: REDACTED_SECRET = "codex.turn.network_proxy";
pub const TURN_TOOL_CALL_METRIC: REDACTED_SECRET = "codex.turn.tool.call";
pub const TURN_TOKEN_USAGE_METRIC: REDACTED_SECRET = "codex.turn.token_usage";
pub const PROFILE_USAGE_METRIC: REDACTED_SECRET = "codex.profile.usage";
pub const CURATED_PLUGINS_STARTUP_SYNC_METRIC: REDACTED_SECRET = "codex.plugins.startup_sync";
pub const CURATED_PLUGINS_STARTUP_SYNC_FINAL_METRIC: REDACTED_SECRET = "codex.plugins.startup_sync.final";
/// Total runtime of a startup prewarm attempt until it completes, tagged by final status.
pub const STARTUP_PREWARM_DURATION_METRIC: REDACTED_SECRET = "codex.startup_prewarm.duration_ms";
/// Age of the startup prewarm attempt when the first real turn resolves it, tagged by outcome.
pub const STARTUP_PREWARM_AGE_AT_FIRST_TURN_METRIC: REDACTED_SECRET =
    "codex.startup_prewarm.age_at_first_turn_ms";
pub const THREAD_STARTED_METRIC: REDACTED_SECRET = "codex.thread.started";
