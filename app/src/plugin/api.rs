pub trait Plugin {
    fn name(&self) -> &'static str;
    fn on_load(&mut self);
    fn process_text(&mut self, text: &mut String);
}

// Ensure the symbol is unmangled so it can be found by `libloading`
// A plugin will need to provide `#[no_mangle] pub extern "C" fn init_plugin_v1() -> *mut dyn Plugin`
