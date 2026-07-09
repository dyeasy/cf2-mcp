
/*
 * @Author: jiangxin
 * @Date: 2026-07-09 14:54:27
 * @Company: orientsec.com.cn
 * @Description: 
 */
use rmcp::{handler::server::wrapper::Parameters, tool, tool_router};

use crate::tools::{self, getscene::GetSceneParams};

#[derive(Clone, Default)]
pub struct Cf2;

// 2. 所有的工具都在这里“挂牌登记”
#[tool_router(server_handler)]
impl Cf2 {
    #[tool(description = "获取所有可用的场景从 packages目录下 (底层由 getscene 模块执行)")]
    pub async fn get_scene(&self, params: Parameters<GetSceneParams>) -> String {
        // 💡 核心魔法：在这里直接调用子文件的纯函数！
        tools::getscene::get_scene(params).await
    }
}