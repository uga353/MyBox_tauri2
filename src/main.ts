import { createApp } from "vue";
import App from "./App.vue";
import { invoke } from '@tauri-apps/api/core';

// 创建Vue应用
const app = createApp(App);

// 挂载完成后，通知Rust后端：前端加载完毕
app.mount('#app').$nextTick(async () => {
  console.log('✅ Vue 前端初始化完成');
  // 调用Rust命令
  await invoke('set_loaded');
});
