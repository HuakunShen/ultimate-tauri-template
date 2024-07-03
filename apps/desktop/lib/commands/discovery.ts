import { invoke } from "@tauri-apps/api/core";
import { z } from "zod";

export const ServiceDiscoverInfo = z.object({
  addresses: z.string().array(),
  port: z.number(),
  fullname: z.string(),
  serviceType: z.string(),
  hostname: z.string(),
  subType: z.string().nullable()
});
export type ServiceDiscoverInfo = z.infer<typeof ServiceDiscoverInfo>;

export function getPeers() {
  return invoke<ServiceDiscoverInfo[]>("get_peers");
}
