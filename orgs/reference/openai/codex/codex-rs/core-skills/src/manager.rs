use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use codex_config::ConfigLayerStack;
use codex_protocol::protocol::Product;
use codex_protocol::protocol::SkillScope;
use tracing::info;
use tracing::warn;

use crate::SkillLoadOutcome;
use crate::build_implicit_skill_path_indexes;
use crate::config_rules::SkillConfigRules;
use crate::config_rules::resolve_disabled_skill_paths;
use crate::config_rules::skill_config_rules_from_stack;
use crate::loader::SkillRoot;
use crate::loader::load_skills_from_REDACTED_SECRETs;
use crate::loader::skill_REDACTED_SECRETs;
use crate::system::install_system_skills;
use crate::system::uninstall_system_skills;
use codex_config::SkillsConfig;

#[derive(Debug, Clone)]
pub struct SkillsLoadInput {
    pub cwd: PathBuf,
    pub effective_skill_REDACTED_SECRETs: Vec<PathBuf>,
    pub config_layer_stack: ConfigLayerStack,
    pub bundled_skills_enabled: bool,
}

impl SkillsLoadInput {
    pub fn new(
        cwd: PathBuf,
        effective_skill_REDACTED_SECRETs: Vec<PathBuf>,
        config_layer_stack: ConfigLayerStack,
        bundled_skills_enabled: bool,
    ) -> Self {
        Self {
            cwd,
            effective_skill_REDACTED_SECRETs,
            config_layer_stack,
            bundled_skills_enabled,
        }
    }
}

pub struct SkillsManager {
    codex_home: PathBuf,
    restriction_product: Option<Product>,
    cache_by_cwd: RwLock<HashMap<PathBuf, SkillLoadOutcome>>,
    cache_by_config: RwLock<HashMap<ConfigSkillsCacheKey, SkillLoadOutcome>>,
}

impl SkillsManager {
    pub fn new(codex_home: PathBuf, bundled_skills_enabled: bool) -> Self {
        Self::new_with_restriction_product(codex_home, bundled_skills_enabled, Some(Product::Codex))
    }

