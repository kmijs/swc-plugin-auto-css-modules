use serde::Deserialize;
use swc_core::ecma::{
    ast::{ImportDecl, ImportSpecifier, Str},
    visit::{noop_visit_mut_type, VisitMut, VisitMutWith},
};

pub struct TransformVisitor {
    pub config: Config,
}

// 使用lazy_static或者直接用数组，const HashSet在Rust里还不太稳定
const CSS_EXTS: [&str; 5] = [".css", ".less", ".scss", ".sass", ".styl"];
const CORE_JS: &str = "core-js/";

impl VisitMut for TransformVisitor {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);

        self.rewrite_css_file_import(n);

        self.rewrite_core_js_import(n);
    }
}

impl TransformVisitor {
    fn rewrite_core_js_import(&self, n: &mut ImportDecl) {
        let core_js_pkg_path = &self.config.lock_core_js_pkg_path;
        if core_js_pkg_path.is_empty() {
            return;
        }

        // Wtf8Atom需要用as_str()转换为字符串切片，需要处理Option
        if let Some(source) = n.src.value.as_str() {
            if let Some(suffix) = source.strip_prefix(CORE_JS) {
                n.src = Box::new(Str {
                    span: n.src.span,
                    value: format!("{}/{}", core_js_pkg_path, suffix).into(),
                    raw: None,
                });
            }
        }
    }

    fn is_css_file(&self, value: &str) -> bool {
        CSS_EXTS.iter().any(|ext| value.ends_with(ext))
    }

    fn rewrite_css_file_import(&self, n: &mut ImportDecl) {
        if n.specifiers.len() == 1 {
            if let ImportSpecifier::Default(_) = &n.specifiers[0] {
                if let Some(import_source) = n.src.value.as_str() {
                    // 先检查是否已经是CSS文件
                    if self.is_css_file(import_source) {
                        // 避免重复添加后缀
                        let new_value = if import_source.ends_with(&self.config.style_file_suffix) {
                            import_source.to_string()
                        } else {
                            format!("{}{}", import_source, self.config.style_file_suffix)
                        };
                        
                        n.src = Box::new(Str {
                            span: n.src.span,
                            value: new_value.into(),
                            raw: None,
                        });
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "get_default_lock_core_js_pkg_path")]
    pub lock_core_js_pkg_path: String,

    #[serde(default = "get_default_style_file_suffix")]
    pub style_file_suffix: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            lock_core_js_pkg_path: get_default_lock_core_js_pkg_path(),
            style_file_suffix: get_default_style_file_suffix(),
        }
    }
}

impl Config {
    /// 验证配置的有效性
    pub fn validate(&self) -> Result<(), String> {
        // 检查style_file_suffix格式
        if !self.style_file_suffix.starts_with('?') {
            return Err("style_file_suffix应该以'?'开头，比如'?modules'".to_string());
        }
        
        // 检查是否包含非法字符
        if self.style_file_suffix.contains(' ') {
            return Err("style_file_suffix不能包含空格".to_string());
        }
        
        Ok(())
    }
}

fn get_default_style_file_suffix() -> String {
    "?modules".to_string()
}

fn get_default_lock_core_js_pkg_path() -> String {
    "".to_string()
}

pub fn auto_css_modules(config: Config) -> TransformVisitor {
    TransformVisitor { config }
}
