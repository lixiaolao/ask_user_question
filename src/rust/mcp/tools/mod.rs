// MCP工具注册模块
// 工具实现按各自的模块目录组织

pub mod interaction;

// 重新导出工具以便访问
pub use interaction::InteractionTool;
