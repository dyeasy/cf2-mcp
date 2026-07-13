/*
 * @Author: jiangxin
 * @Date: 2026-07-09 14:54:27
 * @Company: orientsec.com.cn
 * @Description:
 */
use rmcp::{handler::server::wrapper::Parameters, tool, tool_router};

use crate::tools::{
    createfolder::{CreateFolderBusinessParams, create_folder_business},
    createscene::{CreateSceneParams, create_scene},
    getscene::{GetSceneParams, get_scene},
};

#[derive(Clone, Default)]
pub struct Cf2;

// 2. 所有的工具都在这里“挂牌登记”
#[tool_router(server_handler)]
impl Cf2 {
    #[tool(description = "获取所有可用的场景从 packages目录下 (底层由 get_scene 模块执行)")]
    pub async fn get_scene(&self, params: Parameters<GetSceneParams>) -> String {
        get_scene(params).await
    }
    #[tool(description = "用来放置业务代码的文件夹，也就是开发目录")]
    pub async fn create_folder_business(
        &self,
        params: Parameters<CreateFolderBusinessParams>,
    ) -> String {
        create_folder_business(params).await
    }
    #[tool(description = "这是用来创建场景的")]
    pub async fn create_scene(&self, params: Parameters<CreateSceneParams>) -> String {
        create_scene(params).await
    }
}
