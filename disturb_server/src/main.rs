use bevy::prelude::*;
use bevy::{
    app::ScheduleRunnerSettings,
    prelude::App,
    prelude::{EventReader, ResMut},
    MinimalPlugins,
};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin};
use disturb_shared::ServerMessage;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::time::Duration;
#[derive(Serialize, Deserialize)]
struct NetworkHandle(u32);

fn main() {
    simple_logger::SimpleLogger::new()
        .env()
        .init()
        .expect("A logger was already initialized");

    App::build()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(
            1000 / 30,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(disturb_shared::network_channels_setup.system())
        .add_startup_system(server_setup_system.system())
        .add_system(handle_network_events_system.system())
        .add_system(read_server_message_channel_system.system())
        .run();
}

fn server_setup_system(mut net: ResMut<NetworkResource>) {
    let socket_address: SocketAddr = "192.168.0.100:5223".parse().expect("cannot parse ip");
    // let data_channel_address = "192.168.0.100:5224".parse().expect("cannot parse ip");
    // let public_address = "192.168.0.100:5225".parse().expect("cannot parse ip");
    println!("Listening... {:?}", socket_address);
    net.listen(socket_address.clone(), None, None);
}

fn read_server_message_channel_system(mut net: ResMut<NetworkResource>) {
    for (_, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();

        while let Some(message) = channels.recv::<ServerMessage>() {
            match message {
                ServerMessage::SimpleMessage(mesg) => log::info!("{}", mesg),
                _ => {}
            }
        }
    }
}

fn handle_network_events_system(
    mut net: ResMut<NetworkResource>,
    mut network_event_reader: EventReader<NetworkEvent>,
) {
    for event in network_event_reader.iter() {
        match event {
            NetworkEvent::Connected(handle) => {
                println!("New connection handle {}", handle);
                let it = net.connections.iter().map(|c| *c.0).collect::<Vec<_>>();
                log::info!("Connections {}", it.len());
                net.broadcast_message(ServerMessage::SimpleMessage("COUCOU".to_owned()));
            }
            _ => {}
        }
    }
}
