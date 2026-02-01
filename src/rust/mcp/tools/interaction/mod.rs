//! ask_user_question工具模块
//!
//! Ask the user a question with predefined options. Use this when you need the user to make a choice between specific options.

pub mod mcp;

// 重新导出主要类型和功能
pub use mcp::InteractionTool;
