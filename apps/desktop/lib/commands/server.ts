import { invoke } from "@tauri-apps/api/core";

export function setServerProtocol(protocol: "http" | "https") {
  return invoke<undefined>("set_server_protocol", { protocol });
}

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
