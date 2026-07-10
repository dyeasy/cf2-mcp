/*
 * @Author: jiangxin
 * @Date: 2026-07-08 14:05:12
 * @Company: orientsec.com.cn
 * @Description:
 */
use rmcp::{ServiceExt, handler::server::wrapper::Parameters, transport::stdio};

use crate::{entry::Cf2, tools::getscene::GetSceneParams};

// 引入你的子模块
mod entry;
mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("[RMCP] 🚀 多文件中心路由服务准备启动...");

    // 3. 极简启动，完全抛弃复杂的 merge()
    // let service = Cf2::default().serve(stdio()).await?;
    // service.waiting().await?;

    let params = GetSceneParams {
        path: "/Users/jiangxin/dfzq/dfyj-h5-v2/packages/fastman2-business-scenes".to_string(),
        scene: vec!["状态查询".to_string(), "资格审查".to_string()],
    };

    // 2. 用 Parameters(...) 包装一层，直接调用！
    let result_json = tools::getscene::get_scene(Parameters(params)).await;
    Ok(())
}
