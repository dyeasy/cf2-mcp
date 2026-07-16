/*
 * @Author: jiangxin
 * @Date: 2026-07-08 14:05:12
 * @Company: orientsec.com.cn
 * @Description:
 */
use rmcp::{ServiceExt, handler::server::wrapper::Parameters, transport::stdio};

use crate::{
    entry::Cf2,
    tools::{
        createbusinessdir::CreateFolderBusinessParams, getallscene::GetAllSceneParams,
        getscene::GetSceneParams,
    },
};

// 引入你的子模块
mod entry;
mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("[RMCP] 🚀 多文件中心路由服务准备启动...");

    // 3. 极简启动，完全抛弃复杂的 merge()
    let service = Cf2::default().serve(stdio()).await?;
    service.waiting().await?;

    // let params = CreateFolderBusinessParams {
    //     path: "/Users/jiangxin/dfzq/dfyj-h5-v2/src/business-aaaa".to_string(),
    //     taskname: String::from("三方存管"),
    //     codetemplatepath:String::from("/Users/jiangxin/dfzq/dfyj-h5-v2/packages/create-fastman2-app/src/templates/business-tpl")
    // };
    // 2. 用 Parameters(...) 包装一层，直接调用！
    // let result_json = tools::getscene::get_scene(Parameters(params)).await;
    // let result_json = tools::createbusinessdir::create_business_dir(Parameters(params)).await;

    // let _ = tools::getallscene::get_all_scene(Parameters(GetAllSceneParams {
    //     path: String::from(
    //         "/Users/jiangxin/dfzq/dfyj-h5-v2/packages/fastman2-business-scenes/map.ts",
    //     ),
    // }))
    // .await;
    Ok(())
}
