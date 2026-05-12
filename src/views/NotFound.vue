<template>
  <div class="not-found">
    <div class="content">
      <h1>404</h1>
      <p>页面不存在或已被删除</p>
      <button @click="goBack" class="back-btn">返回首页</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

// 返回首页
const goBack = () => {
  router.push('/')
}
</script>

<style scoped>
.not-found {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f9fafb;
  text-align: center;
}

.content {
  color: #374151;
}

h1 {
  font-size: 80px;
  margin: 0;
  color: #ef4444;
}

p {
  font-size: 18px;
  margin: 10px 0 30px;
}

.back-btn {
  padding: 10px 24px;
  background: #3b82f6;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  transition: background 0.2s;
}

.back-btn:hover {
  background: #2563eb;
}
</style>