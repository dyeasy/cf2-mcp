/*
 * @Author: jiangxin
 * @Date: 2026-07-15 16:54:48
 * @Company: orientsec.com.cn
 * @Description:
 */

use std::{
    collections::HashSet,
    fs::{self, read_to_string},
    sync::Arc,
};

use rmcp::handler::server::wrapper::Parameters;
use swc_core::{
    common::{FileName, SourceMap},
    css::parser::Parse,
    ecma::{
        ast::{ExportDefaultExpr, Expr, IdentName, ObjectLit, Prop, PropName, PropOrSpread},
        parser::{Parser, StringInput, Syntax, lexer::Lexer},
        visit::{Visit, VisitWith},
    },
};

#[derive(Debug, Default, serde::Deserialize, schemars::JsonSchema)]
struct AllScene {
    pub keys: HashSet<String>,
}

impl Visit for AllScene {
    fn visit_export_default_expr(&mut self, node: &ExportDefaultExpr) {
        // let bbb = Box::parse(node);
        let Expr::Object(ObjectLit { props, .. }) = node.expr.as_ref() else {
            return;
        };

        for p_s in props {
            // 安检门 1：我只要 Prop，不是 Prop 的统统踢走
            let PropOrSpread::Prop(prop) = p_s else {
                continue;
            };

            // 安检门 2：我只要 KeyValue 类型的 Prop，不是的也踢走
            let Prop::KeyValue(kv) = &**prop else {
                continue;
            };

            match &kv.key {
                PropName::Ident(IdentName { sym, .. }) => {
                    self.keys.insert(sym.to_string());
                }
                _ => {}
            }
        }
    }
}
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetAllSceneParams {
    /// 路径，这里是获取所有可用场景的路径（必须以工作区根目录开头，例如：/Users/.../packages/fastman2-business-scenes/map.ts）
    pub path: String,
}

pub async fn get_all_scene(
    Parameters(GetAllSceneParams { path }): Parameters<GetAllSceneParams>,
) -> Result<String, String> {
    eprintln!(
        "[Rust MCP] ⚡️ 大模型调用了 get_all_scene 工具！接收参数 path:{}",
        path
    );

    let mut allscene = AllScene {
        keys: Default::default(),
    };

    if let Ok(code) = read_to_string(path)
        && !code.is_empty()
    {
        let cm = Arc::new(SourceMap::default());
        let fm = cm.new_source_file(FileName::Custom("index.ts".into()).into(), code.to_string());

        let lexer = Lexer::new(
            Syntax::Typescript(Default::default()),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        let module = parser.parse_module().expect("Failed to parse");

        module.visit_with(&mut allscene);
    }

    let mut sorted: Vec<String> = allscene.keys.iter().cloned().collect();
    sorted.sort();

    println!("{:?}",sorted);

    Ok(format!("目前有以下可用的场景:{}", sorted.join(",")))
}
