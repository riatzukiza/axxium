use crate::shell::Shell;
use crate::shell::ShellType;
use crate::tools::handlers::agent_jobs::BatchJobHandler;
use crate::tools::handlers::multi_agents_common::DEFAULT_WAIT_TIMEOUT_MS;
use crate::tools::handlers::multi_agents_common::MAX_WAIT_TIMEOUT_MS;
use crate::tools::handlers::multi_agents_common::MIN_WAIT_TIMEOUT_MS;
use crate::tools::registry::ToolRegistryBuilder;
use codex_mcp::mcp::CODEX_APPS_MCP_SERVER_NAME;
use codex_mcp::mcp_connection_manager::ToolInfo;
use codex_protocol::dynamic_tools::DynamicToolSpec;
use codex_tools::DiscoverableTool;
use codex_tools::ToolHandlerKind;
use codex_tools::ToolRegistryPlanAppTool;
use codex_tools::ToolRegistryPlanParams;
use codex_tools::ToolUserShellType;
use codex_tools::ToolsConfig;
use codex_tools::WaitAgentTimeoutOptions;
use codex_tools::build_tool_registry_plan;
use std::collections::HashMap;
use std::sync::Arc;

pub(crate) fn tool_user_shell_type(user_shell: &Shell) -> ToolUserShellType {
    match user_shell.shell_type {
        ShellType::Zsh => ToolUserShellType::Zsh,
        ShellType::Bash => ToolUserShellType::Bash,
        ShellType::PowerShell => ToolUserShellType::PowerShell,
        ShellType::Sh => ToolUserShellType::Sh,
        ShellType::Cmd => ToolUserShellType::Cmd,
    }
}

