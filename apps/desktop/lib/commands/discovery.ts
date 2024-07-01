import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

// export const ServiceDiscoverInfo  =z.object({
//     addr: SocketAddr,
//     service_port: u16,
// })

export function discoverPeers() {
  return invoke<any>("discovery_peers", {});
}
