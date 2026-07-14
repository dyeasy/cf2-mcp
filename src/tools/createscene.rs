/*
 * @Author: jiangxin
 * @Date: 2026-07-10 13:15:45
 * @Company: orientsec.com.cn
 * @Description:
 */

use rmcp::handler::server::wrapper::Parameters;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateSceneParams {
    /// 要创建的场景名称（例如：前置、资格审核、后置状态）一般是中文有可能最后有一几个英文字母
    name: String,
    /// 场景需要被创建到的目标绝对路径（必须以工作区根目录开头，例如：/Users/.../packages/fastman2-business-scenes）
    path: String,
}
pub async fn create_scene(
    Parameters(CreateSceneParams { name, path }): Parameters<CreateSceneParams>,
) -> Result<String, String> {
    eprintln!(
        "[Rust MCP] ⚡️ 大模型调用了 create_scene 工具！接收参数 name:{} path:{}",
        name, path
    );

    Ok(format!("成功创建{}", name))
}