pub(crate) fn build_specs_with_discoverable_tools(
    config: &ToolsConfig,
    mcp_tools: Option<HashMap<String, rmcp::model::Tool>>,
    app_tools: Option<HashMap<String, ToolInfo>>,
    discoverable_tools: Option<Vec<DiscoverableTool>>,
    dynamic_tools: &[DynamicToolSpec],
) -> ToolRegistryBuilder {
    use crate::tools::handlers::ApplyPatchHandler;
    use crate::tools::handlers::CodeModeExecuteHandler;
    use crate::tools::handlers::CodeModeWaitHandler;
    use crate::tools::handlers::DynamicToolHandler;
    use crate::tools::handlers::JsReplHandler;
    use crate::tools::handlers::JsReplResetHandler;
    use crate::tools::handlers::ListDirHandler;
    use crate::tools::handlers::McpHandler;
    use crate::tools::handlers::McpResourceHandler;
    use crate::tools::handlers::PlanHandler;
    use crate::tools::handlers::RequestPermissionsHandler;
    use crate::tools::handlers::RequestUserInputHandler;
    use crate::tools::handlers::ShellCommandHandler;
    use crate::tools::handlers::ShellHandler;
    use crate::tools::handlers::TestSyncHandler;
    use crate::tools::handlers::ToolSearchHandler;
    use crate::tools::handlers::ToolSuggestHandler;
    use crate::tools::handlers::UnifiedExecHandler;
    use crate::tools::handlers::ViewImageHandler;
    use crate::tools::handlers::multi_agents::CloseAgentHandler;
    use crate::tools::handlers::multi_agents::ResumeAgentHandler;
    use crate::tools::handlers::multi_agents::REDACTED_SECRETInputHandler;
    use crate::tools::handlers::multi_agents::SpawnAgentHandler;
    use crate::tools::handlers::multi_agents::WaitAgentHandler;
    use crate::tools::handlers::multi_agents_v2::CloseAgentHandler as CloseAgentHandlerV2;
    use crate::tools::handlers::multi_agents_v2::FollowupTaskHandler as FollowupTaskHandlerV2;
    use crate::tools::handlers::multi_agents_v2::ListAgentsHandler as ListAgentsHandlerV2;
    use crate::tools::handlers::multi_agents_v2::REDACTED_SECRETMessageHandler as REDACTED_SECRETMessageHandlerV2;
    use crate::tools::handlers::multi_agents_v2::SpawnAgentHandler as SpawnAgentHandlerV2;
    use crate::tools::handlers::multi_agents_v2::WaitAgentHandler as WaitAgentHandlerV2;

    let mut builder = ToolRegistryBuilder:REDACTED_SECRET);
    let app_tool_sources = app_tools.as_ref().map(|app_tools| {
        app_tools
            .values()
            .map(|tool| ToolRegistryPlanAppTool {
                tool_name: tool.tool_name.as_str(),
                tool_namespace: tool.tool_namespace.as_str(),
                server_name: tool.server_name.as_str(),
                connector_name: tool.connector_name.as_deref(),
                connector_description: tool.connector_description.as_deref(),
            })
            .collect::<Vec<_>>()
    });
    let default_agent_type_description =
        crate::agent::role::spawn_tool_spec::build(&std::collections::BTreeMap:REDACTED_SECRET));
    let plan = build_tool_registry_plan(
        config,
        ToolRegistryPlanParams {
            mcp_tools: mcp_tools.as_ref(),
            app_tools: app_tool_sources.as_deref(),
            discoverable_tools: discoverable_tools.as_deref(),
            dynamic_tools,
            default_agent_type_description: &default_agent_type_description,
            wait_agent_timeouts: WaitAgentTimeoutOptions {
                default_timeout_ms: DEFAULT_WAIT_TIMEOUT_MS,
                min_timeout_ms: MIN_WAIT_TIMEOUT_MS,
                max_timeout_ms: MAX_WAIT_TIMEOUT_MS,
            },
            codex_apps_mcp_server_name: CODEX_APPS_MCP_SERVER_NAME,
        },
    );
    let shell_handler = Arc:REDACTED_SECRETShellHandler);
    let unified_exec_handler = Arc:REDACTED_SECRETUnifiedExecHandler);
    let plan_handler = Arc:REDACTED_SECRETPlanHandler);
    let apply_patch_handler = Arc:REDACTED_SECRETApplyPatchHandler);
    let dynamic_tool_handler = Arc:REDACTED_SECRETDynamicToolHandler);
    let view_image_handler = Arc:REDACTED_SECRETViewImageHandler);
    let mcp_handler = Arc:REDACTED_SECRETMcpHandler);
    let mcp_resource_handler = Arc:REDACTED_SECRETMcpResourceHandler);
    let shell_command_handler = Arc:REDACTED_SECRETShellCommandHandler::from(config.shell_command_backend));
    let request_permissions_handler = Arc:REDACTED_SECRETRequestPermissionsHandler);
    let request_user_input_handler = Arc:REDACTED_SECRETRequestUserInputHandler {
        default_mode_request_user_input: config.default_mode_request_user_input,
    });
    let mut tool_search_handler = None;
    let tool_suggest_handler = Arc:REDACTED_SECRETToolSuggestHandler);
    let code_mode_handler = Arc:REDACTED_SECRETCodeModeExecuteHandler);
    let code_mode_wait_handler = Arc:REDACTED_SECRETCodeModeWaitHandler);
    let js_repl_handler = Arc:REDACTED_SECRETJsReplHandler);
    let js_repl_reset_handler = Arc:REDACTED_SECRETJsReplResetHandler);

    for spec in plan.specs {
        if spec.supports_parallel_tool_calls {
            builder.push_spec_with_parallel_support(
                spec.spec, /*supports_parallel_tool_calls*/ true,
            );
        } else {
            builder.push_spec(spec.spec);
        }
    }

    for handler in plan.handlers {
        match handler.kind {
            ToolHandlerKind::AgentJobs => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETBatchJobHandler));
            }
            ToolHandlerKind::ApplyPatch => {
                builder.register_handler(handler.name, apply_patch_handler.clone());
            }
            ToolHandlerKind::CloseAgentV1 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETCloseAgentHandler));
            }
            ToolHandlerKind::CloseAgentV2 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETCloseAgentHandlerV2));
            }
            ToolHandlerKind::CodeModeExecute => {
                builder.register_handler(handler.name, code_mode_handler.clone());
            }
            ToolHandlerKind::CodeModeWait => {
                builder.register_handler(handler.name, code_mode_wait_handler.clone());
            }
            ToolHandlerKind::DynamicTool => {
                builder.register_handler(handler.name, dynamic_tool_handler.clone());
            }
            ToolHandlerKind::FollowupTaskV2 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETFollowupTaskHandlerV2));
            }
            ToolHandlerKind::JsRepl => {
                builder.register_handler(handler.name, js_repl_handler.clone());
            }
            ToolHandlerKind::JsReplReset => {
                builder.register_handler(handler.name, js_repl_reset_handler.clone());
            }
            ToolHandlerKind::ListAgentsV2 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETListAgentsHandlerV2));
            }
            ToolHandlerKind::ListDir => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETListDirHandler));
            }
            ToolHandlerKind::Mcp => {
                builder.register_handler(handler.name, mcp_handler.clone());
            }
            ToolHandlerKind::McpResource => {
                builder.register_handler(handler.name, mcp_resource_handler.clone());
            }
            ToolHandlerKind::Plan => {
                builder.register_handler(handler.name, plan_handler.clone());
            }
            ToolHandlerKind::RequestPermissions => {
                builder.register_handler(handler.name, request_permissions_handler.clone());
            }
            ToolHandlerKind::RequestUserInput => {
                builder.register_handler(handler.name, request_user_input_handler.clone());
            }
            ToolHandlerKind::ResumeAgentV1 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETResumeAgentHandler));
            }
            ToolHandlerKind::REDACTED_SECRETInputV1 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETREDACTED_SECRETInputHandler));
            }
            ToolHandlerKind::REDACTED_SECRETMessageV2 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETREDACTED_SECRETMessageHandlerV2));
            }
            ToolHandlerKind::Shell => {
                builder.register_handler(handler.name, shell_handler.clone());
            }
            ToolHandlerKind::ShellCommand => {
                builder.register_handler(handler.name, shell_command_handler.clone());
            }
            ToolHandlerKind::SpawnAgentV1 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETSpawnAgentHandler));
            }
            ToolHandlerKind::SpawnAgentV2 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETSpawnAgentHandlerV2));
            }
            ToolHandlerKind::TestSync => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETTestSyncHandler));
            }
            ToolHandlerKind::ToolSearch => {
                if tool_search_handler.is_none() {
                    tool_search_handler = app_tools
                        .as_ref()
                        .map(|app_tools| Arc:REDACTED_SECRETToolSearchHandler:REDACTED_SECRETapp_tools.clone())));
                }
                if let Some(tool_search_handler) = tool_search_handler.as_ref() {
                    builder.register_handler(handler.name, tool_search_handler.clone());
                }
            }
            ToolHandlerKind::ToolSuggest => {
                builder.register_handler(handler.name, tool_suggest_handler.clone());
            }
            ToolHandlerKind::UnifiedExec => {
                builder.register_handler(handler.name, unified_exec_handler.clone());
            }
            ToolHandlerKind::ViewImage => {
                builder.register_handler(handler.name, view_image_handler.clone());
            }
            ToolHandlerKind::WaitAgentV1 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETWaitAgentHandler));
            }
            ToolHandlerKind::WaitAgentV2 => {
                builder.register_handler(handler.name, Arc:REDACTED_SECRETWaitAgentHandlerV2));
            }
        }
    }
    builder
}

#[cfg(test)]
#[path = "spec_tests.rs"]
mod tests;
