//! Executor with your game connected to it as a plugin.
use fyrox::engine::executor::Executor;
use game1_take2::GameConstructor;

fn main() {
    let mut executor = Executor::new();
    executor.add_plugin_constructor(GameConstructor);
    executor.run()
}