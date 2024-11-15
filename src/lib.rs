mod config;
mod tests;
mod visitor;

pub use config::TransformConfig;
use visitor::TransformVisitor;

use swc_core::ecma::{ast::Program, visit::{as_folder, FoldWith}};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<TransformConfig>(
        &metadata.get_transform_plugin_config().unwrap_or_default()
    ).unwrap_or_default();
    
    program.fold_with(&mut as_folder(TransformVisitor::new(config.attribute_name)))
}