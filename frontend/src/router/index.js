import {createRouter,createWebHistory} from "vue-router"
import Dashboard from "../views/Dashboard.vue"
import TestGround from "../views/TestGround.vue"

const routes = [
    {
        path:"/login",
        name:"Login",
        // lazy load the component
        // this will only load the component when the route is visited
        component: () => import("../views/Login.vue"),
        beforeEnter: (to,from) => {
            const token = localStorage.getItem('token')
            if(token){
                router.push("/dashboard")
            }
        }
    },
    {
        path:"/signup",
        name:"Signup",
        // lazy load the component
        component: () => import("../views/Signup.vue"),
        beforeEnter: (to,from) => {
            const token = localStorage.getItem('token')
            if(token){
                router.push("/login")
            }
        }
    },
    {
        path:"/dashboard",
        name:"Dashboard",
        component: Dashboard,
        beforeEnter: (to,from) => {
            const token = localStorage.getItem('token')
            if(!token){
                router.push("/login")
            }
        }
    },
    {
        path:"/recoverpassword",
        name:"RecoverPassword",
        // lazy load the component
        component: () => import("../views/PasswordRecover.vue"),
        beforeEnter: (to,from) => {
            const token = localStorage.getItem('token')
            if(token){
                router.push("/dashboard")
            }
        }
    },
    {
        path:"/testground",
        name:"TestGround",
        component: TestGround,
    }
];

const router = createRouter({
    history:createWebHistory("/"),
    routes
})

export default router;