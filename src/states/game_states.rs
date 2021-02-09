use amethyst::{
    core::{
        transform::Transform,
        math::{Point2, Vector3},
        ecs::{Dispatcher, DispatcherBuilder},
        SystemBundle,
        ArcThreadPool,
    },
    input::{is_close_requested, is_key_down, VirtualKeyCode, InputBundle},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};
use std::path::PathBuf;

use crate::entities::initialize_tilemap;
use westiny_client::MovementBindingTypes;
use crate::resources::{
    Collisions,
    ProjectileCollisions,
    SpriteResource,
    SpriteId,
};
use westiny_common::components::BoundingCircle;
use westiny_common::resources::AudioQueue;
use westiny_client::systems::{AudioPlayerSystem};
use westiny_client::resources::initialize_audio;
use crate::events::WestinyEvent;
use crate::systems;

// later, other states like "MenuState", "PauseState" can be added.
pub struct PlayState {
    dispatcher: Option<Dispatcher<'static, 'static>>,
    resource_dir: PathBuf,
    sprite_resource: SpriteResource,
}

impl PlayState {
    pub fn new(resource_dir: &std::path::Path, sprite_resource: SpriteResource) -> Self {
        PlayState {
            dispatcher: Default::default(),
            resource_dir: resource_dir.to_path_buf(),
            sprite_resource
        }
    }
}

impl State<GameData<'static, 'static>, WestinyEvent> for PlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();

        let key_bindings = self.resource_dir.join("input.ron");

        InputBundle::<MovementBindingTypes>::new().with_bindings_from_file(key_bindings).unwrap()
            .build(&mut world, &mut dispatcher_builder).unwrap();

        let mut dispatcher = dispatcher_builder
            // .with(systems::InputDebugSystem::default(), "input_debug_system", &["input_system"])
            .with(systems::InputStateSystem, "input_state_system", &["input_system"])
            .with(systems::CameraMovementSystem, "camera_movement_system", &["input_system"])
            .with(systems::PlayerMovementSystem, "player_movement_system", &["input_state_system"])
            .with(systems::PhysicsSystem, "physics_system", &["player_movement_system"])
            .with(systems::CollisionSystem, "collision_system", &["physics_system"])
            .with(systems::CollisionHandlerForObstacles, "collision_handler_for_obstacles", &["collision_system"])
            .with(systems::ProjectileCollisionSystem, "projectile_collision_system", &["collision_system"])
            .with(systems::ProjectileCollisionHandler, "projectile_collision_handler", &["projectile_collision_system"])
            .with(systems::PlayerShooterSystem, "player_shooter_system", &["input_system"])
            .with(systems::CursorPosUpdateSystem, "cursor_pos_update_system", &["camera_movement_system"])
            .with(AudioPlayerSystem, "audio_player_system", &["cursor_pos_update_system"])

            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();
        dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        world.insert(Collisions::default());
        world.insert(ProjectileCollisions::default());
        world.insert(AudioQueue::default());
        init_camera(world, &dimensions);

        let objects_reference_pos = Point2::new(
            dimensions.width() * 0.5,
            dimensions.height() * 0.5
        );
        initialize_tilemap(world, &self.sprite_resource, Point2::new(dimensions.width() / 2.0, dimensions.height() / 2.0));
        initialize_audio(world);
        place_objects(world, &self.sprite_resource, &objects_reference_pos);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: WestinyEvent
    ) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        if let WestinyEvent::EngineEvent(StateEvent::Window(event)) = &event {
            if is_close_requested(event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        Trans::None
    }

    fn update(&mut self, data: StateData<GameData<'_, '_>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        if let Some(dispatcher) = self.dispatcher.as_mut() {
            dispatcher.dispatch(&data.world);
        }
        data.data.update(&data.world);
        Trans::None
    }
}

fn place_objects(world: &mut World, sprites: &SpriteResource, objects_reference_pos: &Point2<f32>) {
    //TODO placing barrels and other objects should be based on a map
    place_barrel(world, &sprites, objects_reference_pos, 3, 3);
    place_barrel(world, &sprites, objects_reference_pos, 3, 5);
    place_barrel(world, &sprites, objects_reference_pos, 3, 6);
    place_barrel(world, &sprites, objects_reference_pos, 3, 7);
    place_barrel(world, &sprites, objects_reference_pos, 3, 8);
    place_barrel(world, &sprites, objects_reference_pos, 4, 8);
    place_barrel(world, &sprites, objects_reference_pos, 5, 8);
    place_barrel(world, &sprites, objects_reference_pos, 5, 7);
}

fn place_barrel(world: &mut World, sprites: &SpriteResource, objects_reference_pos: &Point2<f32>, x: u32, y: u32) {

    let mut transform = Transform::default();
    transform.set_translation_xyz(objects_reference_pos.x + (x as f32) * 16.0, objects_reference_pos.y + (y as f32) * 16.0, 0.0);

    world
        .create_entity()
        .with(sprites.sprite_render_for(SpriteId::Barrel))
        .with(transform)
        .with(BoundingCircle{radius: 8.0})
        .build();
}

pub fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        dimensions.width() * 0.5,
        dimensions.height() * 0.5,
        1.0);

    // Zoom-in
    transform.set_scale(Vector3::new(0.25, 0.25, 1.0));

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}
