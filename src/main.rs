/*
 * @Author: jiangxin
 * @Date: 2026-07-08 14:05:12
 * @Company: orientsec.com.cn
 * @Description:
 */
use rmcp::{ServiceExt, transport::stdio};

use crate::entry::Cf2;

// 引入你的子模块
mod entry;
mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    eprintln!("[RMCP] 🚀 多文件中心路由服务准备启动...");

    // 3. 极简启动，完全抛弃复杂的 merge()
    let service = Cf2::default().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
