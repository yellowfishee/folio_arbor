import { createRouter, createWebHashHistory } from 'vue-router'
import MainLayout from '../layouts/MainLayout.vue'
import LiteratureNoteList from '../views/Literature/index.vue'
import NoteEditor from '../views/NoteEdit/index.vue'
import KnowledgeGraph from '../views/KnowledgeGraph/index.vue'
import Settings from '../views/Setteings/index.vue'
import BookList from '../views/BookList/index.vue'

const routes = [
    {
        path: '/',
        component: MainLayout, // 父布局组件
        children: [
            { path: '', redirect: 'notes' },
            {
              path: '/books',
              component: BookList,
              name: 'BookList',
              meta: { title: '图书列表' }
            },
            {
                path: 'notes',
                component: LiteratureNoteList,
                name: 'NotesList',
                meta: { title: '文献笔记列表' }
            },
            {
                path: 'note/:id',
                component: NoteEditor,
                name: 'NoteEditor',
                meta: { title: '笔记编辑' }
            },
            {
                path: 'graph',
                component: KnowledgeGraph,
                name: 'KnowledgeGraph',
                meta: { title: '知识图谱' }
            },
            {
                path: 'settings',
                component: Settings,
                name: 'Settings',
                meta: { title: '系统设置' }
            }
        ]
    },
    // 添加非布局路由（如登录页)
    // {
    //     path: '/login',
    //     component: () => import('../views/Login.vue'),
    //     name: 'Login',
    //     meta: { requiresAuth: false }
    // }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})

export default router
