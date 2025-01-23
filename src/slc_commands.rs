use crossbeam_channel::Sender;
use wg_2024::{network::NodeId, packet::Packet};
// todo: receive messages, web page, drone events
// received nacks from drones should be redirected to the source (client/server)
// Commands sent by the Simulation Controller to a Client
pub enum ClientCommand {
    AddSender(NodeId, Sender<Packet>),
    SendChatText(String),
}

// Commands sent by the Simulation Controller to a Server
pub enum ServerCommand {
    AddSender(NodeId, Sender<Packet>),
}

// Command sent by a Client to the Simulation Controller
pub enum ClientEvent {
    PacketSent(Packet),
}

// Command sent by a Server to the Simulation Controller
pub enum ServerEvent {
    PacketSent(Packet),
}