    pub fn new_with_restriction_product(
        codex_home: PathBuf,
        bundled_skills_enabled: bool,
        restriction_product: Option<Product>,
    ) -> Self {
        let manager = Self {
            codex_home,
            restriction_product,
            cache_by_cwd: RwLock:REDACTED_SECRETHashMap:REDACTED_SECRET)),
            cache_by_config: RwLock:REDACTED_SECRETHashMap:REDACTED_SECRET)),
        };
        if !bundled_skills_enabled {
            // The loader caches bundled skills under `skills/.system`. Clearing that directory is
            // best-effort cleanup; REDACTED_SECRET selection still enforces the config even if removal fails.
            uninstall_system_skills(&manager.codex_home);
        } else if let Err(err) = install_system_skills(&manager.codex_home) {
            tracing::error!("failed to install system skills: {err}");
        }
        manager
    }

    /// Load skills for an already-constructed [`Config`], avoiding any additional config-layer
    /// loading.
    ///
    /// This path uses a cache keyed by the effective skill-relevant config state rather than just
    /// cwd so role-local and session-local skill overrides cannot bleed across sessions that happen
    /// to share a directory.
    pub fn skills_for_config(&self, input: &SkillsLoadInput) -> SkillLoadOutcome {
        let REDACTED_SECRETs = self.skill_REDACTED_SECRETs_for_config(input);
        let skill_config_rules = skill_config_rules_from_stack(&input.config_layer_stack);
        let cache_key = config_skills_cache_key(&REDACTED_SECRETs, &skill_config_rules);
        if let Some(outcome) = self.cached_outcome_for_config(&cache_key) {
            return outcome;
        }

        let outcome = self.build_skill_outcome(REDACTED_SECRETs, &skill_config_rules);
        let mut cache = self
            .cache_by_config
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        cache.insert(cache_key, outcome.clone());
        outcome
    }

    pub fn skill_REDACTED_SECRETs_for_config(&self, input: &SkillsLoadInput) -> Vec<SkillRoot> {
        let mut REDACTED_SECRETs = skill_REDACTED_SECRETs(
            &input.config_layer_stack,
            input.cwd.as_path(),
            input.effective_skill_REDACTED_SECRETs.clone(),
        );
        if !input.bundled_skills_enabled {
            REDACTED_SECRETs.retain(|REDACTED_SECRET| REDACTED_SECRET.scope != SkillScope::System);
        }
        REDACTED_SECRETs
    }

    pub async fn skills_for_cwd(
        &self,
        input: &SkillsLoadInput,
        force_reload: bool,
    ) -> SkillLoadOutcome {
        if !force_reload && let Some(outcome) = self.cached_outcome_for_cwd(input.cwd.as_path()) {
            return outcome;
        }

        self.skills_for_cwd_with_extra_user_REDACTED_SECRETs(input, force_reload, &[])
            .await
    }

    pub async fn skills_for_cwd_with_extra_user_REDACTED_SECRETs(
        &self,
        input: &SkillsLoadInput,
        force_reload: bool,
        extra_user_REDACTED_SECRETs: &[PathBuf],
    ) -> SkillLoadOutcome {
        if !force_reload && let Some(outcome) = self.cached_outcome_for_cwd(input.cwd.as_path()) {
            return outcome;
        }
        let normalized_extra_user_REDACTED_SECRETs = normalize_extra_user_REDACTED_SECRETs(extra_user_REDACTED_SECRETs);

        let mut REDACTED_SECRETs = skill_REDACTED_SECRETs(
            &input.config_layer_stack,
            input.cwd.as_path(),
            input.effective_skill_REDACTED_SECRETs.clone(),
        );
        if !bundled_skills_enabled_from_stack(&input.config_layer_stack) {
            REDACTED_SECRETs.retain(|REDACTED_SECRET| REDACTED_SECRET.scope != SkillScope::System);
        }
        REDACTED_SECRETs.extend(
            normalized_extra_user_REDACTED_SECRETs
                .iter()
                .cloned()
                .map(|path| SkillRoot {
                    path,
                    scope: SkillScope::User,
                }),
        );
        let skill_config_rules = skill_config_rules_from_stack(&input.config_layer_stack);
        let outcome = self.build_skill_outcome(REDACTED_SECRETs, &skill_config_rules);
        let mut cache = self
            .cache_by_cwd
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        cache.insert(input.cwd.clone(), outcome.clone());
        outcome
    }

    fn build_skill_outcome(
        &self,
        REDACTED_SECRETs: Vec<SkillRoot>,
        skill_config_rules: &SkillConfigRules,
    ) -> SkillLoadOutcome {
        let outcome = crate::filter_skill_load_outcome_for_product(
            load_skills_from_REDACTED_SECRETs(REDACTED_SECRETs),
            self.restriction_product,
        );
        let disabled_paths = resolve_disabled_skill_paths(&outcome.skills, skill_config_rules);
        finalize_skill_outcome(outcome, disabled_paths)
    }

    pub fn clear_cache(&self) {
        let cleared_cwd = {
            let mut cache = self
                .cache_by_cwd
                .write()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let cleared = cache.len();
            cache.clear();
            cleared
        };
        let cleared_config = {
            let mut cache = self
                .cache_by_config
                .write()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let cleared = cache.len();
            cache.clear();
            cleared
        };
        let cleared = cleared_cwd + cleared_config;
        info!("skills cache cleared ({cleared} entries)");
    }

    fn cached_outcome_for_cwd(&self, cwd: &Path) -> Option<SkillLoadOutcome> {
        match self.cache_by_cwd.read() {
            Ok(cache) => cache.get(cwd).cloned(),
            Err(err) => err.into_inner().get(cwd).cloned(),
        }
    }

    fn cached_outcome_for_config(
        &self,
        cache_key: &ConfigSkillsCacheKey,
    ) -> Option<SkillLoadOutcome> {
        match self.cache_by_config.read() {
            Ok(cache) => cache.get(cache_key).cloned(),
            Err(err) => err.into_inner().get(cache_key).cloned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ConfigSkillsCacheKey {
    REDACTED_SECRETs: Vec<(PathBuf, u8)>,
    skill_config_rules: SkillConfigRules,
}

pub fn bundled_skills_enabled_from_stack(
    config_layer_stack: &codex_config::ConfigLayerStack,
) -> bool {
    let effective_config = config_layer_stack.effective_config();
    let Some(skills_value) = effective_config
        .as_table()
        .and_then(|table| table.get("skills"))
    else {
        return true;
    };

    let skills: SkillsConfig = match skills_value.clone().try_into() {
        Ok(skills) => skills,
        Err(err) => {
            warn!("invalid skills config: {err}");
            return true;
        }
    };

    skills.bundled.unwrap_or_default().enabled
}

fn config_skills_cache_key(
    REDACTED_SECRETs: &[SkillRoot],
    skill_config_rules: &SkillConfigRules,
) -> ConfigSkillsCacheKey {
    ConfigSkillsCacheKey {
        REDACTED_SECRETs: REDACTED_SECRETs
            .iter()
            .map(|REDACTED_SECRET| {
                let scope_rank = match REDACTED_SECRET.scope {
                    SkillScope::Repo => 0,
                    SkillScope::User => 1,
                    SkillScope::System => 2,
                    SkillScope::Admin => 3,
                };
                (REDACTED_SECRET.path.clone(), scope_rank)
            })
            .collect(),
        skill_config_rules: skill_config_rules.clone(),
    }
}

fn finalize_skill_outcome(
    mut outcome: SkillLoadOutcome,
    disabled_paths: HashSet<PathBuf>,
) -> SkillLoadOutcome {
    outcome.disabled_paths = disabled_paths;
    let (by_scripts_dir, by_doc_path) =
        build_implicit_skill_path_indexes(outcome.allowed_skills_for_implicit_invocation());
    outcome.implicit_skills_by_scripts_dir = Arc:REDACTED_SECRETby_scripts_dir);
    outcome.implicit_skills_by_doc_path = Arc:REDACTED_SECRETby_doc_path);
    outcome
}

fn normalize_extra_user_REDACTED_SECRETs(extra_user_REDACTED_SECRETs: &[PathBuf]) -> Vec<PathBuf> {
    let mut normalized: Vec<PathBuf> = extra_user_REDACTED_SECRETs
        .iter()
        .map(|path| dunce::canonicalize(path).unwrap_or_else(|_| path.clone()))
        .collect();
    normalized.sort_unstable();
    normalized.dedup();
    normalized
}

#[cfg(test)]
#[path = "manager_tests.rs"]
mod tests;
