use amethyst::{
    derive::SystemDesc,
    ecs::{System, SystemData},
    shrev::ReaderId,
    network::simulation::NetworkSimulationEvent,
};
use amethyst::core::ecs::{Read, Write};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use crate::events::AppEvent;
use amethyst::core::Time;
use std::time::Duration;
use bincode::{deserialize, serialize};
use crate::network;
use crate::resources::ServerAddress;

const RUN_EVERY_N_SEC: u64 = 1;
const PLAYER_NAME_MAGIC: &str = "Narancsos_Feco";

#[derive(SystemDesc)]
#[system_desc(name(ClientConnectSystemDesc))]
pub struct ClientConnectSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkSimulationEvent>,

    #[system_desc(skip)]
    last_run: Duration,
}

impl ClientConnectSystem {
    fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        ClientConnectSystem {
            reader,
            last_run: Duration::default()
        }
    }
}

impl<'s> System<'s> for ClientConnectSystem {
    type SystemData = (
        Read<'s, ServerAddress>,
        Read<'s, Time>,
        Write<'s, TransportResource>,
        Read<'s, EventChannel<NetworkSimulationEvent>>,
        Write<'s, EventChannel<AppEvent>>
    );

    fn run(&mut self, (server, time, mut net, net_event_ch, mut app_event): Self::SystemData) {
        let time_since_start = time.absolute_time();

        if (time_since_start-self.last_run) >= Duration::from_secs(RUN_EVERY_N_SEC) {
            self.last_run = time_since_start;
                let msg = serialize(&network::PackageType::ConnectionRequest { player_name: PLAYER_NAME_MAGIC.to_string() })
                    .expect("ConnectionRequest could not be serialized");

                log::info!("Sending message. Time: {}", time_since_start.as_secs_f32());
                net.send_with_requirements(server.address, &msg, DeliveryRequirement::Reliable, UrgencyRequirement::OnTick);
        }

        for event in net_event_ch.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, msg) => {
                    log::info!("Message: [{}], {:?}", addr, msg);
                    if &server.address == addr {
                        match deserialize(&msg) as bincode::Result<network::PackageType> {
                            Ok(package) => {
                                match package {
                                    network::PackageType::ConnectionResponse(result) => {
                                       // push event
                                        app_event.single_write(AppEvent::Connection(result));
                                    }
                                    _ => log::error!("Unexpected package from server")
                                }
                            }
                            Err(err) => log::error!("Connection response could not be deserialized. Cause: {}", err)
                        }
                    } else {
                        log::warn!("Unexpected message arrived from {} while waiting for connection response", addr);
                    }
                }
                _ => log::info!("Network event: {:?}", event)

            }
        }
    }
}
