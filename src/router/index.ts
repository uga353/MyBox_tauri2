import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

// 路由规则
const routes: Array<RouteRecordRaw> = [
  { path: '/', component: () => import('../layout/MainLayout.vue'), children: [
    { path: '', name: 'Home', component: () => import('../views/Home.vue')}, 
    { path: 'about', name: 'About', component: () => import('../views/About.vue')},
    
    // 404页面（需放在最后）
    { path: ':pathMatch(.*)*', name: 'NotFound', component: () => import('../views/NotFound.vue')}
   ]
  },
  { path: '/top', name: 'Top', component: () => import('../views/Top.vue')}
];

// 创建路由实例
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL), // HTML5 History 模式
  routes
})

export default router