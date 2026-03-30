use super::api::Plugin;
use libloading::{Library, Symbol};
use std::collections::HashMap;

#[derive(Default)]
pub struct PluginManager {
    _libraries: Vec<Library>,
    pub plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            _libraries: Vec::new(),
            plugins: HashMap::new(),
        }
    }

    /// Load a dynamically linked plugin (e.g. .dll, .so, .dylib)
    pub unsafe fn load_plugin(&mut self, path: &str) -> std::result::Result<(), libloading::Error> {
        let lib = Library::new(path)?;
        
        // Find the C API standard initialization function
        let init_func: Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> = lib.get(b"init_plugin_v1\0")?;
        
        let raw_plugin = init_func();
        let mut plugin = Box::from_raw(raw_plugin);
        
        plugin.on_load();
        self.plugins.insert(plugin.name().to_string(), plugin);
        
        // Keep the library handle alive
        self._libraries.push(lib);
        
        Ok(())
    }
}
