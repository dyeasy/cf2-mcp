/*
 * @Author: jiangxin
 * @Date: 2026-07-10 09:43:24
 * @Company: orientsec.com.cn
 * @Description:
 */
use globset::{Glob, GlobSet, GlobSetBuilder};
// use fs_extra::{
//     copy_items,
//     dir::{CopyOptions, copy, create},
// };
use rmcp::handler::server::wrapper::Parameters;
use std::{
    error::Error,
    fs::create_dir,
    path::{Path, PathBuf},
};
use tokio::fs::{self, create_dir_all, read_to_string, write};
use walkdir::{DirEntry, WalkDir};

fn create_global_matcher(patterns: &[&str]) -> Result<GlobSet, Box<dyn Error>> {
    let mut builder = GlobSetBuilder::new();
    for g in patterns {
        builder.add(Glob::new(g)?);
    }
    Ok(builder.build()?)
}

enum MatchFile {
    App,
    MutationsIndex,
    Declare,
    Types,
    EffectsMain,
    EffectsIndex,
}

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
) -> Result<String, String> {
    eprintln!(
        "[Rust MCP] ⚡️ 大模型调用了 create_dir 工具！接收参数 name:{},taskname:{},codetemplatepath:{}",
        path, taskname, codetemplatepath
    );

    let path_ref = Path::new(&path);

    let code_template_path_url = Path::new(&codetemplatepath);

    let _create = create_dir(path_ref);

    // 用于收集成功创建的文件，最后返回给大模型，极其有助于大模型理解上下文
    let mut created_files = Vec::new();

    let path_matcher = create_global_matcher(&[
        "**/app.ts",
        "**/declare.ts",
        "**/types.ts",
        "**/effects/*.ts",
        "**/mutations/index.ts",
    ])
    .map_err(|err| format!("路径匹配器创建失败:{}", err))?;

    // 2. 优化：直接使用 WalkDir，去除了多余且易报错的单个 create_dir 步骤
    let dirs = WalkDir::new(code_template_path_url)
        .min_depth(1)
        .max_depth(2)
        .into_iter()
        .filter_map(|f| f.ok())
        .map(|p| p.into_path());

    for entry in dirs {
        if entry.is_file() {
            // 排除 macOS 自动生成的垃圾文件（防止污染项目）
            if entry.file_name().and_then(|n| n.to_str()) == Some(".DS_Store") {
                continue;
            }

            let code = read_to_string(&entry)
                .await
                .map_err(|e| format!("读取模板文件失败 {:?}: {}", entry, e))?;

            if path_matcher.is_match(&entry) {
                println!("匹配成功需要修改:{:?}", entry)
            }

            let current_path = match entry.strip_prefix(code_template_path_url) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let new_path = path_ref.join(current_path);

            if let Some(parent) = new_path.parent() {
                create_dir_all(parent)
                    .await
                    .map_err(|e| format!("创建目录失败 {:?}: {}", parent, e))?;
            }

            write(&new_path, code)
                .await
                .map_err(|e| format!("写入文件失败 {:?}: {}", new_path, e))?;

            if let Some(path_str) = current_path.to_str() {
                created_files.push(path_str.to_string());
            }
        }
    }
    if created_files.is_empty() {
        return Err("未能在模板路径中找到任何有效文件进行复制".to_string());
    }

    let file_list = created_files
        .iter()
        .map(|f| format!("  - {}", f))
        .collect::<Vec<_>>()
        .join("\n");
    Ok(format!(
        "🎉 成功在目标路径创建业务目录 '{}'！\n目标全路径: {:?}\n\n已生成以下文件：\n{}",
        taskname, path, file_list
    ))
}
