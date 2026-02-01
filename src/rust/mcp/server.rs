use anyhow::Result;
use rmcp::{
    Error as McpError, ServerHandler, ServiceExt, RoleServer,
    model::*,
    transport::stdio,
    service::RequestContext,
};

use super::tools::InteractionTool;
use super::types::AskUserQuestionRequest;
use crate::{log_important, log_debug};

#[derive(Clone, Default)]
pub struct AskUserQuestionServer;

impl AskUserQuestionServer {
    pub fn new() -> Self {
        Self
    }
}

impl ServerHandler for AskUserQuestionServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "ask_user_question-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some("Ask the user a question with predefined options. Use this when you need the user to make a choice between specific options. You can provide up to 4 options, each with a label and description. NEVER include \"other\" as an option - the user can always automatically provide a custom response.".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ServerInfo, McpError> {
        Ok(self.get_info())
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        use std::sync::Arc;
        use std::borrow::Cow;

        let mut tools = Vec::new();

        // ask_user_question工具
        let ask_user_question_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "要显示给用户的消息"
                },
                "predefined_options": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "预定义的选项列表（可选）"
                },
                "is_markdown": {
                    "type": "boolean",
                    "description": "消息是否为Markdown格式，默认为true"
                }
            },
            "required": ["message"]
        });

        if let serde_json::Value::Object(schema_map) = ask_user_question_schema {
            tools.push(Tool {
                name: Cow::Borrowed("ask_user_question"),
                description: Some(Cow::Borrowed("Ask the user a question with predefined options. Use this when you need the user to make a choice between specific options. You can provide up to 4 options, each with a label and description. NEVER include \"other\" as an option - the user can always automatically provide a custom response.")),
                input_schema: Arc::new(schema_map),
                annotations: None,
            });
        }

        log_debug!("返回给客户端的工具列表: {:?}", tools.iter().map(|t| &t.name).collect::<Vec<_>>());

        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        log_debug!("收到工具调用请求: {}", request.name);

        match request.name.as_ref() {
            "ask_user_question" => {
                // 解析请求参数
                let arguments_value = request.arguments
                    .map(serde_json::Value::Object)
                    .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

                let ask_user_question_request: AskUserQuestionRequest = serde_json::from_value(arguments_value)
                    .map_err(|e| McpError::invalid_params(format!("参数解析失败: {}", e), None))?;

                // 调用ask_user_question工具
                InteractionTool::ask_user_question(ask_user_question_request).await
            }
            _ => {
                Err(McpError::invalid_request(
                    format!("未知的工具: {}", request.name),
                    None
                ))
            }
        }
    }
}



/// 启动MCP服务器
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // 创建并运行服务器
    let service = AskUserQuestionServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            log_important!(error, "启动服务器失败: {}", e);
        })?;

    // 等待服务器关闭
    service.waiting().await?;
    Ok(())
}
