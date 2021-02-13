use amethyst::core::ecs::{System, ReadStorage, WriteExpect, Join};
use amethyst::shred::ReadExpect;
use crate::resources::ClientRegistry;
use crate::components;
use amethyst::core::Transform;
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use westiny_common::network;
use amethyst::core::math::{Point2, UnitQuaternion};

/// This system is responsible for sending the transform of all the entities that has NetworkID
/// to every connected clients
pub struct EntityStateBroadcasterSystem;

impl<'s> System<'s> for EntityStateBroadcasterSystem {
    type SystemData = (
        ReadExpect<'s, ClientRegistry>,
        WriteExpect<'s, TransportResource>,
        ReadStorage<'s, components::NetworkId>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (client_registry, mut net, network_ids, transforms): Self::SystemData) {
        // TODO these should be sent in 1 message per client
        for (network_id, transform) in (&network_ids, &transforms).join() {
            let entity_state = network::EntityState {
                network_id: *network_id,
                position: Point2::new(transform.translation().x, transform.translation().y),
                rotation: get_angle(transform.rotation()),
            };

            let msg = bincode::serialize(&network::PacketType::EntityStateUpdate(entity_state)).expect("entity state update could not be serialized");
            client_registry.get_clients().iter().for_each(|&handle|{
                net.send_with_requirements(
                    handle.addr,
                    &msg,
                    DeliveryRequirement::UnreliableSequenced(None),
                    UrgencyRequirement::OnTick
                )
            })
        }
    }
}

fn get_angle(rotation: &UnitQuaternion<f32>) -> f32 {
    if rotation.coords.w < 0.0 {
        2.0 * std::f32::consts::PI - rotation.angle()
    } else {
        rotation.angle()
    }
}