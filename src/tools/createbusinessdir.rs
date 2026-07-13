/*
 * @Author: jiangxin
 * @Date: 2026-07-10 09:43:24
 * @Company: orientsec.com.cn
 * @Description:
 */
use fs_extra::{copy_items, dir::{CopyOptions, copy, create}};
use rmcp::handler::server::wrapper::Parameters;
use std::{
    fs::{self, read_dir},
    path::{Path, PathBuf},
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct CreateFolderBusinessParams {
    /// 放置业务代码的路径（例如：/Users/.../src/todo-todo-todo）
    pub path: String,
    /// 任务名
    pub taskname: String,
    /// 这个是模板代码的路径 跟场景一下都放置在packages 目录下
    pub codetemplatepath: String,
}

pub async fn create_business_dir(
    Parameters(CreateFolderBusinessParams {
        path,
        taskname,
        codetemplatepath,
    }): Parameters<CreateFolderBusinessParams>,
) -> String {
    eprintln!(
        "[Rust MCP] ⚡️ 大模型调用了 create_dir 工具！接收参数 name:{},taskname:{},codetemplatepath:{}",
        path, taskname, codetemplatepath
    );

    let path_ref = Path::new(&path);

    let code_template_path_url = Path::new(&codetemplatepath);

    let _create = create(path_ref, bool::default());

    let entries: Vec<PathBuf> = fs::read_dir(code_template_path_url)
        .ok() // 1. Result -> Option<ReadDir> (如果出错就变 None)
        .into_iter() // 2. Option<ReadDir> -> Iterator (包含 1 个或 0 个 ReadDir)
        .flatten() // 3. 将 Iterator<ReadDir> 压平为 Iterator<DirEntry>
        .filter_map(|f| f.ok()) // 4. 过滤掉读取 DirEntry 过程中的内部错误
        .map(|f| f.path()) // 5. 提取路径
        .collect();

    let options = CopyOptions {
        copy_inside: true, // 如果目标已存在，拷贝到里面
        overwrite: true,   // 允许覆盖
        ..Default::default()
    };
    let _ccc=copy_items(&entries, path_ref, &options);
    "afdsafd".to_string()
}
