use swc_core::{
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
pub fn process(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    // 安全地获取和解析配置
    let config_str = data.get_transform_plugin_config().unwrap_or_default();
    let config = if config_str.is_empty() {
        // 空配置时使用默认配置
        auto_css_modules::Config::default()
    } else {
        match serde_json::from_str::<auto_css_modules::Config>(&config_str) {
            Ok(config) => {
                // 验证配置
                if let Err(e) = config.validate() {
                    eprintln!("配置验证失败: {}, 使用默认配置", e);
                    auto_css_modules::Config::default()
                } else {
                    config
                }
            }
            Err(e) => {
                eprintln!("配置解析失败: {}, 使用默认配置", e);
                auto_css_modules::Config::default()
            }
        }
    };

    program.visit_mut_with(&mut ::auto_css_modules::auto_css_modules(config));
    program
}
