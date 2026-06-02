


use godot::prelude::*;


use godot::classes::{Engine, FileAccess};
mod godot_tokio;

use godot_tokio::AsyncRuntime;


struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {

     fn on_stage_init(level: InitStage) {
        match level {
            InitStage::Scene => {
                let mut engine = Engine::singleton();

                // This is where we register our async runtime singleton.
                engine.register_singleton(AsyncRuntime::SINGLETON, &AsyncRuntime::new_alloc());
            }
            _ => (),
        }
    }

    fn on_stage_deinit(level: InitStage) {
        match level {
            InitStage::Scene => {
                let mut engine = Engine::singleton();

                // Here is where we free our async runtime singleton from memory.
                if let Some(async_singleton) = engine.get_singleton(AsyncRuntime::SINGLETON) {
                    engine.unregister_singleton(AsyncRuntime::SINGLETON);
                    async_singleton.free();
                } else {
                    godot_warn!(
                        "Failed to find & free singleton -> {}",
                        AsyncRuntime::SINGLETON
                    );
                }
            }
            _ => (),
        }
    }
}

