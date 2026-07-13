/*
 * @Author: jiangxin
 * @Date: 2026-07-09 14:12:27
 * @Company: orientsec.com.cn
 * @Description:
 */

use globset::{Glob, GlobSet, GlobSetBuilder};
use rmcp::handler::server::wrapper::Parameters;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::{Component, Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SceneConfig {
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub disabled: bool,
}

pub enum FileTarget {
    Router {
        display_path: String,
        is_action: bool,
        is_view: bool,
    },
    Config(SceneConfig),
    Ignore,
}

fn create_global_matcher(patterns: &[&str]) -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    for g in patterns {
        builder.add(Glob::new(g).expect("Glob 模式语法编写错误！"));
    }
    builder.build().unwrap()
}

fn get_target_files<P: AsRef<Path>>(
    url: P,
) -> impl Iterator<Item = walkdir::Result<walkdir::DirEntry>> {
    const EXCLUSION_LIST: [&str; 4] = ["README.md", "__tests__", ".DS_Store", "component"];
    WalkDir::new(url)
        .min_depth(1)
        .max_depth(3)
        .into_iter()
        .filter_entry(|f| {
            let file_name = f.file_name().to_string_lossy();
            if EXCLUSION_LIST.contains(&file_name.as_ref()) {
                return false;
            }
            if f.depth() == 1 {
                // 第一层只允许文件夹进入
                return f.file_type().is_dir();
            }
            true
        })
}

pub fn read_scene_config<P: AsRef<Path>>(path: P) -> Result<SceneConfig, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}

pub fn file_target_type<P: AsRef<Path>>(
    path: P,
    base_path: &str,
    matcher_router: &GlobSet,
    matcher_config: &GlobSet,
) -> FileTarget {
    let path_ref = path.as_ref();
    let split_path = Path::new(base_path).file_name().expect("解析出错");
    if matcher_router.is_match(path_ref) {
        let parts: PathBuf = path_ref
            .components()
            .skip_while(|c| c.as_os_str() != split_path)
            .collect();
        let display_path = parts.to_string_lossy().into_owned();
        let file_name = path_ref
            .file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or_default();

        const ACTIONS_KEY: &str = "actions";
        const VIEWS_KEY: &str = "views";

        return FileTarget::Router {
            display_path,
            is_action: file_name.contains(ACTIONS_KEY)
                || path_ref
                    .parent()
                    .map_or(false, |f| f.ends_with(ACTIONS_KEY)),
            is_view: file_name.contains(VIEWS_KEY)
                || path_ref.parent().map_or(false, |f| f.ends_with(VIEWS_KEY)),
        };
    }

    if matcher_config.is_match(path_ref) {
        return match read_scene_config(path_ref) {
            Ok(config) => FileTarget::Config(config),
            Err(e) => {
                eprintln!("❌ 解析配置文件失败 {:?}: {}", path_ref, e);
                FileTarget::Ignore
            }
        };
    }
    FileTarget::Ignore
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetSceneParams {
    /// 场景的路径，必须是绝对的路径
    pub path: String,
    /// 当前业务需要使用到的目标场景名称列表（例如：["前置", "资格审核", "后置状态"]）
    pub scene: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneMetaData {
    pub key: String,
    pub scenename: String,
    pub actions: Vec<String>,
    pub views: Vec<String>,
    // #[serde(rename = "sceneData")]
    // pub scene_config: SceneConfig,
}

pub async fn get_scene(
    Parameters(GetSceneParams { path, scene }): Parameters<GetSceneParams>,
) -> String {
    eprintln!(
        "[Rust MCP] ⚡️ 大模型调用了 get_scene 工具！接收参数 path:{} scene:{:?}",
        path, scene
    );

    let target_dir_path = Path::new(&path);

    let aaaa = target_dir_path
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or("ddd");

    let matcher_router = create_global_matcher(&[
        "**/actions/*.ts",
        "**/actions.ts",
        "**/views/*.tsx",
        "**/views.tsx",
    ]);

    let matcher_scene_config = create_global_matcher(&["**/scene.json"]);

    let mut all_scene: HashMap<String, SceneMetaData> = HashMap::new();

    let iter = get_target_files(&path);

    for entry in iter {
        match entry {
            Ok(entry) => {
                let current_path = entry.path();

                let rel_path = current_path.strip_prefix(&target_dir_path).unwrap();

                let Some(scene_id) = rel_path
                    .components()
                    .next()
                    .and_then(|f| f.as_os_str().to_str())
                else {
                    continue;
                };

                let metadata = all_scene
                    .entry(scene_id.to_string())
                    .or_insert(SceneMetaData {
                        key: scene_id.to_string(),
                        actions: Vec::new(),
                        views: Vec::new(),
                        scenename: String::default(),
                    });
                let target =
                    file_target_type(current_path, &path, &matcher_router, &matcher_scene_config);

                match target {
                    FileTarget::Router {
                        display_path,
                        is_action,
                        is_view,
                    } => {
                        if is_action {
                            metadata.actions.push(display_path);
                        } else if is_view {
                            metadata.views.push(display_path);
                        }
                    }
                    FileTarget::Config(SceneConfig { title, .. }) => {
                        metadata.scenename = title;
                    }
                    FileTarget::Ignore => {}
                }
            }
            Err(e) => println!("Error: {:?}", e),
        }
    }

    all_scene.retain(|_folder_id, metadata| scene.contains(&metadata.scenename));

    serde_json::to_string(&all_scene).unwrap_or_else(|_| "{}".to_string())
}
