/*
 * @Author: jiangxin
 * @Date: 2026-07-10 09:43:24
 * @Company: orientsec.com.cn
 * @Description:
 */
use rmcp::handler::server::wrapper::Parameters;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateFolderBusinessParams {
    /// 放置业务代码的路径（例如：/Users/.../src/todo-todo-todo）
    path: String,
}

pub async fn create_folder_business(
    Parameters(CreateFolderBusinessParams { path }): Parameters<CreateFolderBusinessParams>,
) -> String {
    eprintln!(
        "[Rust MCP] ⚡️ 大模型调用了 create_dir 工具！接收参数 name:{}",
        path
    );

    "afdsafd".to_string()
}
