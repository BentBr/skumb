import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import Routes from './routes'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: Routes.HOME,
            component: HomeView,
        },
        {
            path: '/chat',
            name: Routes.CHAT,
            // route level code-splitting
            // this generates a separate chunk (About.[hash].js) for this route
            // which is lazy-loaded when the route is visited.
            component: () => import('../views/ChatView.vue'),
        },
        {
            path: '/chat/:chat_uuid',
            name: Routes.SHARED_CHAT,
            // route level code-splitting
            // this generates a separate chunk (About.[hash].js) for this route
            // which is lazy-loaded when the route is visited.
            component: () => import('../views/ChatView.vue'),
        },
    ],
})

export default router
