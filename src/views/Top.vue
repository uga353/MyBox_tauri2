<template>
  <div>
    <!-- 分割线 -->
    <div class="full-line"></div>

    <!-- 顶部导航工具栏 -->
    <div class="top-bar">
      <button class="icon-btn" title="后退" @click="goBack">🢀</button>
      <button class="icon-btn" title="前进" @click="goForward">🢂</button>
      <button class="icon-btn" title="刷新" @click="refresh">🔄</button>
      <button class="icon-btn" title="首页" @click="goHome">🏠</button>
      <button class="icon-btn" title="收藏" @click="toggleFavorite">⭐</button>
      <ul id="favoriteList" style="display: none;"></ul>

      <input
        v-model="url"
        class="url-input"
        placeholder="输入网址、搜索内容..."
        @keydown.enter="goToUrl"
      />

      <button class="btn" title="加入收藏" @click="addFavorite">➕︎</button>
    </div>
    
    <!-- 分割线 -->
    <div class="full-line"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
// 引入 Tauri 通信
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

document.addEventListener('contextmenu', e => e.preventDefault());
const url = ref('')

// ========== 真正控制底部网页 ==========
async function goBack() {
  await invoke("webview_go_back")
}
async function goForward() {
  await invoke("webview_go_forward")
}
async function refresh() {
  await invoke("webview_reload")
}
async function goHome() {
  url.value = "http://localhost:1420/"
  await invoke("webview_navigate", { url: url.value })
}
async function goToUrl() {
  await invoke("handle_address_input", { input: url.value })
}

onMounted(async () => {
  await listen("url_updated", (res) => {
    url.value = res.payload
  })
  await invoke("start_url_sync")
})

// 提示框
function showAlert(msg) {
  const div = document.createElement('div');
  div.style.cssText = `
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    background: #1a73e8;
    color: white;
    padding: 80px 100px;
    border-radius: 2px;
    font-size: 20px;
    font-weight: 500;
    z-index: 999999;
    box-shadow: 0 2px 10px rgba(0,0,0,0.2);
  `;
  div.innerText = msg;
  document.body.appendChild(div);
  setTimeout(() => div.remove(), 2222);
}

const webPageTitle = ref('')
// 网页加载完成时自动更新标题
function onWebViewLoaded() {
  const webview = document.querySelector('webview');
  if (webview) {
    webPageTitle.value = webview.getTitle() || url.value.replace(/https?:\/\/(www\.)?/i, '').split('/')[0];
  }
}

// 收藏
function addFavorite() {
  const currentUrl = url.value.trim();

  if (!currentUrl || currentUrl === 'about:blank') {
    showAlert('请打开网页后再收藏！');
    return;
  }
  // 自动生成干净标题（永远正确）
  const title = webPageTitle.value || currentUrl.replace(/https?:\/\/(www\.)?/i, '').split('/')[0];
  addToFavoriteList(currentUrl, title);
}

function addToFavoriteList(link, title) {
let favorites = JSON.parse(localStorage.getItem('favorites') || '[]');
  
  if (favorites.some(item => item.url === link)) {
    showAlert('已收藏！');
    return;
  }

  favorites.push({
    url: link,
    title: title,
    time: new Date().toLocaleString()
  });

  localStorage.setItem('favorites', JSON.stringify(favorites));
  showAlert('收藏成功！');
  showFavoriteList();
}

// 显示/关闭收藏
function toggleFavorite() {
  const list = document.getElementById("favoriteList");
  if (!list) return;
  showFavoriteList();
  list.style.display = list.style.display === "none" ? "block" : "none";
}

