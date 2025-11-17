use super::api::Plugin;
use crate::config::PluginsConfig;
use libloading::{Library, Symbol};
use std::fs;

type PluginInit = unsafe extern "C" fn() -> *mut dyn Plugin;

pub fn load_plugins(config: &PluginsConfig) -> anyhow::Result<Vec<Box<dyn Plugin>>> {
    let mut plugins: Vec<Box<dyn Plugin>> = Vec::new();
    let plugin_dir = match &config.directory {
        Some(dir) => dir,
        None => return Ok(plugins),
    };

    for entry in fs::read_dir(plugin_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let extension = path.extension().and_then(|s| s.to_str());
            if extension == Some("so") || extension == Some("dll") || extension == Some("dylib") {
                let library = unsafe { Library::new(&path) };
                match library {
                    Ok(lib) => {
                        let init_func: Result<Symbol<PluginInit>, _> =
                            unsafe { lib.get(b"_plugin_init") };
                        if let Ok(init) = init_func {
                            let plugin_ptr = unsafe { init() };
                            let plugin = unsafe { Box::from_raw(plugin_ptr) };
                            if config.enabled.contains(&plugin.name().to_string()) {
                                plugins.push(plugin);
                            }
                        } else {
                            log::warn!("Failed to find _plugin_init symbol in {}: {:?}", path.display(), init_func.err().unwrap());
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to load plugin library from {}: {:?}", path.display(), e);
                    }
                }
            }
        }
    }

    Ok(plugins)
}
