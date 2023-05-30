use crate::protocol::entities::{Torrent, TrackerUrl};
use crate::protocol::net::Peer;

pub trait NetworkClient {
    fn obtain_connection_id(&self, tracker: &TrackerUrl) -> Result<i64, String>;

    fn get_peers_list(&self, torrent: &Torrent, tracker: &TrackerUrl) -> Result<Vec<Peer>, String>;
}
