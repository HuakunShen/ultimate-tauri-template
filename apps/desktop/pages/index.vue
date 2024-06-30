<script setup lang="ts">
import {
  startServer,
  startServiceDiscoveryServer,
  stopServer,
  restartServer,
  serverIsRunning,
  restartServiceDiscoveryServer,
  stopServiceDiscoveryServer,
} from "@/lib/commands/server";

const serverUp = ref(false);
onMounted(() => {
  setInterval(() => {
    serverIsRunning().then((up) => (serverUp.value = up));
  }, 2000);
});
</script>
<template>
  <h1>Server</h1>
  <div class="grid grid-cols-3 gap-3 px-4 content-center">
    <el-button type="primary" @click="startServer">Start Server</el-button>
    <el-button type="primary" @click="stopServer">Stop Server</el-button>
    <el-button type="primary" @click="restartServer">Restart Server</el-button>
    <el-button type="primary" @click="startServiceDiscoveryServer"
      >Start Discovery Server</el-button
    >
    <el-button type="primary" @click="stopServiceDiscoveryServer"
      >Stop Discovery Server</el-button
    >
    <el-button type="primary" @click="restartServiceDiscoveryServer"
      >Restart Discovery Server</el-button
    >
  </div>
  <p><strong>Server Up: </strong>{{ serverUp }}</p>
  <el-button @click="ElMessage('hello')">button</el-button>
  <ElButton :icon="ElIconEditPen" type="success">button</ElButton>
</template>
