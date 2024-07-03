use std::{collections::HashSet, sync::Mutex};
use tauri_plugin_network::network::mdns::ServiceInfoMod;

#[derive(Default, Debug)]
pub struct Peers {
    pub peers: Mutex<HashSet<ServiceInfoMod>>,
}

impl Peers {
    pub fn add_peer(&self, peer: ServiceInfoMod) {
        println!("Added peer: {:?}", peer.clone());
        let mut peers = self.peers.lock().unwrap();
        peers.insert(peer);
    }

    pub fn remove_peer(&self, service_type: String, fullname: String) {
        let mut peers = self.peers.lock().unwrap();
        // filter out the peer
        peers.retain(|peer| peer.service_type != service_type || peer.fullname != fullname);
    }

    pub fn clear(&self) {
        let mut peers = self.peers.lock().unwrap();
        peers.clear();
    }

    pub fn set_peers(&self, peers: HashSet<ServiceInfoMod>) {
        self.clear();
        self.peers.lock().unwrap().extend(peers);
    }
}

#[tauri::command]
pub async fn get_peers(state: tauri::State<'_, Peers>) -> Result<HashSet<ServiceInfoMod>, String> {
    let _peers = state.peers.lock().unwrap();
    Ok(_peers.to_owned())
}
