import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'
import HomeView from './views/Home.vue'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
    meta: {
      requiresAuth: true
    }
  },
  {
    path: '/login',
    name: 'login',
    component: () => import(/* webpackChunkName: "about" */ './views/Login.vue'),
    meta: {
      requiresAuth: false
    }
  },
  {
    path: '/admin-user',
    name: 'admin-users-list',
    component: () => import(/* webpackChunkName: "about" */ './views/AdminUsersIndex.vue'),
    meta: {
      requiresAuth: true
    }
  },
  {
    path: '/admin-user/create',
    name: 'admin-user-create',
    component: () => import(/* webpackChunkName: "about" */ './views/AdminUserCreate.vue'),
    meta: {
      requiresAuth: true
    }
  },
]



const router = createRouter({
  history: createWebHistory(),
  routes
})


router.beforeEach((to, from, next) => {
  if (to.matched.some(record => record.meta.requiresAuth )) {
    const isLoggedIn = localStorage.getItem('is_logged_in')
    if (isLoggedIn != 'true') {
      next({ name: 'login' })
    } else {
      next() // go to wherever I'm going
    }
  } else {
    next() // does not require auth, make sure to always call next()!
  }
})


export default router
