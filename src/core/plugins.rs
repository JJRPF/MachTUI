//! Dynamic Plugin System for MachTUI.
//! Allows loading external compiled components at runtime.

use crate::core::components::Component;
use libloading::{Library, Symbol};

pub struct PluginLoader {
    libraries: Vec<Library>,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            libraries: Vec::new(),
        }
    }

    /// Loads a plugin from a shared library file.
    /// The plugin must export a function: extern "C" fn create_component() -> *mut dyn Component
    pub unsafe fn load_component<P: libloading::AsFilename>(
        &mut self,
        path: P,
    ) -> Result<Box<dyn Component>, String> {
        let lib = Library::new(path).map_err(|e| e.to_string())?;

        type CreateComponent = unsafe extern "C" fn() -> *mut dyn Component;
        let constructor: Symbol<CreateComponent> =
            lib.get(b"create_component").map_err(|e| e.to_string())?;

        let component_ptr = constructor();
        let component = Box::from_raw(component_ptr);

        self.libraries.push(lib);
        Ok(component)
    }
}
