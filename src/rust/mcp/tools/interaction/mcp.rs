use anyhow::Result;
use rmcp::{Error as McpError, model::*};

use crate::mcp::{AskUserQuestionRequest, PopupRequest};
use crate::mcp::handlers::{create_tauri_popup, parse_mcp_response};
use crate::mcp::utils::{generate_request_id, popup_error};

/// ask_user_question工具
///
/// Ask the user a question with predefined options. Use this when you need the user to make a choice between specific options.
#[derive(Clone)]
pub struct InteractionTool;

impl InteractionTool {
    pub async fn ask_user_question(
        request: AskUserQuestionRequest,
    ) -> Result<CallToolResult, McpError> {
        let popup_request = PopupRequest {
            id: generate_request_id(),
            message: request.message,
            predefined_options: if request.predefined_options.is_empty() {
                None
            } else {
                Some(request.predefined_options)
            },
            is_markdown: request.is_markdown,
        };

        match create_tauri_popup(&popup_request) {
            Ok(response) => {
                // 解析响应内容，支持文本和图片
                let content = parse_mcp_response(&response)?;
                Ok(CallToolResult::success(content))
            }
            Err(e) => {
                Err(popup_error(e.to_string()).into())
            }
        }
    }
}
