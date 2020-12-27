
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::SpriteRender
};

use crate::components::Velocity;

pub fn spawn_bullet(transform: Transform, velocity: Velocity, sprite_render: SpriteRender,
                    entities: &Entities, lazy_update: &ReadExpect<LazyUpdate>) {
    lazy_update
        .create_entity(entities)
        .with(transform)
        .with(velocity)
        .with(sprite_render)
        .build();
}
