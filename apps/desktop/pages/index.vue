<script setup lang="ts">
import {
  startServer,
  startServiceDiscoveryServer,
  stopServer,
  restartServer,
  serverIsRunning,
  restartServiceDiscoveryServer,
  stopServiceDiscoveryServer,
  serviceDiscoveryServerIsRunning,
  setServerProtocol,
} from "@/lib/commands/server";
import { discoverPeers } from "@/lib/commands/discovery";

const serverUp = ref(false);
const discoveryServerUp = ref(false);
const protocol = ref<"http" | "https">("http");

watch(protocol, (_protocol) => {
  setServerProtocol(_protocol)
    .then(() => {
      return restartServer();
    })
    .then(() => {
      ElNotification.success({
        title: `Protocol set to ${_protocol}`,
      });
    });
});

onMounted(() => {
  startServer();
  startServiceDiscoveryServer();
  setInterval(() => {
    serverIsRunning().then((up) => (serverUp.value = up));
    serviceDiscoveryServerIsRunning().then(
      (up) => (discoveryServerUp.value = up),
    );
  }, 2000);
});
</script>
<template>
  <div class="p-4">
    <h1 class="text-3xl">Server</h1>
    <div class="grid grid-cols-3 gap-3 content-center">
      <el-button class="!ml-0" type="primary" @click="startServer"
        >Start Server</el-button
      >
      <el-button class="!ml-0" type="primary" @click="stopServer"
        >Stop Server</el-button
      >
      <el-button class="!ml-0" type="primary" @click="restartServer"
        >Restart Server</el-button
      >
      <el-button
        class="!ml-0"
        type="success"
        @click="startServiceDiscoveryServer"
        >Start Discovery Server</el-button
      >
      <el-button
        class="!ml-0"
        type="success"
        @click="stopServiceDiscoveryServer"
        >Stop Discovery Server</el-button
      >
      <el-button
        class="!ml-0"
        type="success"
        @click="restartServiceDiscoveryServer"
        >Restart Discovery Server</el-button
      >
    </div>
    <p><strong>Server Up: </strong>{{ serverUp }}</p>
    <p><strong>Service Discovery Server Up: </strong>{{ discoveryServerUp }}</p>
    <div>
      <el-select
        v-model="protocol"
        placeholder="Protocol"
        size="large"
        style="width: 240px"
      >
        <el-option label="http" value="http" />
        <el-option label="https" value="https" />
      </el-select>
    </div>
    <el-button
      @click="
        () =>
          discoverPeers()
            .then((peers) => {
              console.debug(peers);
            })
            .catch((err) => {
              console.error(err);
            })
      "
      >Discover Peers</el-button
    >
  </div>
</template>
