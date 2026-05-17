use tokio::sync::mpsc::UnboundedSender;
use crate::protocol::response::Response;

pub struct Player {
    pub name: String,
    pub addr: String,
    pub room: String,
    pub tx: UnboundedSender<Response>
}