// 显示收藏列表
function showFavoriteList() {
  const list = document.getElementById("favoriteList");
  if (!list) return;

  let favorites = JSON.parse(localStorage.getItem('favorites') || '[]');
  list.innerHTML = "";
  list.style.padding = "0";
  list.style.margin = "8px 0";
  list.style.listStyle = "none";

  if (favorites.length === 0) {
    const li = document.createElement("li");
    li.innerText = "暂无收藏";
    li.style.padding = "10px";
    li.style.textAlign = "center";
    li.style.color = "#1a73e8";
    li.style.fontSize = "14px";
    list.appendChild(li);
    return;
  }

  favorites.forEach(item => {
    const li = document.createElement("li");
    li.style.display = "flex";
    li.style.justifyContent = "space-between";
    li.style.alignItems = "center";
    li.style.padding = "10px 12px";
    li.style.margin = "4px 0";
    li.style.background = "#f5f5f5";
    li.style.borderRadius = "6px";
    li.style.cursor = "pointer";

    // 标题
    const title = document.createElement("span");
    title.innerText = item.title;
    title.style.flex = "1";
    title.style.color = "#333";
    title.style.fontSize = "15px";
    title.onclick = () => openFavorite(item.url);

    // 删除按钮
    const del = document.createElement("button");
    del.innerText = "×";
    del.style.border = "none";
    del.style.background = "none";
    del.style.color = "red";
    del.style.fontSize = "16px";
    del.style.cursor = "pointer";
    del.onclick = () => deleteFavorite(item.url);

    li.appendChild(title);
    li.appendChild(del);
    list.appendChild(li);
  });
}

// 打开收藏一链接
function openFavorite(link) {
  url.value = link;
  goToUrl();
}

// 删除收藏一链接
function deleteFavorite(link) {
  let favorites = JSON.parse(localStorage.getItem('favorites') || '[]');
  favorites = favorites.filter(item => item.url !== link);
  localStorage.setItem('favorites', JSON.stringify(favorites));
  showFavoriteList();
}

</script>

<style>
/* ============================================== */
/* ======== 全局永久隐藏所有滚动条 ========== */
/* ============================================== */
html, body {
  /* 禁用滚动 + 隐藏滚动条 */
  overflow: hidden !important;
  scrollbar-width: none !important;     /* Firefox */
  -ms-overflow-style: none !important;  /* IE/Edge */
}

/* Chrome / Edge / Safari */
html::-webkit-scrollbar,
body::-webkit-scrollbar,
*::-webkit-scrollbar {
  display: none !important;
  width: 0 !important;
  height: 0 !important;
  background: transparent !important;
}

/* 强制所有容器不显示滚动条 */
* {
  scrollbar-width: none !important;
  overflow: hidden !important;
}

/* 全局清零 */
* {
  margin: 0 !important;
  padding: 0 !important;
  box-sizing: border-box;
}

/* 分割线 */
.full-line {
  width: 100%;
  height: 1px;
  background: var(--el-border-color-light);
  display: block;
}

/* 顶部工具栏 */
.top-bar {
  width: 100%;
  height: 50px;
  background: #202124;
  display: flex;
  align-items: center;
  padding: 0 10px;
  gap: 6px;
  user-select: none;
}

/* 方形按钮 无圆角 */
.icon-btn {
  width: 36px;
  height: 36px;
  /* 方形：取消圆角 */
  border-radius: 2px;
  border: none;
  background: transparent;
  color: #e8eaed;
  font-size: 18px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s ease;
}

/* 悬浮效果 */
.icon-btn:hover {
  background: #38393c;
}
/* 按下点击效果 */
.icon-btn:active {
  background: #454649;
}

/* 地址栏 */
.url-input {
  flex: 1;
  height: 36px;
  padding: 0 14px;
  border-radius: 2px;
  border: none;
  outline: none;
  background: #303134;
  color: #fff;
  font-size: 14px;
}
.url-input::placeholder {
  color: #9aa0a6;
}

/* 加入收藏、下载 方形按钮 */
.btn {
  width: 36px;
  height: 36px;
  border-radius: 2px;
  border: none;
  background: #1a73e8;
  color: #fff;
  font-size: 16px;
  cursor: pointer;
  transition: background 0.15s ease;
}
.btn:hover {
  background: #2b86ff;
}
.btn:active {
  background: #0f62d9;
}

</style>