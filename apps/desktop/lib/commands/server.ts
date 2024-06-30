import { invoke } from "@tauri-apps/api/core";

export function startServer() {
  return invoke<undefined>("start_server");
}

export function stopServer() {
  return invoke<undefined>("stop_server");
}

export function restartServer() {
  return invoke<undefined>("restart_server");
}

export function serverIsRunning() {
  return invoke<boolean>("server_is_running");
}

export function startServiceDiscoveryServer() {
  return invoke<undefined>("start_service_discovery_server");
}

export function stopServiceDiscoveryServer() {
  return invoke<undefined>("stop_service_discovery_server");
}

export function restartServiceDiscoveryServer() {
  return invoke<undefined>("restart_service_discovery_server");
}

export function serviceDiscoveryServerIsRunning() {
  return invoke<boolean>("service_discovery_server_is_running");
}
