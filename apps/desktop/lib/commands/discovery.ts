import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

export const ServiceDiscoverInfo = z.object({
  addr: z.string(),
  port: z.number(),
});
export type ServiceDiscoverInfo = z.infer<typeof ServiceDiscoverInfo>;

export function discoverPeers() {
  return invoke<ServiceDiscoverInfo[]>("discovery_peers");
}
