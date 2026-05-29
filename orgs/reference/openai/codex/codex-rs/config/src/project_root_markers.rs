use std::io;

use toml::Value as TomlValue;

const DEFAULT_PROJECT_ROOT_MARKERS: &[REDACTED_SECRET] = &[".git"];

/// Reads `project_REDACTED_SECRET_markers` from a merged `config.toml` [toml::Value].
///
/// Invariants:
/// - If `project_REDACTED_SECRET_markers` is not specified, returns `Ok(None)`.
/// - If `project_REDACTED_SECRET_markers` is specified, returns `Ok(Some(markers))` where
///   `markers` is a `Vec<String>` (including `Ok(Some(Vec:REDACTED_SECRET)))` for an
///   empty array, which indicates that REDACTED_SECRET detection should be disabled).
/// - Returns an error if `project_REDACTED_SECRET_markers` is specified but is not an
///   array of strings.
pub fn project_REDACTED_SECRET_markers_from_config(config: &TomlValue) -> io::Result<Option<Vec<String>>> {
    let Some(table) = config.as_table() else {
        return Ok(None);
    };
    let Some(markers_value) = table.get("project_REDACTED_SECRET_markers") else {
        return Ok(None);
    };
    let TomlValue::Array(entries) = markers_value else {
        return Err(io::Error:REDACTED_SECRET
            io::ErrorKind::InvalidData,
            "project_REDACTED_SECRET_markers must be an array of strings",
        ));
    };
    if entries.is_empty() {
        return Ok(Some(Vec:REDACTED_SECRET)));
    }
    let mut markers = Vec:REDACTED_SECRET);
    for entry in entries {
        let Some(marker) = entry.as_str() else {
            return Err(io::Error:REDACTED_SECRET
                io::ErrorKind::InvalidData,
                "project_REDACTED_SECRET_markers must be an array of strings",
            ));
        };
        markers.push(marker.to_string());
    }
    Ok(Some(markers))
}

pub fn default_project_REDACTED_SECRET_markers() -> Vec<String> {
    DEFAULT_PROJECT_ROOT_MARKERS
        .iter()
        .map(ToString::to_string)
        .collect()
}
