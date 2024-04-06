//! Game project.
use fyrox::{
    core::{
        algebra::Vector2, pool::Handle, reflect::prelude::*, type_traits::prelude::*,
        visitor::prelude::*,
    },
    event::{ElementState, Event, WindowEvent},
    gui::message::UiMessage,
    keyboard::{KeyCode, PhysicalKey},
    plugin::{Plugin, PluginConstructor, PluginContext, PluginRegistrationContext},
    scene::{dim2::rigidbody::{self, RigidBody}, graph::Graph, Scene},
    script::{ScriptContext, ScriptTrait},
};
use std::{cell::RefCell, path::Path, rc::Rc};

#[derive(Visit, Reflect, Debug, Clone, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "2922cb59-aba7-46a1-aac7-5a3c6c3a7ded")]
#[visit(optional)]
struct Player {
    move_left: bool,
    move_right: bool,
    jump: bool,
    already_jumped: bool,
    jump_impulse: f32,
}

impl ScriptTrait for Player {
    fn on_init(&mut self, #[allow(unused_variables)] ctx: &mut ScriptContext) {}

    fn on_start(&mut self, #[allow(unused_variables)] ctx: &mut ScriptContext) {}

    fn on_os_event(
        &mut self,
        #[allow(unused_variables)] event: &Event<()>,
        #[allow(unused_variables)] ctx: &mut ScriptContext,
    ) {
        if let Event::WindowEvent { event, .. } = event {
            if let WindowEvent::KeyboardInput { event, .. } = event {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    let is_pressed = event.state == ElementState::Pressed;

                    match keycode {
                        KeyCode::KeyA => self.move_left = is_pressed,
                        KeyCode::KeyD => self.move_right = is_pressed,
                        KeyCode::Space => self.jump = is_pressed,
                        KeyCode::KeyS => self.already_jumped = is_pressed,
                        _ => (),
                    }
                }
            }
        }
    }

    fn on_update(&mut self, #[allow(unused_variables)] ctx: &mut ScriptContext) {
        if let Some(rigidbody) = ctx.scene.graph[ctx.handle].cast_mut::<RigidBody>() {
            let x_speed = if self.move_left {
                3.0
            } else if self.move_right {
                -3.0
            } else {
                0.0
            };

            let graph = RefCell::new(&ctx.scene.graph);
            self.check_ground_collision(graph, rigidbody);

            if self.jump && !self.already_jumped {
                // rigidbody.set_lin_vel(Vector2::new(x_speed, 4.0));
                rigidbody.apply_impulse(Vector2::new(0.0, self.jump_impulse));
                self.already_jumped = true;
            } else {
                rigidbody.set_lin_vel(Vector2::new(x_speed, rigidbody.lin_vel().y));
            }
        }
    }
}

impl Player {
    pub fn check_ground_collision(&mut self, graph: RefCell<&Graph>, rigidbody: &mut RigidBody) {
        for pair in graph[rigidbody.children()[0]].as_collider().contacts(&graph.physics) {
            if graph[graph[pair.collider1].parent()].has_script::<Ground>() || 
            graph[graph[pair.collider2].parent()].has_script::<Ground>() {
                self.already_jumped = false;
                break;
            }
        }
    }
}

#[derive(Visit, Reflect, Debug, Clone, Default, TypeUuidProvider, ComponentProvider)]
#[type_uuid(id = "62069772-2d47-4828-b97c-fe43d720e628")]
struct Ground;

impl ScriptTrait for Ground {
    fn on_update(&mut self, #[allow(unused_variables)] ctx: &mut ScriptContext) {}
}

pub struct GameConstructor;

impl PluginConstructor for GameConstructor {
    fn register(&self, context: PluginRegistrationContext) {
        let script_constructors = &context.serialization_context.script_constructors;
        script_constructors.add::<Player>("Player");
        script_constructors.add::<Ground>("Ground");
    }

    fn create_instance(&self, scene_path: Option<&str>, context: PluginContext) -> Box<dyn Plugin> {
        Box::new(Game::new(scene_path, context))
    }
}

pub struct Game {
    scene: Handle<Scene>,
}

impl Game {
    pub fn new(scene_path: Option<&str>, context: PluginContext) -> Self {
        context
            .async_scene_loader
            .request(scene_path.unwrap_or("data/scene.rgs"));

        Self {
            scene: Handle::NONE,
        }
    }
}

impl Plugin for Game {
    fn on_deinit(&mut self, _context: PluginContext) {
        // Do a cleanup here.
    }

    fn update(&mut self, _context: &mut PluginContext) {
        // Add your global update code here.
    }

    fn on_os_event(&mut self, _event: &Event<()>, _context: PluginContext) {
        // Do something on OS event here.
    }

    fn on_ui_message(&mut self, _context: &mut PluginContext, _message: &UiMessage) {
        // Handle UI events here.
    }

    fn on_scene_begin_loading(&mut self, path: &Path, ctx: &mut PluginContext) {
        if self.scene.is_some() {
            ctx.scenes.remove(self.scene);
        }
    }

    fn on_scene_loaded(
        &mut self,
        path: &Path,
        scene: Handle<Scene>,
        data: &[u8],
        context: &mut PluginContext,
    ) {
        self.scene = scene;
    }
}
