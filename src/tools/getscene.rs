/*
 * @Author: jiangxin
 * @Date: 2026-07-09 14:12:27
 * @Company: orientsec.com.cn
 * @Description:
 */

use rmcp::handler::server::wrapper::Parameters;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetSceneParams {
    path: String,
}

pub async fn get_scene(Parameters(GetSceneParams { path }): Parameters<GetSceneParams>) -> String {
    eprintln!(
        "[Rust MCP] ⚡️ 大模型调用了 get_scene 工具！接收参数 path:{}",
        path
    );
    path
}
