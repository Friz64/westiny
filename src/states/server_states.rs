use crate::events::WestinyEvent;
use amethyst::prelude::*;
use amethyst::core::{Time, Transform};
use westiny_server::resources::{ClientRegistry, NetworkIdSupplier};
use westiny_common::components::{BoundingCircle};
use crate::resources::Collisions;
use amethyst::core::math::Point2;
use crate::states::game_states::barrel_positions;

use log::info;
use crate::components;

#[derive(Default)]
pub struct ServerState;

fn log_fps(time: &Time) {
    if time.frame_number() % 60 == 0 {
        // Note: this is not an average, calculated from the last frame delta.
        info!("FPS: {}", 1.0 / time.delta_real_seconds());
    }
}


fn log_clients(time: &Time, registry: &ClientRegistry) {
    if time.frame_number() % 600 == 0 {
        log::info!("Number of players online: {}", registry.client_count());
        log::debug!("{}", &registry);
    }
}


impl State<GameData<'static, 'static>, WestinyEvent> for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'static, 'static>>) {
        data.world.insert(ClientRegistry::new(16));
        data.world.insert(NetworkIdSupplier::new());
        data.world.insert(Collisions::default());

        data.world.register::<components::NetworkId>();
        data.world.register::<components::Player>();
        data.world.register::<components::Velocity>();
        data.world.register::<components::BoundingCircle>();
        data.world.register::<components::weapon::Weapon>();
        data.world.register::<Transform>();
        place_objects(data.world);
    }

    fn update(&mut self, data: StateData<'_, GameData<'static, 'static>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        let time = *data.world.fetch::<Time>();
        log_fps(&time);
        data.data.update(&data.world);
        log_clients(&time, &data.world.fetch::<ClientRegistry>());
        Trans::None
    }
}

fn place_objects(world: &mut World) {

    //TODO placing barrels and other objects should be based on a map
    for pos in barrel_positions() {
        place_barrel(world, pos);
    }
}

fn place_barrel(world: &mut World, pos: Point2<u32>) {

    let mut transform = Transform::default();
    transform.set_translation_xyz((pos.x as f32) * 16.0, (pos.y as f32) * 16.0, 0.0);

    world
        .create_entity()
        .with(transform)
        .with(BoundingCircle{radius: 8.0})
        .build();
}
