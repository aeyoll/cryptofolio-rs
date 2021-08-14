use std::sync::{Arc, RwLock};
use tera::Tera;

/// A `TemplateStore` contains a "global" templates reference, along
/// with an optional background thread for monitoring template changes for
/// automatic rebuilding.
#[derive(Debug)]
pub struct TemplateStore {
    pub templates: Arc<RwLock<Tera>>,
}

/// Loads a glob of Tera templates into memory behind an `Arc<RwLock<>>`. This can be
/// used in `app_data()` calls.
pub fn load() -> TemplateStore {
    let templates_glob = "server/templates/**/*.html";
    let templates = Arc::new(RwLock::new(
        Tera::new(&templates_glob).expect("Unable to compile templates!"),
    ));

    TemplateStore { templates }
}
