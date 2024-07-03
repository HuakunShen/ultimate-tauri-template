<script setup lang="ts">
import {
  startServer,
  stopServer,
  restartServer,
  serverIsRunning,
  setServerProtocol
} from "@/lib/commands/server";
import { getPeers, Peers, ServiceDiscoverInfo } from "@/lib/commands/discovery";

const serverUp = ref(false);
const discoveryServerUp = ref(false);
const protocol = ref<"http" | "https">("http");
const peers = ref<Peers>({});

watch(protocol, (_protocol) => {
  setServerProtocol(_protocol)
    .then(() => {
      return restartServer();
    })
    .then(() => {
      ElNotification.success({
        title: `Protocol set to ${_protocol}`
      });
    });
});

onMounted(() => {
  startServer();
  setInterval(() => {
    serverIsRunning().then((up) => (serverUp.value = up));
    getPeers().then((p) => {
      peers.value = p;
    });
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
    <ul class="list-decimal ml-8">
      <li v-for="([hostname, peer], idx) in Object.entries(peers)" :key="idx">
        {{ hostname }}
        <pre
          >{{ JSON.stringify(peer, null, 2) }}
        </pre>
      </li>
    </ul>
  </div>
</template>
