import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: HomeView
    },
    {
      path: '/login',
      component: () => import('../views/Login.vue')
    },
    {
      path: '/hard',
      component: () => import('../views/DeviceHard.vue')
    },
    {
      path: '/soft',
      component: () => import('../views/DeviceSoft.vue')
    }
  ]
})

export default router
