import { createRouter, createWebHashHistory, RouteRecordRaw } from 'vue-router'
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
    path: '/role',
    name: 'role-list',
    component: () => import(/* webpackChunkName: "about" */ './views/role/RoleIndex.vue'),
    meta: {
      requiresAuth: true
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
  {
    path: '/admin-user-edit/:admin_user_id',
    name: 'admin-user-edit',
    component: () => import(/* webpackChunkName: "about" */ './views/AdminUserEdit.vue'),
    meta: {
      requiresAuth: true
    }
  },
  {
    path: '/admin-user-show/:admin_user_id',
    name: 'admin-user-show',
    component: () => import(/* webpackChunkName: "about" */ './views/AdminUserShow.vue'),
    meta: {
      requiresAuth: true
    }
  },
]



const router = createRouter({
  history: createWebHashHistory(),
  routes
})


router.beforeEach((to, _from, next) => {
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
