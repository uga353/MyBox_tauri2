<template>
  <div class="main-layout">
    <!-- 顶部浏览器工具栏 -->
    <div class="browser-toolbar">
      <!-- 顶部导航工具栏 -->
      <div class="top-bar">
        <button class="icon-btn" title="后退" @click="goBack">🢀</button>
        <button class="icon-btn" title="前进" @click="goForward">🢂</button>
        <button class="icon-btn" title="刷新" @click="refresh">🔄</button>
        <button class="icon-btn" title="首页" @click="goHome">🏠</button>
        
        <!-- 收藏按钮 + 下拉面板 -->
        <div class="favorite-dropdown">
          <button class="icon-btn" title="我的收藏" @click="toggleFavorite">⭐</button>
          <ul v-show="showFavoriteList" id="favoriteList" class="favorite-dropdown-menu">
            <li v-for="(item, index) in favoriteList" :key="index" @click="goToFavorite(item)">
              {{ item.name || item.url }}
            </li>
          </ul>
        </div>

        <input
          v-model="url"
          class="url-input"
          placeholder="输入网址点击地址栏右边➕收藏、回车打开网址！"
          @keydown.enter="goToUrl"
        />
        <button class="btn" title="加入收藏" @click="addFavorite">➕</button>
      </div>
    </div>

    <!-- 顶部菜单栏 -->
    <header class="header">
      <h1>MyBox</h1>
      <nav>
        <router-link to="/">首页</router-link>
        <router-link to="/about">关于</router-link>
      </nav>
    </header>

    <!-- 页面内容（支持正常滚动） -->
    <main class="content">
      <router-view />
    </main>

    <!-- 底部 -->
    <footer class="footer">
      MyBox © {{ year }}
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute() // 获取当前路由

const url = ref('')

// 收藏面板显示
const showFavoriteList = ref(false)

// 收藏列表
const favoriteList = ref<{ url: string; name?: string }[]>([])

// 获取当前年份
const year = computed(() => new Date().getFullYear())

// ==============================================
// 核心：路由变化时，自动同步地址栏显示当前网址
// ==============================================
watch(
  () => route.fullPath,
  (newPath) => {
    if (newPath.startsWith('/')) {
      url.value = '输入网址点击地址栏右边➕收藏、回车打开网址！';
    } else {
      url.value = newPath;
    }
  },
  { immediate: true }
)

// 后退
const goBack = () => {
  window.history.back()
}

// 前进
const goForward = () => {
  window.history.forward()
}

// 刷新
const refresh = () => {
  window.location.reload()
}

// 首页
const goHome = () => {
  router.push('/')
}

// 跳转地址
const goToUrl = () => {
  if (!url.value) return
  window.open(url.value, '_blank')
}

// 收藏面板显示/隐藏
const toggleFavorite = () => {
  showFavoriteList.value = !showFavoriteList.value
}

// 添加收藏
const addFavorite = () => {
  if (!url.value) return
  const hasExist = favoriteList.value.some(item => item.url === url.value)
  if (!hasExist) {
    favoriteList.value.push({ url: url.value })
  }
  showFavoriteList.value = true
}

// 点击收藏项跳转
const goToFavorite = (item: { url: string }) => {
  url.value = item.url
  window.open(item.url, '_blank')
  showFavoriteList.value = false
}

// 点击外部关闭收藏面板
onMounted(() => {
  const clickOutside = (e: MouseEvent) => {
    const el = document.querySelector('.favorite-dropdown')
    if (el && !el.contains(e.target as Node)) {
      showFavoriteList.value = false
    }
  }
  document.addEventListener('click', clickOutside)
  return () => document.removeEventListener('click', clickOutside)
})
</script>

<style scoped>
:root {
  font-size: clamp(14px, 1.5vw, 18px);
}

/* 全局滚动条优化：只隐藏，不破坏滚动能力 */
:global(html),
:global(body) {
  margin: 0;
  padding: 0;
  height: 100%;
  overflow: hidden;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

:global(body::-webkit-scrollbar),
:global(html::-webkit-scrollbar) {
  display: none;
  width: 0;
  height: 0;
}

/* 全局基础样式 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

/* 布局根容器 */
.main-layout {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* 顶部工具栏 */
.top-bar {
  width: 100%;
  height: 7vh;
  background: #202124;
  display: flex;
  align-items: center;
  padding: 0 0;
  gap: 0;
  user-select: none;
}

/* 图标按钮 */
.icon-btn {
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 0.25rem;
  border: none;
  background: transparent;
  color: #e8eaed;
  font-size: 1.2rem;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s ease;
}

.icon-btn:hover {
  background: #38393c;
}

.icon-btn:active {
  background: #454649;
}

/* 地址栏 */
.url-input {
  flex: 1;
  height: 2.5rem;
  padding: 0 1rem;
  border-radius: 0.25rem;
  border: none;
  outline: none;
  background: #303134;
  color: #fff;
  font-size: 1rem;
}

.url-input::placeholder {
  color: #9aa0a6;
}

/* ➕ 按钮 */
.btn {
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 0.25rem;
  border: none;
  background: #3b82f6;
  color: #fff;
  font-size: 1.1rem;
  cursor: pointer;
  transition: background 0.15s ease;
}

.btn:hover {
  background: #2563eb;
}

.btn:active {
  background: #026bfd;
}

/* 收藏下拉面板 */
.favorite-dropdown {
  position: relative;
}

.favorite-dropdown-menu {
  position: absolute;
  top: 100%;
  left: 0;
  margin-top: 4px;
  min-width: 15vw;
  max-height: 35vh;
  background: #292a2d;
  border-radius: 0.25rem;
  list-style: none;
  padding: 4% 0;
  box-shadow: 0 0.25rem 1rem rgba(0, 0, 0, 0.3);
  z-index: 999;
  overflow-y: auto;
}

.favorite-dropdown-menu li {
  padding: 0.5rem 1rem;
  color: #e8eaed;
  cursor: pointer;
  font-size: 1rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.favorite-dropdown-menu li:hover {
  background: #3b3c40;
}

/* 头部导航 */
.header {
  padding: 1rem 0;
  background: #222831;
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header nav {
  display: flex;
  gap: 1rem;
}

.header a {
  color: white;
  text-decoration: none;
}

/* 内容区域（可正常滚动） */
.content {
  flex: 1;
  overflow: auto;
  scrollbar-width: none;
}

.content::-webkit-scrollbar {
  display: none;
}

/* 底部 */
.footer {
  padding: 1vh 2%;
  text-align: center;
  font-size: 0.9rem;
  color: #666;
}
</style>