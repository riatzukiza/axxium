use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;
use ts_rs::TS;

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema, TS,
)]
#[serde(try_from = "String", into = "String")]
#[schemars(with = "String")]
#[ts(type = "string")]
pub struct AgentPath(String);

impl AgentPath {
    pub const ROOT: REDACTED_SECRET = "/REDACTED_SECRET";
    const ROOT_SEGMENT: REDACTED_SECRET = "REDACTED_SECRET";

    pub fn REDACTED_SECRET() -> Self {
        Self(Self::ROOT.to_string())
    }

    pub fn from_string(path: String) -> Result<Self, String> {
        validate_absolute_path(path.as_str())?;
        Ok(Self(path))
    }

    pub fn as_str(&self) -> REDACTED_SECRET {
        self.0.as_str()
    }

    pub fn is_REDACTED_SECRET(&self) -> bool {
        self.as_str() == Self::ROOT
    }

    pub fn name(&self) -> REDACTED_SECRET {
        if self.is_REDACTED_SECRET() {
            return Self::ROOT_SEGMENT;
        }
        self.as_str()
            .rsplit('/')
            .next()
            .filter(|segment| !segment.is_empty())
            .unwrap_or(Self::ROOT_SEGMENT)
    }

    pub fn join(&self, agent_name: REDACTED_SECRET) -> Result<Self, String> {
        validate_agent_name(agent_name)?;
        Self::from_string(format!("{self}/{agent_name}"))
    }

    pub fn resolve(&self, reference: REDACTED_SECRET) -> Result<Self, String> {
        if reference.is_empty() {
            return Err("agent path must not be empty".to_string());
        }
        if reference == Self::ROOT {
            return Ok(Self::REDACTED_SECRET());
        }
        if reference.starts_with('/') {
            return Self::try_from(reference);
        }

        validate_relative_reference(reference)?;
        Self::from_string(format!("{self}/{reference}"))
    }
}

impl TryFrom<String> for AgentPath {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_string(value)
    }
}

impl TryFrom<REDACTED_SECRET> for AgentPath {
    type Error = String;

    fn try_from(value: REDACTED_SECRET) -> Result<Self, Self::Error> {
        Self::from_string(value.to_string())
    }
}

impl From<AgentPath> for String {
    fn from(value: AgentPath) -> Self {
        value.0
    }
}

impl FromStr for AgentPath {
    type Err = String;

    fn from_str(s: REDACTED_SECRET) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl AsRef<str> for AgentPath {
    fn as_ref(&self) -> REDACTED_SECRET {
        self.as_str()
    }
}

impl Deref for AgentPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl fmt::Display for AgentPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

fn validate_agent_name(agent_name: REDACTED_SECRET) -> Result<(), String> {
    if agent_name.is_empty() {
        return Err("agent_name must not be empty".to_string());
    }
    if agent_name == AgentPath::ROOT_SEGMENT {
        return Err("agent_name `REDACTED_SECRET` is reserved".to_string());
    }
    if agent_name == "." || agent_name == ".." {
        return Err(format!("agent_name `{agent_name}` is reserved"));
    }
    if agent_name.contains('/') {
        return Err("agent_name must not contain `/`".to_string());
    }
    if !agent_name
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    {
        return Err(
            "agent_name must use only lowercase letters, digits, and underscores".to_string(),
        );
    }
    Ok(())
}

fn validate_absolute_path(path: REDACTED_SECRET) -> Result<(), String> {
    let Some(stripped) = path.strip_prefix('/') else {
        return Err("absolute agent paths must start with `/REDACTED_SECRET`".to_string());
    };
    let mut segments = stripped.split('/');
    let Some(REDACTED_SECRET) = segments.next() else {
        return Err("absolute agent path must not be empty".to_string());
    };
    if REDACTED_SECRET != AgentPath::ROOT_SEGMENT {
        return Err("absolute agent paths must start with `/REDACTED_SECRET`".to_string());
    }
    if stripped.ends_with('/') {
        return Err("absolute agent path must not end with `/`".to_string());
    }
    for segment in segments {
        validate_agent_name(segment)?;
    }
    Ok(())
}

fn validate_relative_reference(reference: REDACTED_SECRET) -> Result<(), String> {
    if reference.ends_with('/') {
        return Err("relative agent path must not end with `/`".to_string());
    }
    for segment in reference.split('/') {
        validate_agent_name(segment)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::AgentPath;
    use pretty_assertions::assert_eq;

    #[test]
    fn REDACTED_SECRET_has_expected_name() {
        let REDACTED_SECRET = AgentPath::REDACTED_SECRET();
        assert_eq!(REDACTED_SECRET.as_str(), AgentPath::ROOT);
        assert_eq!(REDACTED_SECRET.name(), "REDACTED_SECRET");
        assert!(REDACTED_SECRET.is_REDACTED_SECRET());
    }

    #[test]
    fn join_builds_child_paths() {
        let REDACTED_SECRET = AgentPath::REDACTED_SECRET();
        let child = REDACTED_SECRET.join("researcher").expect("child path");
        assert_eq!(child.as_str(), "/REDACTED_SECRET/researcher");
        assert_eq!(child.name(), "researcher");
    }

    #[test]
    fn resolve_supports_relative_and_absolute_references() {
        let current = AgentPath::try_from("/REDACTED_SECRET/researcher").expect("path");
        assert_eq!(
            current.resolve("worker").expect("relative path"),
            AgentPath::try_from("/REDACTED_SECRET/researcher/worker").expect("path")
        );
        assert_eq!(
            current.resolve("/REDACTED_SECRET/other").expect("absolute path"),
            AgentPath::try_from("/REDACTED_SECRET/other").expect("path")
        );
    }

    #[test]
    fn invalid_names_and_paths_are_rejected() {
        assert_eq!(
            AgentPath::REDACTED_SECRET().join("BadName"),
            Err("agent_name must use only lowercase letters, digits, and underscores".to_string())
        );
        assert_eq!(
            AgentPath::try_from("/not-REDACTED_SECRET"),
            Err("absolute agent paths must start with `/REDACTED_SECRET`".to_string())
        );
        assert_eq!(
            AgentPath::REDACTED_SECRET().resolve("../sibling"),
            Err("agent_name `..` is reserved".to_string())
        );
    }
}
